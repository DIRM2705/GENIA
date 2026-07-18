use super::group::Group;
use crate::data::hypergraph::Hypergraph;
use crate::utils::bitmap::BitmapLen;
use rand::distr::{Distribution, Uniform};
use rand::rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Individual {
    groups: Vec<Group>,
    student_total: usize,
    fitness: f64,
}

impl Individual {
    pub fn new(group_amount: usize, hypergraph: &Hypergraph) -> Self {
        let mut individual = Individual {
            groups: Vec::new(),
            student_total: hypergraph.get_student_count(),
            fitness: 0.0,
        };

        individual.generate_random_groups(group_amount);
        individual.calculate_fitness(hypergraph);
        return individual;
    }

    pub fn get_solution(&self) -> Vec<Vec<usize>> {
        /*
         * Returns the solution of the individual as a vector of groups,
         * where each group is a vector of student indices.
         */

    
        let groups_bitmaps = self
            .groups
            .iter()
            .map(|group| group.get_students())
            .collect::<Vec<BitmapLen>>();
        let solution = groups_bitmaps
            .into_par_iter()
            .map(|group_bitmap| {
                let mut students_in_group = Vec::new();
                for student_idx in 0..self.student_total {
                    if let Ok(true) = group_bitmap.get_bit(student_idx) {
                        students_in_group.push(student_idx);
                    }
                }
                return students_in_group;
            })
            .collect::<Vec<Vec<usize>>>();

        return solution;
    }

    pub fn get_fitness(&self) -> f64 {
        return self.fitness;
    }

    fn generate_random_groups(&mut self, group_amount: usize) {
        let queue = get_random_permutation(self.student_total);
        let max_students_per_group =
            (self.student_total as f64 / group_amount as f64).ceil() as usize;
        let mut i: usize = 0;
        let mut group = Vec::new();

        while i < queue.len() {
            // Group is full
            if group.len() == max_students_per_group {
                let mut group_bitmap = BitmapLen::new(self.student_total as usize);

                // Set the bits of the group from the students assigned to the group
                if let Ok(_) = group_bitmap.set_bits(&group) {
                    self.groups.push(Group::new(group_bitmap)); // Add the group to the individual
                } else {
                    println!(
                        "Error al establecer los bits del grupo: {}",
                        self.groups.len() + 1
                    );
                }
                group = Vec::new();
            }

            // Adds the student to the group
            group.push(queue[i]);
            i += 1;
        }

        let mut group_bitmap = BitmapLen::new(self.student_total as usize);

        // Set the bits of the group from the students assigned to the group
        if let Ok(_) = group_bitmap.set_bits(&group) {
            self.groups.push(Group::new(group_bitmap)); // Add the group to the individual
        } else {
            println!(
                "Error al establecer los bits del grupo: {}",
                self.groups.len() + 1
            );
        }
    }

    pub fn calculate_fitness(&mut self, hypergraph: &Hypergraph) {
        /*
         * Calculates individual's fitness as the MAE of the discardabilities of each group,
         * where the discardability of a group is calculated from the hypergraph.
         * This function is parallelized for each group.
         */

        let fitness_sum: f64 = self
            .groups
            .par_iter()
            .map(|group| 11f64 - group.calculate_discardability(hypergraph))
            .sum();

        self.fitness = fitness_sum/self.groups.len() as f64;
    }

    pub fn crossover(&self, other: &Individual, crossover_rate: u8) -> (Individual, Individual) {
        /*
        * Performs crossover between two individuals, generating two new individuals (children).
        * The crossover is done by exchanging students between the groups of the parents according to a crossover rate.
        * This function is parallelized for each group.
        */

        if crossover_rate < 1 || crossover_rate > 100 {
            println!("La tasa de crossover debe estar entre 1 y 100");
            return (self.clone(), other.clone());
        }

        let results = (0..self.groups.len())
            .into_par_iter()
            .map(|group_idx| {
                let generator = Uniform::new_inclusive(0, 100).unwrap();
                let mut positive_mask = BitmapLen::new(self.student_total);
                let mut negative_mask = positive_mask.clone();

                // Create the new groups from the crossover masks,
                // swapping students between the parents according to the masks
                let new_group1 = (self.groups[group_idx].get_students() & positive_mask.clone())
                    | (other.groups[group_idx].get_students() & negative_mask.clone());
                let new_group2 = (self.groups[group_idx].get_students() & negative_mask.clone())
                    | (other.groups[group_idx].get_students() & positive_mask.clone());

                (Group::new(new_group1), Group::new(new_group2))
            })
            .collect::<Vec<(Group, Group)>>();

        let (child1_groups, child2_groups): (Vec<Group>, Vec<Group>) = results.into_iter().unzip();

        let childs = (
            Individual {
                groups: child1_groups,
                student_total: self.student_total,
                fitness: 0.0,
            },
            Individual {
                groups: child2_groups,
                student_total: self.student_total,
                fitness: 0.0,
            },
        );
        return childs;
    }

    pub fn check_constraints(&mut self, group_amount: usize) {
        /*
        Verify wether the individual meets the constraints:
        - Each student must belong to exactly one group
        - There must be no empty groups
        - All groups must have a number of students within an allowed range
        - All students must be assigned to a group
        If any of the constraints are not met, students are moved arbitrarily to meet them.
        */

        let max_students_per_group =
            (self.student_total as f64 / group_amount as f64).ceil() as usize;

        // Bitmaps de los grupos
        let mut group_bitmaps = self
            .groups
            .iter()
            .map(|group| group.get_students())
            .collect::<Vec<BitmapLen>>();
        
        // Mutex for each bitmap to allow parallel access
        let bitmaps_locks = group_bitmaps
            .iter_mut()
            .map(|bitmap| Mutex::new(bitmap))
            .collect::<Vec<Mutex<&mut BitmapLen>>>();

        let mut freed_students = HashSet::new();
        let freed_mtx = Mutex::new(&mut freed_students);
        
        (0..self.student_total).into_par_iter()
        .for_each(|student_idx|
        {
            // Asumir que no se le asignó a un grupo
            let mut assigned = false;

            // Verify if the student is assigned to more than one group or 
            // if the group has more students than allowed
            for i in 0..bitmaps_locks.len() {
                let mut lock_res = bitmaps_locks[i].lock();

                // If the student is assigned to the group and the group has more students than allowed, 
                // remove the student from the group
                if let Ok(ref mut bitmap) = lock_res && let Ok(true) = bitmap.get_bit(student_idx) {
                    if bitmap.count_ones() as usize > max_students_per_group {

                        bitmap.clear_bit(student_idx).unwrap();
                        if cfg!(debug_assertions) {
                            println!("Grupo {} tiene más estudiantes de los permitidos, eliminando estudiante {}...", i, student_idx);
                            println!("Bitmap del grupo {} después de eliminar: {:?}", i, bitmap);
                        }
                    } 
                    else if assigned {
                        if cfg!(debug_assertions) {
                            println!("Estudiante {} asignado grupo {}, corrigiendo...", student_idx, i);
                        }
                        // The student has already been assigned to a group, remove it from the current group
                        bitmap.clear_bit(student_idx).unwrap();
                    } else {
                        assigned = true; 
                    }
                }
            }

            // The student is not assigned to any group, add it to the freed students set
            if !assigned {
                let lock_res = freed_mtx.lock();
                if lock_res.is_err() {
                    println!("Error al adquirir el lock del mutex de estudiantes liberados");
                    return;
                }
                let mut freed = lock_res.unwrap();
                freed.insert(student_idx);
            }
        });

        // Reconstruct the groups with the corrected bitmaps and 
        // add freed students to groups that have less than the maximum number of students
        self.groups.clear(); 

        for group in group_bitmaps.iter_mut()
        {
            let mut student_count = group.count_ones() as usize;
            while student_count < max_students_per_group && freed_students.len() > 0
            {
                // Get a student from the freed students set and add it to the group
                let index = *freed_students.iter().next().unwrap();
                freed_students.remove(&index);
                group.set_bit(index).unwrap();
                student_count += 1;
            }
            self.groups.push(Group::new(group.clone()));
        }

        if cfg!(debug_assertions) {
            println!("Individuo después de verificar restricciones: {:?}", self.get_solution());
        }

    }

    pub fn mutate(&self, mutation_rate: u8) -> Individual {
        /*
         * Mutates the individual changing students between groups according to a mutation rate.
         * This function is parallelized for each group.
        */

        if mutation_rate < 1 || mutation_rate > 100 {
            println!("La tasa de mutación debe estar entre 1 y 100");
            return self.clone();
        }

        let random_students = get_random_permutation(self.student_total);

        // Divide the number of students to change by 200, since we will swap students between groups
        let changes =
            ((self.student_total as f32) * (mutation_rate as f32 / 200.0)).floor() as usize;

        // Create a new individual as a clone of the current one, to apply the mutations
        let mut new_individual = self.clone();

        //Switch students between groups according to the mutation rate,
        //taking random students from the current group
        for i in (0..changes).step_by(2) {
            for ref mut group in new_individual.groups.iter_mut() {
                if let Ok(true) = group.get_students().get_bit(random_students[i]) {
                    group.get_students().set_bit(random_students[i]).unwrap();
                    group
                        .get_students()
                        .clear_bit(random_students[i + 1])
                        .unwrap();
                } else if let Ok(true) = group.get_students().get_bit(random_students[i + 1]) {
                    group
                        .get_students()
                        .set_bit(random_students[i + 1])
                        .unwrap();
                    group.get_students().clear_bit(random_students[i]).unwrap();
                }
            }
        }
        return new_individual;
    }
}

//Generates a random permutation of the numbers from 0 to n-1
fn get_random_permutation(n: usize) -> Vec<usize> {
    let mut perm: Vec<usize> = (0..n).collect();
    perm.shuffle(&mut rng());
    return perm;
}

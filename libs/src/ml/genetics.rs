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
            //El grupo está lleno
            if group.len() > max_students_per_group {
                let mut group_bitmap = BitmapLen::new(self.student_total as usize);

                //Establece los bits del grupo a partir de los estudiantes asignados al grupo
                if let Ok(_) = group_bitmap.set_bits(&group) {
                    self.groups.push(Group::new(group_bitmap)); //Agrega el grupo al individuo
                } else {
                    println!(
                        "Error al establecer los bits del grupo: {}",
                        self.groups.len() + 1
                    );
                }

                group = Vec::new();
            }

            //Agrega el estudiante al grupo
            group.push(queue[i]);
            i += 1;
        }
    }

    pub fn calculate_fitness(&mut self, hypergraph: &Hypergraph) {
        //Calcula la fitness del individuo como la suma de las descartabilidades de cada grupo,
        //donde la descartabilidad de un grupo se calcula a partir del hipergrafo
        //Esta función es en paralelo para cada grupo
        let fitness_sum : f64 = self
            .groups
            .par_iter()
            .map(|group| group.calculate_discartability(hypergraph))
            .sum();

        //Mayores sumas de fitness indican mayor descartabilidad
        //para maximizar fitness se toma el inverso de la suma de las descartabilidades
        self.fitness = 1.0/(fitness_sum + 1e-10);
    }

    pub fn crossover(&self, other: &Individual, crossover_rate: u8) -> (Individual, Individual) {
        /*
        Crea dos hijos a partir de dos individuos padres,
        intercambiando estudiantes entre grupos según una tasa de crossover
        esta función es en paralelo para cada grupo
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

                //Generar las máscaras de crossover para el grupo actual,
                //indicando qué estudiantes serán intercambiados entre los hijos
                for student_idx in 0..self.student_total {
                    if generator.sample(&mut rand::rng()) <= crossover_rate {
                        positive_mask.set_bit(student_idx).unwrap();
                    } else {
                        negative_mask.set_bit(student_idx).unwrap();
                    }
                }

                //Crea los grupos a partir de las máscaras de crossover,
                //intercambiando los estudiantes entre los padres según las máscaras
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
        Verifica si el individuo cumple con las restricciones:
        - Cada estudiante debe pertenecer a exactamente un grupo
        - No debe haber grupos vacíos
        - Todos los grupos deben tener un número de estudiantes dentro de un rango permitido
        - Todos los estudiantes deben ser asignados a un grupo
        Si alguna de las restricciones no se cumple se mueven estudiantes arbitrariamente para cumplirlas
        */

        let max_students_per_group =
            (self.student_total as f64 / group_amount as f64).ceil() as usize;

        let assigned_students = HashSet::new();
        let freed_students = HashSet::new();
        let assigned_mtx = Mutex::new(assigned_students);
        let freed_mtx = Mutex::new(freed_students);
        let fixed_groups = self.groups.clone();

        let updated_groups = fixed_groups
            .into_par_iter()
            .map(|group| {
                let mut student_count = 0;

                for student_idx in 0..self.student_total {
                    if let Ok(true) = group.get_students().get_bit(student_idx) {
                        //Verificar tamaño del grupo
                        if student_count >= max_students_per_group {
                            //Si el grupo ya tiene el número máximo de estudiantes,
                            //se elimina el estudiante del grupo
                            group.get_students().clear_bit(student_idx).unwrap();
                            let lock_res = freed_mtx.lock();
                            if lock_res.is_err() {
                                println!(
                                    "Error al adquirir el lock del mutex de estudiantes liberados"
                                );
                                continue;
                            }
                            let mut freed = lock_res.unwrap();
                            freed.insert(student_idx); //Agrega el estudiante al conjunto de estudiantes liberados
                            drop(freed); //Libera el lock del mutex de estudiantes liberados
                            continue; //Pasa al siguiente estudiante
                        }

                        //Validar que el estudiante no haya sido asignado a otro grupo
                        let lock_res = assigned_mtx.lock();
                        if lock_res.is_err() {
                            println!(
                                "Error al adquirir el lock del mutex de estudiantes asignados"
                            );
                            return group;
                        }

                        //Verificar si el estudiante ya ha sido asignado a un grupo
                        let mut students = lock_res.unwrap();
                        if students.contains(&student_idx) {
                            //Si el estudiante ya ha sido asignado a un grupo, se elimina del grupo actual
                            let result = group.get_students().clear_bit(student_idx);
                            if result.is_err() {
                                println!("Error al eliminar al estudiante del grupo");
                            }
                        } else {
                            //Si el estudiante no ha sido asignado a un grupo,
                            //se agrega al conjunto de estudiantes asignados
                            students.insert(student_idx);
                            student_count += 1; //Hay un estudiante válido más en el grupo
                        }
                        drop(students); //Libera el lock del mutex de estudiantes asignados
                    }
                }

                //Si el grupo tiene espacio y hay estudiantes liberados, se asignan estudiantes liberados al grupo
                while student_count < max_students_per_group && let Ok(mut freed) = freed_mtx.lock() && freed.len() > 0 {
                    let student_idx = *freed.iter().next().unwrap(); //Toma un estudiante del conjunto de estudiantes liberados
                    group.get_students().set_bit(student_idx).unwrap(); //Agrega el estudiante al grupo
                    freed.remove(&student_idx); //Elimina el estudiante del conjunto de estudiantes liberados
                    student_count += 1; //Hay un estudiante válido más en el grupo
                }

                return group;
            })
            .collect::<Vec<Group>>();

        self.groups = updated_groups;
    }

    pub fn mutate(&self, mutation_rate: u8) -> Individual {
        //Mutar el individuo cambiando la asignación de estudiantes a grupos según una tasa de mutación
        //Esta función es en paralelo para cada grupo

        if mutation_rate < 1 || mutation_rate > 100 {
            println!("La tasa de mutación debe estar entre 1 y 100");
            return self.clone();
        }

        let random_students = get_random_permutation(self.student_total);

        // Divide entre 200 porque solo tomamos la mitad de estudiantes pues, son intercambios por pareja
        let changes =
            ((self.student_total as f32) * (mutation_rate as f32 / 200.0)).floor() as usize;

        // Crea un nuevo individuo a partir del actual
        let mut new_individual = self.clone();

        //Intercambia estudiantes entre grupos según la tasa de mutación,
        //tomando estudiantes aleatorios del grupo actuald
        for i in (0..changes).step_by(2) {
            for group in new_individual.groups.iter_mut() {
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

//Genera una permutación aleatoria de los números del 0 al n-1
fn get_random_permutation(n: usize) -> Vec<usize> {
    let mut perm: Vec<usize> = (0..n).collect();
    perm.shuffle(&mut rng());
    return perm;
}

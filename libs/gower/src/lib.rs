use hypergraph::Student;

const TOTAL_ATTRIBUTES: f64 = 17.0; //Total number of attributes per student
const NUMERICAL_ATTRIBUTES: usize = 10; //Number of numerical attributes
const MI_ATTRIBUTES: usize = 2; //Number of multiple intelligences attributes
const NDD_ATTRIBUTES: usize = 5; //Number of neurodevelopmental disorder attributes

pub fn calculate_gower_distance(s1: &Student, s2: &Student, ranks: &Vec<f64>) -> f64 {
    //Calculate Gower distance between two students
    let mut distance = 0.0;

    //Numerical attributes
    distance += calculate_distances_numerical(&s1, &s2, ranks);

    //Categorical attributes
    distance += calculate_distances_categorical(&s1, &s2);

    return distance/TOTAL_ATTRIBUTES;
}

fn calculate_distances_numerical(s1 : &Student, s2: &Student, ranks: &Vec<f64>) -> f64 {
    //Get gower similarity bewteen two numerical values
    let mut distance = 0.0;

    let mut s1_row = vec![
        s1.be,
        s1.ee,
        s1.ce,
        s1.autonomous_motivation,
        s1.competitive_motivation,
        s1.relationship_motivation,
    ];
    s1_row.extend(s1.vark_scores.iter()); //Add VARK scores

    let mut s2_row = vec![
        s2.be,
        s2.ee,
        s2.ce,
        s2.autonomous_motivation,
        s2.competitive_motivation,
        s2.relationship_motivation,
    ];
    s2_row.extend(s2.vark_scores.iter()); //Add VARK scores

    for k in 0..NUMERICAL_ATTRIBUTES {
        let diff = (s1_row[k] - s2_row[k]).abs();
        distance += 1.0 - diff / ranks[k];
    }
    return distance;
}

fn calculate_distances_categorical(s1 : &Student, s2: &Student) -> f64 {
    //Get gower similarity bewteen two categorical values
    let mut distance = 0.0;
    let s1_row = vec![
        s1.mi_order[0],
        s1.mi_order[1]
    ];
    let s2_row = vec![
        s2.mi_order[0],
        s2.mi_order[1],
    ];

    for k in 0..MI_ATTRIBUTES {
        distance += (s1_row[k] == s2_row[k]) as u32 as f64;
    }

    for k in 0..NDD_ATTRIBUTES
    {
        //Compare ndd bit by bit (8 bits)
        distance += (s1.ndd & s2.ndd & (1 << k)) as f64;
    }
    return distance;
}

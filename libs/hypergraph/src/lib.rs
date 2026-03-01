pub trait Hypergraph
{
    const ID_IDX : usize = 0;
    const AM1_IDX : usize = 3;
    const AM2_IDX : usize = 4;
    const AM3_IDX : usize = 5;
    const AM4_IDX : usize = 6;
    const AM5_IDX : usize = 7;
    const RM1_IDX : usize = 8;
    const RM2_IDX : usize = 9;
    const RM3_IDX : usize = 10;
    const RM4_IDX : usize = 11;
    const RM5_IDX : usize = 12;
    const CM1_IDX : usize = 13;
    const CM2_IDX : usize = 14;
    const CM3_IDX : usize = 15;
    const CM4_IDX : usize = 16;
    const CM5_IDX : usize = 17;
    const BE1_IDX : usize = 18;
    const BE2_IDX : usize = 19;
    const BE3_IDX : usize = 20;
    const BE4_IDX : usize = 21;
    const BE5_IDX : usize = 22;
    const EE1_IDX : usize = 23;
    const EE2_IDX : usize = 24;
    const EE3_IDX : usize = 25;
    const EE4_IDX : usize = 26;
    const EE5_IDX : usize = 27;
    const CE1_IDX : usize = 28;
    const CE2_IDX : usize = 29;
    const CE3_IDX : usize = 30;
    const CE4_IDX : usize = 31;
    const CE5_IDX : usize = 32;
    const VARK_VIS_IDX : usize = 33;
    const VARK_AUR_IDX : usize = 34;
    const VARK_RW_IDX : usize = 35;
    const VARK_KIN_IDX : usize = 36;
    const MI_KIN_IDX : usize = 37;
    const MI_EXIST_IDX : usize = 38;
    const MI_INTER_IDX : usize = 39;
    const MI_INTRA_IDX : usize = 40;
    const MI_LOG_IDX : usize = 41;
    const MI_MUS_IDX : usize = 42;
    const MI_NAT_IDX : usize = 43;
    const MI_VER_IDX : usize = 44;
    const MI_VIS_IDX : usize = 45;

    fn new(students : usize) -> Self;
    fn add_to_hyperedge(&mut self, student_idx : usize, hyperedge_idx : usize);
    fn call_neighborghs_in_he(&self, student_idx : usize, hyperedge_idx : usize);
    fn print(&self);
}

impl Hypergraph for Vec<u64>
{
    fn new(students : usize) -> Vec<u64>
    {
        vec![0; students]
    }

    fn add_to_hyperedge(&mut self, student_idx : usize, hyperedge_idx : usize)
    {
        self[student_idx] |= 1 << hyperedge_idx;
    }

    fn call_neighborghs_in_he(&self, student_idx : usize, hyperedge_idx : usize)
    {
        let mask = 1 << hyperedge_idx;
        for (idx, &he) in self.iter().enumerate() {
            if idx != student_idx && (he & mask) != 0 {
                // This is a neighbor in the same hyperedge
                println!("Student {} is a neighbor of student {} in hyperedge {}", idx, student_idx, hyperedge_idx);
            }
        }
    }

    fn print(&self) {
        let mut mask = 8;
        for index in 3..=45 {
            print!("Hyperedge ");
            match index
            {
                3 => println!("AMotiv1: "),
                4 => println!("AMotiv2: "),
                5 => println!("AMotiv3: "),
                6 => println!("AMotiv4: "),
                7 => println!("AMotiv5: "),
                8 => println!("RMotiv1: "),
                9 => println!("RMotiv2: "),
                10 => println!("RMotiv3: "),
                11 => println!("RMotiv4: "),
                12 => println!("RMotiv5: "),
                13 => println!("CMotiv1: "),
                14 => println!("CMotiv2: "),
                15 => println!("CMotiv3: "),
                16 => println!("CMotiv4: "),
                17 => println!("CMotiv5: "),
                18 => println!("BEngage1: "),
                19 => println!("BEngage2: "),
                20 => println!("BEngage3: "),
                21 => println!("BEngage4: "),
                22 => println!("BEngage5: "),
                23 => println!("EEngage1: "),
                24 => println!("EEngage2: "),
                25 => println!("EEngage3: "),
                26 => println!("EEngage4: "),
                27 => println!("EEngage5: "),
                28 => println!("CEngage1: "),
                29 => println!("CEngage2: "),
                30 => println!("CEngage3: "),
                31 => println!("CEngage4: "),
                32 => println!("CEngage5: "),
                33 => println!("VARK_VIS: "),
                34 => println!("VARK_AUR: "),
                35 => println!("VARK_RW: "),
                36 => println!("VARK_KIN: "),
                37 => println!("MI_KIN: "),
                38 => println!("MI_EXIST: "),
                39 => println!("MI_INTER: "),
                40 => println!("MI_INTRA: "),
                41 => println!("MI_LOG: "),
                42 => println!("MI_MUS: "),
                43 => println!("MI_NAT: "),
                44 => println!("MI_VER: "),
                45 => println!("MI_VIS: "),
                 _=> (),
            }

            for student in 0..self.len() {
                if (self[student] & mask)  != 0{
                    println!("\t - Student {} ", student);
                }
            }
            mask <<= 1;
        }
    }
}
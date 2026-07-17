use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

pub fn log(message: String, file_path: Option<&str>) {
    if let Some(path) = file_path {
        let open_file_result = OpenOptions::new().create(true).append(true).open(path);
        if let Err(e) = open_file_result {
            eprintln!("Error al abrir el archivo de registro: {}", e);
            return;
        }

        let mut file = open_file_result.unwrap();

        let mut writter = BufWriter::new(&mut file);

        let writting_result = writter.write_all((message.clone() + "\n").as_bytes());

        if let Err(e) = writting_result {
            eprintln!("Error al escribir en el archivo de registro: {}", e);
            return;
        }

        if let Err(e) = writter.flush() {
            eprintln!("Error al vaciar el búfer de escritura: {}", e);
        }
    }
    println!("[INFO]: {}", message);
}

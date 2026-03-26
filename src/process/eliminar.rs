use crate::composables::tarea::Tarea;
use crate::process::file_process::guardar_tareas;

pub fn ejecutar(mis_tareas: &mut Vec<Tarea>, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Falta el ID para 'eliminar'.");
        eprintln!("Uso correcto: cargo run eliminar <ID>");
        return;
    }

    let id_str = &args[2];

    let id_buscado: u32 = match id_str.parse() {
        Ok(numero) => numero,
        Err(_) => {
            eprintln!("Error: El ID proporcionado ('{}') no es un número válido.", id_str);
            return;
        }
    };

    let cantidad_original = mis_tareas.len();

    mis_tareas.retain(|t| t.id() != id_buscado);

    if mis_tareas.len() == cantidad_original {
        eprintln!("Error: No se encontró ninguna tarea con el ID {}.", id_buscado);
        return;
    }

    if let Err(error) = guardar_tareas(mis_tareas) {
        eprintln!("Error crítico al intentar guardar en el archivo: {}", error);
    } else {
        println!("¡Tarea con ID {} eliminada con éxito!", id_buscado);
    }
}
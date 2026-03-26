use crate::composables::tarea::Tarea;
use crate::process::file_process::guardar_tareas;

pub fn ejecutar(mis_tareas: &mut Vec<Tarea>, args: &[String]) {
    if args.len() < 4 {
        eprintln!("Error: Faltan argumentos para 'actualizar'.");
        eprintln!("Uso correcto: cargo run actualizar <ID> <in-progress | done>");
        return;
    }

    let id_str = &args[2];
    let nuevo_estado = &args[3];

    if nuevo_estado != "in-progress" && nuevo_estado != "done" && nuevo_estado != "todo" {
        eprintln!("Error: Estado no válido '{}'. Usa 'todo', 'in-progress' o 'done'.", nuevo_estado);
        return;
    }

    let id_buscado: u32 = match id_str.parse() {
        Ok(numero) => numero,
        Err(_) => {
            eprintln!("Error: El ID proporcionado ('{}') no es un número válido.", id_str);
            return;
        }
    };

    let mut tarea_encontrada = false;

    for tarea in mis_tareas.iter_mut() {
        if tarea.id() == id_buscado {
            tarea.cambiar_estado(nuevo_estado.to_string());
            tarea_encontrada = true;
            break;
        }
    }

    if !tarea_encontrada {
        eprintln!("Error: No se encontró ninguna tarea con el ID {}.", id_buscado);
        return;
    }
    if let Err(error) = guardar_tareas(mis_tareas) {
        eprintln!("Error crítico al intentar guardar en el archivo: {}", error);
    } else {
        println!("¡Tarea {} actualizada a estado '{}' con éxito!", id_buscado, nuevo_estado);
    }
}
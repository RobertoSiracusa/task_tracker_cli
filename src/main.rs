mod storage;
mod composables;
mod process;
mod utils;

use std::env;

use process::file_process::inicializar_memoria;
use process::agregar::ejecutar as agregar_tarea;
use process::listar::ejecutar as listar_tareas;
use process::actualizar::ejecutar as actualizar_tarea;
use process::eliminar::ejecutar as eliminar_tarea;

fn main() {

    let mut mis_tareas = inicializar_memoria().unwrap_or_else(|error| {
        eprintln!("Error al cargar las tareas: {}", error);
        std::process::exit(1);
    });

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Debes proporcionar una acción.");
        return;
    }

    let accion = &args[1];
    match accion.as_str() {
        "agregar" => {
            agregar_tarea(&mut mis_tareas, &args);
        }

        "actualizar" => {
            actualizar_tarea(&mut mis_tareas, &args);
        }

        "eliminar" => {
            eliminar_tarea(&mut mis_tareas, &args);
        }

        "listar" => {
            listar_tareas(&mis_tareas, &args);
        }

        _ => {
            eprintln!("Error: Comando no reconocido '{}'", accion);
            eprintln!("Comandos válidos: agregar, actualizar, eliminar, listar");
        }
    }
}
use crate::composables::tarea::Tarea;
use crate::process::file_process::guardar_tareas;

pub fn ejecutar(mis_tareas: &mut Vec<Tarea>, args: &[String], usuario_actual_id: u32) {
    // 1. Validación de argumentos
    if args.len() < 3 {
        eprintln!("\n Error: Falta el ID de la tarea.");
        eprintln!("Uso: cargo run eliminar <ID_TAREA>");
        return;
    }

    let id_str = &args[2];
    let id_buscado: u32 = match id_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("\n Error: '{}' no es un ID válido.", id_str);
            return;
        }
    };

    // 2. Primero verificamos si la tarea existe y quién es el dueño
    // Usamos .iter().position() para encontrar dónde está la tarea
    let posicion = mis_tareas.iter().position(|t| t.get_id() == id_buscado);

    match posicion {
        Some(index) => {
            let tarea = &mis_tareas[index];

            // --- REGLA DE SEGURIDAD ---
            if tarea.get_usuario_id() != usuario_actual_id {
                eprintln!("\nAcceso Denegado");
                eprintln!("No puedes eliminar esta tarea: pertenece a otro usuario.");
                return;
            }

            // Si llegamos aquí, el usuario ES el dueño. Procedemos a eliminar.
            mis_tareas.remove(index);
        }
        None => {
            eprintln!(" Error: No se encontró la tarea con ID {}.", id_buscado);
            return;
        }
    }

    // 3. Guardar los cambios en el archivo JSON
    if let Err(e) = guardar_tareas(mis_tareas) {
        eprintln!("Error al guardar cambios: {}", e);
    } else {
        println!(" Tarea {} eliminada con éxito.", id_buscado);
    }
}
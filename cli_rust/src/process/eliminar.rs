use crate::composables::tarea::Tarea;
use crate::process::file_process::guardar_tareas;

/// Ejecuta el proceso para eliminar una tarea existente.
/// 
/// Esta función realiza los siguientes pasos:
/// 1. Verifica que se haya proporcionado un ID de tarea a eliminar.
/// 2. Convierte el ID ingresado de String a número (u32).
/// 3. Busca la posición de la tarea en la lista.
/// 4. Aplica reglas de seguridad: solo el creador original o el administrador (ID 1) pueden eliminar la tarea.
/// 5. Si la regla se cumple, elimina la tarea del vector.
/// 6. Guarda la nueva lista de tareas actualizada en el disco.
pub fn ejecutar(mis_tareas: &mut Vec<Tarea>, args: &[String], usuario_actual_id: u32) {
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

    let posicion = mis_tareas.iter().position(|t| t.get_id() == id_buscado);

    match posicion {
        Some(index) => {
            let tarea = &mis_tareas[index];

            // --- REGLA DE SEGURIDAD ---
            if tarea.get_usuario_id() != usuario_actual_id && tarea.get_usuario_id() != 1 {
                eprintln!("\nAcceso Denegado");
                eprintln!("No puedes eliminar esta tarea: pertenece a otro usuario.");
                return;
            }

            mis_tareas.remove(index);
        }
        None => {
            eprintln!(" Error: No se encontró la tarea con ID {}.", id_buscado);
            return;
        }
    }

    if let Err(e) = guardar_tareas(mis_tareas) {
        eprintln!("Error al guardar cambios: {}", e);
    } else {
        println!(" Tarea {} eliminada con éxito.", id_buscado);
    }
}
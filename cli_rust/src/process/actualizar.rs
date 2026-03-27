use crate::composables::tarea::Tarea;
use crate::process::file_process::guardar_tareas;

/// Ejecuta el proceso de actualización del estado de una tarea.
/// 
/// Esta función realiza los siguientes pasos:
/// 1. Valida que se hayan proporcionado los argumentos correctos (ID de la tarea y nuevo estado).
/// 2. Busca la tarea correspondiente en la lista de tareas en memoria.
/// 3. Verifica los permisos de seguridad (solo el creador de la tarea o el administrador (ID 1) pueden modificarla).
/// 4. Actualiza el estado de la tarea y guarda los cambios en el sistema de archivos.
pub fn ejecutar(mis_tareas: &mut Vec<Tarea>, args: &[String], usuario_actual_id: u32) {
    // 1 validaciones de argumentos
    if args.len() < 4 {
        eprintln!("\n Error: Faltan argumentos.");
        eprintln!("Uso: cargo run actualizar <ID_TAREA> <nuevo_estado>");
        return;
    }

    let id_tarea_str = &args[2];
    let nuevo_estado = &args[3];

    // 2. Convertir ID de tarea a número
    let id_tarea_buscada: u32 = match id_tarea_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: El ID '{}' no es un número válido.", id_tarea_str);
            return;
        }
    };

    // 3. Buscar la tarea y validar propiedad
    let mut tarea_encontrada = false;

    for tarea in mis_tareas.iter_mut() {
        if tarea.get_id() == id_tarea_buscada {
            tarea_encontrada = true;

            // validacion de usuario
            if tarea.get_usuario_id() != usuario_actual_id && tarea.get_usuario_id()!=1{
                eprintln!("\n Acceso Denegado ");
                eprintln!("No tienes permiso para modificar esta tarea porque no eres el creador ni administrador.");
                return; 
            }

            tarea.cambiar_estado(nuevo_estado.to_string());
            break;
        }
    }

    if !tarea_encontrada {
        eprintln!("Error: No se encontró la tarea con ID {}.", id_tarea_buscada);
        return;
    }

    if let Err(e) = guardar_tareas(mis_tareas) {
        eprintln!("Error al guardar: {}", e);
    } else {
        println!("Tarea {} actualizada a '{}' correctamente.", id_tarea_buscada, nuevo_estado);
    }
}
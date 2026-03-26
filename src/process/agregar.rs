use crate::composables::tarea::Tarea;
use crate::process::file_process::guardar_tareas;

pub fn ejecutar(mis_tareas: &mut Vec<Tarea>, args: &[String],usuario_id: u32) {
    if args.len() < 3 {
        eprintln!("Error: Faltan argumentos para 'agregar'.");
        eprintln!("Uso correcto: cargo run agregar \"Descripción de la tarea\"");
        return;
    }
    
    let descripcion = &args[2];

    let nuevo_id_tarea = mis_tareas.iter().map(|t| t.get_id()).max().unwrap_or(0) + 1;

    let nuevo_id = mis_tareas.iter().map(|t| t.get_id()).max().unwrap_or(0) + 1;

    let nueva_tarea = Tarea::new(nuevo_id_tarea, descripcion.to_string(), usuario_id);

    mis_tareas.push(nueva_tarea);

    if let Err(error) = guardar_tareas(mis_tareas) {
        eprintln!("Error crítico al intentar guardar en el archivo: {}", error);
    } else {
        println!("¡Tarea agregada con éxito! (ID: {})", nuevo_id);
    }
}
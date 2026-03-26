use crate::composables::tarea::Tarea;
use crate::process::file_process::guardar_tareas;

pub fn ejecutar(mis_tareas: &mut Vec<Tarea>, args: &[String]) {
    if args.len() < 3 {
        eprintln!("Error: Faltan argumentos para 'agregar'.");
        eprintln!("Uso correcto: cargo run agregar \"Descripción de la tarea\"");
        return;
    }
    
    let descripcion = &args[2];

    let nuevo_id = mis_tareas.iter().map(|t| t.id()).max().unwrap_or(0) + 1;

    let nueva_tarea = Tarea::new(nuevo_id, descripcion.to_string());

    mis_tareas.push(nueva_tarea);

    if let Err(error) = guardar_tareas(mis_tareas) {
        eprintln!("Error crítico al intentar guardar en el archivo: {}", error);
    } else {
        println!("¡Tarea agregada con éxito! (ID: {})", nuevo_id);
    }
}
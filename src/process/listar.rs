
use crate::composables::tarea::Tarea;
use crate::composables::usuario::Usuario;


pub fn ejecutar(mis_tareas: &[Tarea], args: &[String], usuarios: &[Usuario]) {
    // 1. Validar si el usuario proporcionó el filtro
    // args[0] = programa, args[1] = listar, args[2] = filtro
    if args.len() < 3 {
        println!("\n MODO DE USO: cargo run listar <opción>");
        println!("{}", "-".repeat(45));
        println!("Por favor, elige una categoría para mostrar:");
        println!("  • todas       : Muestra el listado completo");
        println!("  • pendientes  : Solo tareas por empezar");
        println!("  • en-proceso  : Tareas en las que se está trabajando");
        println!("  • completadas : Tareas finalizadas");
        println!("{}", "-".repeat(45));
        return; // Detenemos la ejecución aquí mismo
    }

    // 2. Si llegamos aquí, es porque args[2] existe
    let filtro = args[2].as_str();

    // 3. Aplicar el filtrado según la opción elegida
    let tareas_filtradas: Vec<&Tarea> = match filtro {
        "completadas" => mis_tareas.iter().filter(|t| t.get_estado() == "done").collect(),
        "pendientes"  => mis_tareas.iter().filter(|t| t.get_estado() == "todo").collect(),
        "en-proceso"  => mis_tareas.iter().filter(|t| t.get_estado() == "in-progress").collect(),
        "todas"       => mis_tareas.iter().collect(),
        _ => {
            eprintln!("\n Error: El filtro '{}' no es válido.", filtro);
            println!("Opciones permitidas: todas, pendientes, en-proceso, completadas");
            return;
        }
    };

    if tareas_filtradas.is_empty() {
        println!("\n No se encontraron tareas en la categoría: '{}'", filtro);
        return;
    }

    // 5. Mostrar la tabla de resultados
    println!("\n--- RESULTADOS PARA: {} ---", filtro.to_uppercase());
    println!("{:<4} | {:<30} | {:<12} | {}", "ID", "DESCRIPCIÓN", "ESTADO", "AUTOR");
    println!("{}", "-".repeat(77));

    for tarea in tareas_filtradas {
        // Localizar nombre del autor por ID
        let nombre_autor = usuarios.iter()
            .find(|u| u.get_id() == tarea.get_usuario_id())
            .map(|u| u.get_nombre())
            .unwrap_or("Desconocido");

        let texto_estado = match tarea.get_estado() {
            "done" => "Completada",
            "in-progress" => "En Proceso",
            _ => "Pendiente",
        };

        println!("{:<4} | {:<30} | {:<12} | {}", 
            tarea.get_id(), 
            if tarea.get_descripcion().len() > 30 { format!("{}...", &tarea.get_descripcion()[..27]) } else { tarea.get_descripcion().to_string() }, 
            texto_estado,
            nombre_autor
        );
    }
    println!("{}\n", "-".repeat(77));
}
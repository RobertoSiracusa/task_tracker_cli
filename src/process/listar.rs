
use crate::composables::tarea::Tarea;


pub fn ejecutar(mis_tareas: &[Tarea], args: &[String]) {
    let filtro = if args.len() >= 3 {
        args[2].as_str()
    } else {
        "todas"
    };

    let tareas_filtradas: Vec<&Tarea> = match filtro {
        "completadas" => mis_tareas.iter().filter(|t| t.estado() == "done").collect(),
        "pendientes"  => mis_tareas.iter().filter(|t| t.estado() == "todo").collect(),
        "en-proceso"  => mis_tareas.iter().filter(|t| t.estado() == "in-progress").collect(),
        "todas"       => mis_tareas.iter().collect(),
        _ => {
            eprintln!("Error: Filtro no reconocido '{}'", filtro);
            eprintln!("Filtros válidos: completadas, pendientes, en-proceso, todas");
            return;
        }
    };

    if tareas_filtradas.is_empty() {
        println!("No hay tareas para mostrar en la categoría: {}", filtro);
        return;
    }

    println!("\n--- Lista de Tareas ({}) ---", filtro.to_uppercase());
    for tarea in tareas_filtradas {
        let icono_estado = match tarea.estado() {
            "done" => "[x]",
            "in-progress" => "[-]",
            _ => "[ ]",
        };

        println!("{} ID: {} | {}", icono_estado, tarea.id(), tarea.descripcion());
    }
    println!("--------------------------------\n");
}
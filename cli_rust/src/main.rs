//Declaracion de Modulos
mod composables;
mod process;

use std::env;

use process::file_process::{inicializar_usuarios, inicializar_memoria};
use process::auth::iniciar_sesion;

use process::agregar::ejecutar as agregar_tarea;
use process::listar::ejecutar as listar_tareas;
use process::actualizar::ejecutar as actualizar_tarea;
use process::eliminar::ejecutar as eliminar_tarea;
use process::usuarios_process::ejecutar as gestionar_usuarios;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    // SI NO HAY ARGUMENTOS (solo 'cargo run'), MOSTRAR AYUDA Y SALIR
    if args.len() < 2 {
        mostrar_menu_principal();
        return; 
    }

    // Si llegamos aquí, es porque el usuario sí quiere ejecutar un comando.

    let mut usuarios = inicializar_usuarios().unwrap_or_else(|error| {
        eprintln!("Error al cargar usuarios: {}", error);
        std::process::exit(1);
    });

    let usuario_logueado = match iniciar_sesion(&usuarios) {
        Some(u) => u,
        None => return, 
    };

    let mut mis_tareas = inicializar_memoria().unwrap_or_else(|error| {
        eprintln!("Error al cargar tareas: {}", error);
        std::process::exit(1);
    });

    let accion = &args[1];

    // --- ENRUTADOR DE COMANDOS ---
    match accion.as_str() {
        "gestionar-usuarios" => {
            if usuario_logueado.get_id() == 1 {
                gestionar_usuarios(&mut usuarios, &args);
            } else {
                eprintln!(" Error: Solo el administrador puede gestionar usuarios.");
            }
        }
        "agregar" => {
            agregar_tarea(&mut mis_tareas, &args, usuario_logueado.get_id());
        }
        "listar" => {
            listar_tareas(&mis_tareas, &args, &usuarios);
        }
        "actualizar" => {
            actualizar_tarea(&mut mis_tareas, &args, usuario_logueado.get_id());
        }
        "eliminar" => {
            eliminar_tarea(&mut mis_tareas, &args, usuario_logueado.get_id());
        }
        _ => {
            eprintln!("Error: Comando '{}' no reconocido.", accion);
            mostrar_menu_principal();
        }
    }
}

/// Función auxiliar para no ensuciar el main con prints
fn mostrar_menu_principal() {
    println!("\nBIENVENIDO AL TASK TRACKER CLI");
    println!("{}", "=".repeat(40));
    println!("Uso: cargo run <comando> [argumentos]");
    println!("\nComandos disponibles:");
    println!("  • agregar \"desc\"      : Crea una nueva tarea");
    println!("  • listar <filtro>     : todas, pendientes, en-proceso, completadas");
    println!("  • actualizar <id> <st>: Cambia estado (todo, in-progress, done)");
    println!("  • eliminar <id>       : Borra una tarea");
    println!("  • gestionar-usuarios  : (Solo Admin) Crear/Modificar/Borrar");
    println!("{}", "=".repeat(40));
    println!("Nota: Se te pedirán credenciales al ejecutar cualquier comando.\n");
}
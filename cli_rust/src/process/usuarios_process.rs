// src/process/usuarios.rs

use std::io::{self, Write};
use crate::composables::usuario::Usuario;
use crate::process::file_process::guardar_usuarios;

/// Punto de entrada (enrutador) para todas las operaciones relacionadas con la gestión de usuarios.
/// 
/// Intercepta el subcomando escrito por el usuario en consola (crear, modificar, eliminar, limpiar) 
/// y delega la ejecución a la función interna correspondiente. Muestra ayuda si los argumentos son insuficientes.
pub fn ejecutar(lista_usuarios: &mut Vec<Usuario>, args: &[String]) {
    if args.len() < 3 {
        mostrar_ayuda();
        return;
    }

    let sub_comando = &args[2];

    match sub_comando.as_str() {
        "crear" => crear_usuario(lista_usuarios),
        "modificar" => modificar_usuario(lista_usuarios, args),
        "eliminar" => eliminar_usuario(lista_usuarios, args),
        "limpiar" => limpiar_usuarios(lista_usuarios),
        _ => println!(" Sub-comando de usuario no reconocido."),
    }
}

/// Imprime en la consola el menú de ayuda y los comandos disponibles para la gestión de usuarios.
fn mostrar_ayuda() {
    println!("\n GESTIÓN DE USUARIOS");
    println!("Uso: cargo run gestionar-usuarios <opción>");
    println!("Opciones:");
    println!("  • crear     : Agrega un nuevo usuario");
    println!("  • modificar : Cambia nombre/pass de un ID (excepto 1 y 2)");
    println!("  • eliminar  : Borra un ID específico (excepto 1 y 2)");
    println!("  • limpiar   : Borra TODOS los usuarios (excepto 1 y 2)");
}

// --- FUNCIONES DE LÓGICA ---

/// Inicia el proceso interactivo en terminal para agregar un nuevo usuario.
/// Pide en consola el nombre y contraseña, calcula el ID del usuario autoincrementándolo
/// e ingresa el nuevo `Usuario` en la lista, guardando los cambios luego.
fn crear_usuario(lista: &mut Vec<Usuario>) {
    print!("Nombre del nuevo usuario: ");
    io::stdout().flush().unwrap();
    let mut nombre = String::new();
    io::stdin().read_line(&mut nombre).unwrap();
    
    print!("Contraseña: ");
    io::stdout().flush().unwrap();
    let mut pass = String::new();
    io::stdin().read_line(&mut pass).unwrap();

    let nuevo_id = lista.iter().map(|u| u.get_id()).max().unwrap_or(0) + 1;
    let nuevo_usuario = Usuario::new(nuevo_id, nombre.trim().to_string(), pass.trim().to_string());
    
    lista.push(nuevo_usuario);
    actualizar_archivo(lista, "Usuario creado con éxito.");
}

/// Busca un usuario por su ID provisto en los argumentos de la consola y permite modificar
/// su nombre y su contraseña de forma interactiva. Protege contra la modificación de las cuentas
/// base/por defecto (IDs 1 y 2). Si la entrada de un campo está vacía, no altera ese campo.
fn modificar_usuario(lista: &mut Vec<Usuario>, args: &[String]) {
    if args.len() < 4 {
        println!("Uso: cargo run gestionar-usuarios modificar <ID>");
        return;
    }

    let id: u32 = args[3].parse().unwrap_or(0);
    if id <= 2 {
        println!("Error: No se pueden modificar los usuarios predeterminados (1 y 2).");
        return;
    }

    if let Some(user) = lista.iter_mut().find(|u| u.get_id() == id) {
        print!("Nuevo nombre (dejar vacío para no cambiar): ");
        io::stdout().flush().unwrap();
        let mut nombre = String::new();
        io::stdin().read_line(&mut nombre).unwrap();
        if !nombre.trim().is_empty() { user.set_nombre(nombre.trim().to_string()); }

        print!("Nueva contraseña: ");
        io::stdout().flush().unwrap();
        let mut pass = String::new();
        io::stdin().read_line(&mut pass).unwrap();
        if !pass.trim().is_empty() { user.set_password(pass.trim().to_string()); }

        actualizar_archivo(lista, "Usuario actualizado.");
    } else {
        println!("Usuario no encontrado.");
    }
}

/// Elimina a un usuario especifico por su ID proporcionado en los argumentos, protegiendo las 
/// cuentas default globales (Id 1 y 2). Retiene solo a los usuarios cuyo ID local sea diferente de este.
fn eliminar_usuario(lista: &mut Vec<Usuario>, args: &[String]) {
    if args.len() < 4 {
        println!("Uso: cargo run gestionar-usuarios eliminar <ID>");
        return;
    }

    let id: u32 = args[3].parse().unwrap_or(0);
    if id <= 2 {
        println!("Error: No se pueden eliminar los usuarios predeterminados.");
        return;
    }

    let original_len = lista.len();
    lista.retain(|u| u.get_id() != id);

    if lista.len() < original_len {
        actualizar_archivo(lista, "Usuario eliminado.");
    } else {
        println!("Error: ID no encontrado.");
    }
}

/// Borra todos los usuarios registrados a excepción de las cuentas permanentes (IDs 1 y 2). 
/// Sirve como un reset de los usuarios adicionales del sistema.
fn limpiar_usuarios(lista: &mut Vec<Usuario>) {
    // Retenemos solo los IDs 1 y 2
    lista.retain(|u| u.get_id() <= 2);
    actualizar_archivo(lista, "Se han eliminado todos los usuarios adicionales.");
}

/// Función auxiliar usada en este mismo archivo. Delega la lógica de guardar sobre `file_process`, 
/// proveyendo el manejo de errores global unificado y la retroalimentación de éxito.
fn actualizar_archivo(lista: &Vec<Usuario>, mensaje: &str) {
    if let Err(e) = guardar_usuarios(lista) {
        eprintln!("Error al guardar usuarios: {}", e);
    } else {
        println!("Procesado: {}", mensaje);
    }
}
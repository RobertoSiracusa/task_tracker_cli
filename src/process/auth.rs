use std::io::{self, Write};
use crate::composables::usuario::Usuario;

pub fn iniciar_sesion(usuarios: &[Usuario]) -> Option<Usuario> {
    println!("\n=== INICIO DE SESIÓN ===");

    print!("Usuario: ");
    io::stdout().flush().unwrap(); // Obliga a mostrar el texto antes de esperar el input
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Error al leer el usuario");
    let username = username.trim(); // Quitamos el salto de línea (\n)

    // 2. Pedir la contraseña
    print!("Contraseña: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Error al leer la contraseña");
    let password = password.trim();

    // 3. Validar contra la "base de datos" (el Vec)
    for usuario in usuarios {
        if usuario.get_nombre() == username && usuario.get_password() == password {
            println!("\n¡Bienvenido de nuevo, {}!", username);
            return Some(usuario.clone()); // Retornamos una copia del usuario logueado
        }
    }

    println!("\nError: Usuario o contraseña incorrectos.");
    None
}
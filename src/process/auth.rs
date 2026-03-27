use std::io::{self, Write};
use crate::composables::usuario::Usuario;

/// Maneja el proceso interctivo de inicio de sesión por terminal.
/// 
/// Esta función realiza los siguientes pasos:
/// 1. Solicita al usuario que ingrese su nombre de usuario a través de la entrada estándar.
/// 2. Solicita al usuario que ingrese su contraseña.
/// 3. Recorre la lista (slice) de usuarios registrada y compara las credenciales ingresadas.
/// 4. Si hay una coincidencia exacta de nombre y contraseña, retorna `Some(Usuario)` replicando sus datos.
/// 5. Si no hay coincidencias, imprime un mensaje de error y retorna `None` indicando que el login falló.
pub fn iniciar_sesion(usuarios: &[Usuario]) -> Option<Usuario> {
    println!("\n=== INICIO DE SESIÓN ===");

    print!("Usuario: ");
    io::stdout().flush().unwrap(); //lecutura de teclado
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Error al leer el usuario");
    let username = username.trim(); 

    print!("Contraseña: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Error al leer la contraseña");
    let password = password.trim();

    for usuario in usuarios {
        if usuario.get_nombre() == username && usuario.get_password() == password {
            println!("\n¡Bienvenido de nuevo, {}!", username);
            return Some(usuario.clone()); 
        }
    }

    println!("\nError: Usuario o contraseña incorrectos.");
    None
}
// src/process/file_process.rs

use std::fs;
use std::path::Path;
use crate::composables::tarea::Tarea; 
use crate::composables::usuario::Usuario;

const CARPETA_STORAGE: &str = "src/storage";
const USUARIOS_JSON: &str = "src/storage/usuarios.json";
const TAREAS_JSON: &str = "src/storage/tareas.json";


pub fn inicializar_usuarios() -> Result<Vec<Usuario>, Box<dyn std::error::Error>> {
    if !Path::new(CARPETA_STORAGE).exists() {
        fs::create_dir_all(CARPETA_STORAGE)?;
    }

    if !Path::new(USUARIOS_JSON).exists() {
        let usuarios_defecto = vec![
            Usuario::new(1, "admin".to_string(), "123".to_string()),
            Usuario::new(2, "invitado".to_string(), "456".to_string()),
        ];
        let json = serde_json::to_string_pretty(&usuarios_defecto)?;
        fs::write(USUARIOS_JSON, json)?;
        return Ok(usuarios_defecto);
    }

    let contenido = fs::read_to_string(USUARIOS_JSON)?;
    let usuarios: Vec<Usuario> = serde_json::from_str(&contenido)?;
    Ok(usuarios)
}

pub fn inicializar_memoria() -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    let ruta = Path::new(TAREAS_JSON);

    if !ruta.exists() {
        let tareas_vacias: Vec<Tarea> = Vec::new();
        let json = serde_json::to_string_pretty(&tareas_vacias)?;
        fs::write(TAREAS_JSON, json)?;
        return Ok(tareas_vacias);
    }

    let contenido = fs::read_to_string(TAREAS_JSON)?;
    let tareas: Vec<Tarea> = serde_json::from_str(&contenido)?;
    Ok(tareas)
}

/// Guarda la lista completa de tareas en el archivo compartido.
pub fn guardar_tareas(tareas: &Vec<Tarea>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(tareas)?;
    fs::write(TAREAS_JSON, json)?;
    Ok(())
}

pub fn guardar_usuarios(usuarios: &Vec<Usuario>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(usuarios)?;
    fs::write(USUARIOS_JSON, json)?;
    Ok(())
}
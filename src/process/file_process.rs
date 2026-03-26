// src/process/file_process.rs

use std::fs;
use std::path::Path;
use crate::composables::tarea::Tarea; 

const ARCHIVO_JSON: &str = "src/storage/tareas.json";

pub fn inicializar_memoria() -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    let ruta_archivo = Path::new(ARCHIVO_JSON);

    if let Some(carpeta_padre) = ruta_archivo.parent() {
        if !carpeta_padre.exists() {
            fs::create_dir_all(carpeta_padre)?;
            println!("Carpeta 'src/storage' creada exitosamente.");
        }
    }

    if !ruta_archivo.exists() {
        println!("No se encontró el archivo JSON. Creando uno nuevo en '{}'...", ARCHIVO_JSON);
        
        let tareas_vacias: Vec<Tarea> = Vec::new();
        
        let json_vacio = serde_json::to_string_pretty(&tareas_vacias)?;
        
        fs::write(ARCHIVO_JSON, json_vacio)?;
        
        return Ok(tareas_vacias);
    }

    let contenido = fs::read_to_string(ARCHIVO_JSON)?;
    let tareas: Vec<Tarea> = serde_json::from_str(&contenido)?;
    
    Ok(tareas)
}

pub fn guardar_tareas(tareas: &Vec<Tarea>) -> Result<(), Box<dyn std::error::Error>> {
    let ruta_archivo = Path::new(ARCHIVO_JSON);

    if let Some(carpeta_padre) = ruta_archivo.parent() {
        if !carpeta_padre.exists() {
            fs::create_dir_all(carpeta_padre)?;
        }
    }

    let json = serde_json::to_string_pretty(tareas)?;
    
    fs::write(ARCHIVO_JSON, json)?;
    
    Ok(())
}
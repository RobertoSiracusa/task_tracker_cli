use serde::{Serialize, Deserialize};
use chrono::Local;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tarea {
    id: u32,
    descripcion: String,
    estado: String,
    usuario_id: u32,
    creado_en: String,
    actualizado_en: String,
}

impl Tarea {
    pub fn new(id: u32, descripcion: String, usuario_id: u32) -> Self {
        let tiempo_actual = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        Tarea {
            id,
            descripcion,
            estado: String::from("todo"),
            usuario_id,
            creado_en: tiempo_actual.clone(),
            actualizado_en: tiempo_actual,
        }
    }

    // ==========
    // GETTERS 
    // ===================
    
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_descripcion(&self) -> &str {
        &self.descripcion
    }

    pub fn get_estado(&self) -> &str {
        &self.estado
    }

    pub fn get_usuario_id(&self) -> u32 {
        self.usuario_id
    }

    // ===================
    // SETTERS 
    // ========

    pub fn cambiar_estado(&mut self, nuevo_estado: String) {
        self.estado = nuevo_estado;
        self.actualizado_en = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }

}
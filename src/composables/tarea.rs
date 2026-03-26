use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tarea {
    id: u32,
    descripcion: String,
    estado: String,
    creado_en: u64,
    actualizado_en: u64,
}

impl Tarea {
    pub fn new(id: u32, descripcion: String) -> Self {
        let tiempo_actual = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Tarea {
            id,
            descripcion,
            estado: String::from("todo"),
            creado_en: tiempo_actual,
            actualizado_en: tiempo_actual,
        }
    }

    // ==========
    // GETTERS 
    // ===================
    
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn descripcion(&self) -> &str {
        &self.descripcion
    }

    pub fn estado(&self) -> &str {
        &self.estado
    }

    // ===================
    // SETTERS 
    // ========

    pub fn cambiar_estado(&mut self, nuevo_estado: String) {
        self.estado = nuevo_estado;
        self.actualizado_en = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}
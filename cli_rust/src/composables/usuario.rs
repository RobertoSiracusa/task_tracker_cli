use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]


pub struct Usuario{
    id: u32,
    nombre: String,
    password: String,
}

impl Usuario {
    pub fn new(id:u32, nombre:String, password:String)
     ->Self{
        Usuario { id, nombre, password }
    
    }
    //====
    //GETTERS
    //=====
    pub fn get_nombre(&self) -> &str {
        &self.nombre
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    //====
    //SETTERS
    //====
    pub fn set_nombre(&mut self, nuevo_nombre: String) {
        self.nombre = nuevo_nombre;
    }
    pub fn set_password(&mut self, nueva_password: String) {
        self.password = nueva_password;
    }
}


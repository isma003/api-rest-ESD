use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Medico {
    pub id: u32,
    pub nombre: String,
    pub especialidad: String,
    pub junta_vigilancia: String, // Requisito común en El Salvador
    pub disponible: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateMedicoDto {
    pub nombre: String,
    pub space_especialidad: String,
    pub junta_vigilancia: String,
    pub disponible: bool,
}
use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Medico {
    pub id_medico: i32,
    pub nombre: String,
    pub id_especialidad: Option<i32>,
    pub numero_licencia: Option<String>,
    pub telefono: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateMedico {
    pub nombre: String,
    pub id_especialidad: Option<i32>,
    pub numero_licencia: Option<String>,
    pub telefono: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateMedico {
    pub nombre: Option<String>,
    pub id_especialidad: Option<i32>,
    pub numero_licencia: Option<String>,
    pub telefono: Option<String>,
}
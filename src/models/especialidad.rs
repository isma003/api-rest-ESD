use sqlx::FromRow;
use serde::{ Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Especialidad {
    pub id_especialidad: i32,
    pub nombre_especialidad: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEspecialidad {
    pub nombre_especialidad: String,
    pub descripcion: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEspecialidad {
    pub nombre_especialidad: Option<String>,
    pub descripcion: Option<String>,
}
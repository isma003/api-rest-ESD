use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Paciente {
    pub id_paciente: i32,
    pub nombre: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub direccion: Option<String>,
    pub tipo_sangre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaciente {
    pub nombre: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub direccion: Option<String>,
    pub tipo_sangre: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePaciente {
    pub nombre: Option<String>,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub direccion: Option<String>,
    pub tipo_sangre: Option<String>,
}
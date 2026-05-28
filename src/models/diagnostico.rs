use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Diagnostico {
    pub id_diagnostico: i32,
    pub id_cita: Option<i32>,
    pub descripcion_diagnostico: Option<String>,
    pub tratamiento_sugerido: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateDiagnostico {
    pub id_cita: i32,
    pub descripcion_diagnostico: Option<String>,
    pub tratamiento_sugerido: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateDiagnostico {
    pub descripcion_diagnostico: Option<String>,
    pub tratamiento_sugerido: Option<String>,
}
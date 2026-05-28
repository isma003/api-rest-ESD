use chrono::{NaiveDate, NaiveTime};
use sqlx::FromRow;
use serde::{ Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Cita{
    pub id_cita: i32,
    pub id_paciente: Option<i32>,
    pub id_medico: Option<i32>,
    pub fecha_cita: NaiveDate,
    pub hora_cita: NaiveTime,
    pub motivo_consulta: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateCita {
    pub id_paciente: i32,
    pub id_medico: i32,
    pub fecha_cita: NaiveDate,
    pub hora_cita: NaiveTime,
    pub motivo_consulta: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateCita {
    pub id_paciente: Option<i32>,
    pub id_medico: Option<i32>,
    pub fecha_cita: NaiveDate,
    pub hora_cita: NaiveTime,
    pub motivo_consulta: Option<String>,
}

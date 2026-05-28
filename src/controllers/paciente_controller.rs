use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::models::paciente::{CreatePaciente, Paciente, UpdatePaciente};
use crate::services::paciente_service::PacienteService;
use crate::error::AppError;
use crate::utils::PrettyJson;

pub async fn listar_pacientes(
    State(service): State<PacienteService>,
) -> Result<PrettyJson<Vec<Paciente>>, AppError> {
    let pacientes = service.listar_pacientes().await?;
    Ok(PrettyJson(pacientes))
}

pub async fn obtener_paciente(
    State(service): State<PacienteService>,
    Path(id): Path<i32>,
) -> Result<PrettyJson<Paciente>, AppError> {
    let paciente = service.obtener_paciente(id).await?;
    Ok(PrettyJson(paciente))
}

pub async fn crear_paciente(
    State(service): State<PacienteService>,
    Json(data): Json<CreatePaciente>,
) -> Result<(StatusCode, PrettyJson<Paciente>), AppError> {
    let nuevo = service.crear_paciente(data).await?;
    Ok((StatusCode::CREATED, PrettyJson(nuevo)))
}

pub async fn actualizar_paciente(
    State(service): State<PacienteService>,
    Path(id): Path<i32>,
    Json(data): Json<UpdatePaciente>,
) -> Result<PrettyJson<Paciente>, AppError> {
    let actualizado = service.actualizar_paciente(id, data).await?;
    Ok(PrettyJson(actualizado))
}

pub async fn eliminar_paciente(
    State(service): State<PacienteService>,
    Path(id): Path<i32>,
) -> Result<PrettyJson<serde_json::Value>, AppError> {
    service.eliminar_paciente(id).await?;
    let mensaje = json!({ "message": "Paciente eliminado correctamente" });
    Ok(PrettyJson(mensaje))
}
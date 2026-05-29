use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::error::AppError;
use crate::models::medico::{CreateMedico, Medico, UpdateMedico};
use crate::services::medico_service::MedicoService;
use crate::utils::PrettyJson;

pub async fn obtener_todos(
    State(service): State<MedicoService>,
) -> Result<PrettyJson<Vec<Medico>>, AppError> {
    let medicos = service.listar_medicos().await?;
    Ok(PrettyJson(medicos))
}

pub async fn obtener_por_id(
    State(service): State<MedicoService>,
    Path(id): Path<i32>,
) -> Result<PrettyJson<Medico>, AppError> {
    let medico = service.obtener_por_id(id).await?;
    Ok(PrettyJson(medico))
}

pub async fn crear_medico(
    State(service): State<MedicoService>,
    Json(payload): Json<CreateMedico>,
) -> Result<(StatusCode, PrettyJson<Medico>), AppError> {
    let nuevo = service.crear_medico(payload).await?;
    Ok((StatusCode::CREATED, PrettyJson(nuevo)))
}

pub async fn actualizar_medico(
    State(service): State<MedicoService>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMedico>,
) -> Result<PrettyJson<Medico>, AppError> {
    let actualizado = service.actualizar_medico(id, payload).await?;
    Ok(PrettyJson(actualizado))
}

pub async fn eliminar_medico(
    State(service): State<MedicoService>,
    Path(id): Path<i32>,
) -> Result<PrettyJson<serde_json::Value>, AppError> {
    service.eliminar_medico(id).await?;
    let mensaje = json!({ "message": "Médico eliminado correctamente" });
    Ok(PrettyJson(mensaje))
}
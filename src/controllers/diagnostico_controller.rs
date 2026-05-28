use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::error::AppError;
use crate::models::diagnostico::{Diagnostico, CreateDiagnostico, UpdateDiagnostico};
use crate::services::diagnostico_service::DiagnosticoService;
use crate::utils::PrettyJson;

pub async fn obtener_todos(
    State(service): State<DiagnosticoService>,
) -> Result<PrettyJson<Vec<Diagnostico>>, AppError> {
    let diagnosticos = service.listar_diagnosticos().await?;
    Ok(PrettyJson(diagnosticos))
}

pub async fn obtener_por_id(
    Path(id): Path<i32>,
    State(service): State<DiagnosticoService>,
) -> Result<PrettyJson<Diagnostico>, AppError> {
    let diagnostico = service.obtener_por_id(id).await?;
    Ok(PrettyJson(diagnostico))
}

pub async fn crear_diagnostico(
    State(service): State<DiagnosticoService>,
    Json(payload): Json<CreateDiagnostico>,
) -> Result<(StatusCode, PrettyJson<Diagnostico>), AppError> {
    let nuevo = service.crear_diagnostico(payload).await?;
    Ok((StatusCode::CREATED, PrettyJson(nuevo)))
}

pub async fn actualizar_diagnostico(
    Path(id): Path<i32>,
    State(service): State<DiagnosticoService>,
    Json(payload): Json<UpdateDiagnostico>,
) -> Result<PrettyJson<Diagnostico>, AppError> {
    let actualizado = service.actualizar_diagnostico(id, payload).await?;
    Ok(PrettyJson(actualizado))
}

pub async fn eliminar_diagnostico(
    Path(id): Path<i32>,
    State(service): State<DiagnosticoService>,
) -> Result<PrettyJson<serde_json::Value>, AppError> {
    service.eliminar_diagnostico(id).await?;
    let mensaje = serde_json::json!({ "message": "Diagnóstico eliminado correctamente" });
    Ok(PrettyJson(mensaje))
}
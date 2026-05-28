use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::error::AppError;
use crate::models::cita::{Cita, CreateCita, UpdateCita};
use crate::services::cita_service::CitaService;
use crate::utils::PrettyJson;

pub async fn obtener_todas(
    State(service): State<CitaService>,
) -> Result<PrettyJson<Vec<Cita>>, AppError> {
    
    let citas = service.listar_citas().await?;
    Ok(PrettyJson(citas))
}

pub async fn obtener_por_id(
    Path(id): Path<i32>,
    State(service): State<CitaService>,
) -> Result<PrettyJson<Cita>, AppError> {
    let cita = service.obtener_por_id(id).await?;
    Ok(PrettyJson(cita))
}

pub async fn crear_cita(
    State(service): State<CitaService>,
    Json(payload): Json<CreateCita>,
) -> Result<(StatusCode, PrettyJson<Cita>), AppError> {
    let nueva_cita = service.crear_cita(payload).await?;
    Ok((StatusCode::CREATED, PrettyJson(nueva_cita)))
}

pub async fn actualizar_cita(
    Path(id): Path<i32>,
    State(service): State<CitaService>,
    Json(payload): Json<UpdateCita>,
) -> Result<PrettyJson<Cita>, AppError> {
    let actualizada = service.actualizar_cita(id, payload).await?;
    Ok(PrettyJson(actualizada))
}

pub async fn eliminar_cita(
    Path(id): Path<i32>,
    State(service): State<CitaService>,
) -> Result<PrettyJson<serde_json::Value>, AppError> {
    service.eliminar_cita(id).await?;
    let mensaje = serde_json::json!({ "message": "Cita eliminada correctamente" });
    Ok(PrettyJson(mensaje))
}
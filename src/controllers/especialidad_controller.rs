use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::models::especialidad::{CreateEspecialidad, Especialidad, UpdateEspecialidad};
use crate::services::especialidad_service::EspecialidadService;
use crate::error::AppError;
use crate::utils::PrettyJson;

pub async fn listar_especialidades(
    State(service): State<EspecialidadService>,
) -> Result<PrettyJson<Vec<Especialidad>>, AppError> {
    let especialidades = service.listar_especialidades().await?;
    Ok(PrettyJson(especialidades))
}

pub async fn obtener_especialidad(
    State(service): State<EspecialidadService>,
    Path(id): Path<i32>,
) -> Result<PrettyJson<Especialidad>, AppError> {
    let especialidad = service.obtener_especialidad(id).await?;
    Ok(PrettyJson(especialidad))
}

pub async fn crear_especialidad(
    State(service): State<EspecialidadService>,
    Json(data): Json<CreateEspecialidad>,
) -> Result<(StatusCode, PrettyJson<Especialidad>), AppError> {
    let nueva = service.crear_especialidad(data).await?;
    Ok((StatusCode::CREATED, PrettyJson(nueva)))
}

pub async fn actualizar_especialidad(
    State(service): State<EspecialidadService>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateEspecialidad>,
) -> Result<PrettyJson<Especialidad>, AppError> {
    let actualizada = service.actualizar_especialidad(id, data).await?;
    Ok(PrettyJson(actualizada))
}

pub async fn eliminar_especialidad(
    State(service): State<EspecialidadService>,
    Path(id): Path<i32>,
) -> Result<PrettyJson<serde_json::Value>, AppError> {
    service.eliminar_especialidad(id).await?;
    let mensaje = json!({ "message": "Especialidad eliminada correctamente" });
    Ok(PrettyJson(mensaje))
}
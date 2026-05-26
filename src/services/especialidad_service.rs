use crate::error::AppError;
use crate::models::especialidad::{CreateEspecialidad, Especialidad, UpdateEspecialidad};
use crate::repositories::especialidad_repo::EspecialidadRepo;

#[derive(Clone)]
pub struct EspecialidadService {
    repo: EspecialidadRepo,
}

impl EspecialidadService {
    pub fn new(repo: EspecialidadRepo) -> Self {
        Self { repo }
    }

    pub async fn listar_especialidades(&self) -> Result<Vec<Especialidad>, AppError> {
        self.repo.find_all().await
    }

    pub async fn obtener_especialidad(&self, id: i32) -> Result<Especialidad, AppError> {
        self.repo.find_by_id(id).await
    }

    pub async fn crear_especialidad(
        &self,
        data: CreateEspecialidad,
    ) -> Result<Especialidad, AppError> {
        if data.nombre_especialidad.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El nombre de la especialidad es obligatorio".into(),
            ));
        }
        self.repo.create(data).await
    }

    pub async fn actualizar_especialidad(
        &self,
        id: i32,
        data: UpdateEspecialidad,
    ) -> Result<Especialidad, AppError> {
        if data.nombre_especialidad.is_none() && data.descripcion.is_none() {
            return Err(AppError::BadRequest(
                "Se debe enviar al menos un campo para actualizar".into(),
            ));
        }
        self.repo.update(id, data).await
    }

    pub async fn eliminar_especialidad(&self, id: i32) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
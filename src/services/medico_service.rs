use crate::error::AppError;
use crate::models::medico::{CreateMedico, Medico, UpdateMedico};
use crate::repositories::medico_repo::MedicoRepository;

#[derive(Clone)]
pub struct MedicoService {
    repo: MedicoRepository,
}

impl MedicoService {
    pub fn new(repo: MedicoRepository) -> Self {
        Self { repo }
    }

    pub async fn listar_medicos(&self) -> Result<Vec<Medico>, AppError> {
        self.repo.find_all().await
    }

    pub async fn obtener_por_id(&self, id: i32) -> Result<Medico, AppError> {
        self.repo.find_by_id(id).await
    }

    pub async fn crear_medico(&self, data: CreateMedico) -> Result<Medico, AppError> {
        if data.nombre.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El nombre del médico es obligatorio".into(),
            ));
        }
        self.repo.create(data).await
    }

    pub async fn actualizar_medico(
        &self,
        id: i32,
        data: UpdateMedico,
    ) -> Result<Medico, AppError> {
        if data.nombre.is_none() && data.id_especialidad.is_none() && data.numero_licencia.is_none() && data.telefono.is_none() {
            return Err(AppError::BadRequest(
                "Se debe enviar al menos un campo para actualizar".into(),
            ));
        }
        self.repo.update(id, data).await
    }

    pub async fn eliminar_medico(&self, id: i32) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
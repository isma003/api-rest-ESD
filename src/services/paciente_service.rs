use crate::error::AppError;
use crate::models::paciente::{CreatePaciente, Paciente, UpdatePaciente};
use crate::repositories::paciente_repo::PacienteRepo;

#[derive(Clone)]
pub struct PacienteService {
    repo: PacienteRepo,
}

impl PacienteService {
    pub fn new(repo: PacienteRepo) -> Self {
        Self { repo }
    }

    pub async fn listar_pacientes(&self) -> Result<Vec<Paciente>, AppError> {
        self.repo.find_all().await
    }

    pub async fn obtener_paciente(&self, id: i32) -> Result<Paciente, AppError> {
        self.repo.find_by_id(id).await
    }

    pub async fn crear_paciente(&self, data: CreatePaciente) -> Result<Paciente, AppError> {
        if data.nombre.trim().is_empty() {
            return Err(AppError::BadRequest(
                "El nombre del paciente es obligatorio".into(),
            ));
        }

        self.repo.create(data).await
    }

    pub async fn actualizar_paciente(
        &self,
        id: i32,
        data: UpdatePaciente,
    ) -> Result<Paciente, AppError> {
        // Validar que manden al menos un campo a actualizar
        if data.nombre.is_none()
            && data.fecha_nacimiento.is_none()
            && data.direccion.is_none()
            && data.tipo_sangre.is_none()
        {
            return Err(AppError::BadRequest(
                "Se debe enviar al menos un campo para actualizar".into(),
            ));
        }
        self.repo.update(id, data).await
    }

    pub async fn eliminar_paciente(&self, id: i32) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
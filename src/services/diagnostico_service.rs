use crate::error::AppError;
use crate::models::diagnostico::{Diagnostico, CreateDiagnostico, UpdateDiagnostico};
use crate::repositories::diagnostico_repo::DiagnosticoRepository;

#[derive(Clone)]
pub struct DiagnosticoService {
    repo: DiagnosticoRepository,
}

impl DiagnosticoService {
    pub fn new(repo: DiagnosticoRepository) -> Self {
        Self { repo }
    }

    pub async fn listar_diagnosticos(&self) -> Result<Vec<Diagnostico>, AppError> {
        self.repo.find_all().await
    }

    pub async fn obtener_por_id(&self, id_diagnostico: i32) -> Result<Diagnostico, AppError> {
        self.repo.find_by_id(id_diagnostico).await
    }

    pub async fn crear_diagnostico(&self, nuevo: CreateDiagnostico) -> Result<Diagnostico, AppError> {
        // Validamos solo si el cliente envió una descripción (si es Some)
        if let Some(desc) = &nuevo.descripcion_diagnostico {
            if desc.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "La descripción del diagnóstico no puede estar vacía".into()
                ));
            }
        }

        self.repo.create_diagnostico(nuevo).await
    }

    pub async fn actualizar_diagnostico(
        &self,
        id_diagnostico: i32,
        actualizado: UpdateDiagnostico
    ) -> Result<Diagnostico, AppError> {
        // Misma validación segura para el update
        if let Some(desc) = &actualizado.descripcion_diagnostico {
            if desc.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "La descripción del diagnóstico no puede estar vacía".into()
                ));
            }
        }

        self.repo.update(id_diagnostico, actualizado).await
    }

    pub async fn eliminar_diagnostico(&self, id_diagnostico: i32) -> Result<(), AppError> {
        self.repo.delete(id_diagnostico).await
    }
}
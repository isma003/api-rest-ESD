use crate::error::AppError;
use crate::models::cita::{Cita, CreateCita, UpdateCita};
use crate::repositories::cita_repo::CitaRepository;

#[derive(Clone)]
pub struct CitaService {
    repo: CitaRepository,
}

impl CitaService {
    
    pub fn new(repo: CitaRepository) -> Self {
        Self { repo }
    }

    pub async fn listar_citas(&self) -> Result<Vec<Cita>, AppError> {
        self.repo.find_all().await
    }

    pub async fn obtener_por_id(&self, id_cita: i32) -> Result<Cita, AppError> {
        self.repo.find_by_id(id_cita).await
    }

    pub async fn crear_cita(&self,
         nueva_cita: CreateCita) 
         -> Result<Cita, AppError> {
        
        if let Some(motivo) = &nueva_cita.motivo_consulta {
            if motivo.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El motivo de consulta no puede estar vacío".into()
                ));
            } 
        }
        self.repo.create_cita(nueva_cita).await
    }

    pub async fn actualizar_cita(
        &self, id_cita: i32,
         cita_actualizada: UpdateCita)
         -> Result<Cita, AppError> {
        if let Some(motivo) = &cita_actualizada.motivo_consulta {
            if motivo.trim().is_empty() {
                return Err(AppError::BadRequest(
                    "El motivo de consulta no puede estar vacío".into()
                ));
            }
        }
        self.repo.update(id_cita, cita_actualizada).await
    }

    pub async fn eliminar_cita(&self, id_cita: i32) -> Result<(), AppError> {
        self.repo.delete(id_cita).await

    }
}
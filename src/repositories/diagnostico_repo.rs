use crate::{error::AppError, models::diagnostico::{Diagnostico, CreateDiagnostico, UpdateDiagnostico}};
use sqlx::PgPool;

#[derive(Clone)]
pub struct DiagnosticoRepository {
    pool: PgPool,
}

impl DiagnosticoRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn find_all(&self) -> Result<Vec<Diagnostico>, AppError> {
        let diagnosticos = sqlx::query_as!(
            Diagnostico,
            r#"SELECT id_diagnostico, id_cita, descripcion_diagnostico, tratamiento_sugerido
               FROM Diagnosticos ORDER BY id_diagnostico ASC"#
        ).fetch_all(&self.pool).await?;
        Ok(diagnosticos)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Diagnostico, AppError> {
        let diagnostico = sqlx::query_as!(
            Diagnostico,
            r#"SELECT id_diagnostico, id_cita, descripcion_diagnostico, tratamiento_sugerido
               FROM Diagnosticos WHERE id_diagnostico = $1"#, id
        ).fetch_optional(&self.pool).await?.ok_or(AppError::NotFound)?;
        Ok(diagnostico)
    }

    pub async fn create_diagnostico(&self, nuevo: CreateDiagnostico) -> Result<Diagnostico, AppError> {
        let guardado = sqlx::query_as!(
            Diagnostico,
            r#"INSERT INTO Diagnosticos (id_cita, descripcion_diagnostico, tratamiento_sugerido)
               VALUES ($1, $2, $3) RETURNING id_diagnostico, id_cita, descripcion_diagnostico, tratamiento_sugerido"#,
            nuevo.id_cita, nuevo.descripcion_diagnostico, nuevo.tratamiento_sugerido
        ).fetch_one(&self.pool).await?;
        Ok(guardado)
    }

    pub async fn update(&self, id: i32, act: UpdateDiagnostico) -> Result<Diagnostico, AppError> {
        let diagnostico = sqlx::query_as!(
            Diagnostico,
            r#"UPDATE Diagnosticos
               SET descripcion_diagnostico = COALESCE($2, descripcion_diagnostico),
                   tratamiento_sugerido = COALESCE($3, tratamiento_sugerido)
               WHERE id_diagnostico = $1 RETURNING id_diagnostico, id_cita, descripcion_diagnostico, tratamiento_sugerido"#,
            id, act.descripcion_diagnostico, act.tratamiento_sugerido
        ).fetch_optional(&self.pool).await?.ok_or(AppError::NotFound)?;
        Ok(diagnostico)
    }

    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        let result = sqlx::query!("DELETE FROM Diagnosticos WHERE id_diagnostico = $1", id)
            .execute(&self.pool).await?;
        if result.rows_affected() == 0 { Err(AppError::NotFound) } else { Ok(()) }
    }
}

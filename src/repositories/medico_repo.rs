use crate::error::AppError;
use crate::models::medico::{CreateMedico, Medico, UpdateMedico};
use sqlx::PgPool;

#[derive(Clone)]
pub struct MedicoRepository {
    pool: PgPool,
}

impl MedicoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Medico>, AppError> {
        let medicos = sqlx::query_as!(
            Medico,
            r#"
            SELECT id_medico, nombre, id_especialidad, numero_licencia, telefono
            FROM Medicos
            ORDER BY id_medico ASC
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(medicos)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Medico, AppError> {
        let medico = sqlx::query_as!(
            Medico,
            r#"
            SELECT id_medico, nombre, id_especialidad, numero_licencia, telefono
            FROM Medicos
            WHERE id_medico = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(medico)
    }

    pub async fn create(&self, data: CreateMedico) -> Result<Medico, AppError> {
        let medico = sqlx::query_as!(
            Medico,
            r#"
            INSERT INTO Medicos (nombre, id_especialidad, numero_licencia, telefono)
            VALUES ($1, $2, $3, $4)
            RETURNING id_medico, nombre, id_especialidad, numero_licencia, telefono
            "#,
            data.nombre,
            data.id_especialidad,
            data.numero_licencia,
            data.telefono
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(medico)
    }

    pub async fn update(&self, id: i32, data: UpdateMedico) -> Result<Medico, AppError> {
        let medico = sqlx::query_as!(
            Medico,
            r#"
            UPDATE Medicos
            SET
                nombre = COALESCE($2, nombre),
                id_especialidad = COALESCE($3, id_especialidad),
                numero_licencia = COALESCE($4, numero_licencia),
                telefono = COALESCE($5, telefono)
            WHERE id_medico = $1
            RETURNING id_medico, nombre, id_especialidad, numero_licencia, telefono
            "#,
            id,
            data.nombre,
            data.id_especialidad,
            data.numero_licencia,
            data.telefono
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(medico)
    }

    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM Medicos
            WHERE id_medico = $1
            "#,
            id
        )
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(())
        }
    }
}
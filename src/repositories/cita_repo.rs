use crate::{error::AppError, models::cita::UpdateCita};
use sqlx::PgPool;
use crate::models::cita::{Cita, CreateCita};

#[derive(Clone)]
pub struct CitaRepository {
    pool: PgPool,
}

impl CitaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Cita>, AppError> {
        let citas = sqlx::query_as!(
            Cita,
            r#"
            SELECT id_cita, id_paciente, id_medico, fecha_cita, hora_cita, motivo_consulta 
            FROM Citas
            ORDER BY id_cita ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::from(e))?;

        Ok(citas)
    }

    pub async fn find_by_id(&self, id_cita: i32) -> Result<Cita, AppError> {
        let cita = sqlx::query_as!(
            Cita,
            r#"
            SELECT id_cita, id_paciente, id_medico, fecha_cita, hora_cita, motivo_consulta
            FROM Citas
            WHERE id_cita = $1
            "#,
            id_cita
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::from(e))?
        .ok_or(AppError::NotFound)?;

        Ok(cita)
    }

    pub async fn create_cita(&self, nueva_cita: CreateCita) -> Result<Cita, AppError> {
        let cita_guardada = sqlx::query_as!(
            Cita,
            r#"
            INSERT INTO Citas (id_paciente, id_medico, fecha_cita, hora_cita, motivo_consulta)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id_cita, id_paciente, id_medico, fecha_cita, hora_cita, motivo_consulta
            "#,
            nueva_cita.id_paciente,
            nueva_cita.id_medico,
            nueva_cita.fecha_cita,
            nueva_cita.hora_cita,
            nueva_cita.motivo_consulta
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::from(e))?;

        Ok(cita_guardada)
    }

    pub async fn update(&self, id_cita: i32, cita_actualizada: UpdateCita) -> Result<Cita, AppError> {
        let cita = sqlx::query_as!(
            Cita,
            r#"
            UPDATE Citas
            SET id_paciente = COALESCE($2, id_paciente),
                id_medico = COALESCE($3, id_medico),
                fecha_cita = $4,
                hora_cita = $5,
                motivo_consulta = COALESCE($6, motivo_consulta)
            WHERE id_cita = $1
            RETURNING id_cita, id_paciente, id_medico, fecha_cita, hora_cita, motivo_consulta
            "#,
            id_cita,
            cita_actualizada.id_paciente,
            cita_actualizada.id_medico,
            cita_actualizada.fecha_cita,
            cita_actualizada.hora_cita,
            cita_actualizada.motivo_consulta
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::from(e))?
            .ok_or(AppError::NotFound)?;

        Ok(cita)
    }

    pub async fn delete(&self, id_cita: i32) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM Citas
            WHERE id_cita = $1
            "#,
            id_cita
        )        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(())
        }
        
    }
}
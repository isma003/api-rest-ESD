use crate::error::AppError;
use crate::models::paciente::{CreatePaciente, Paciente, UpdatePaciente};
use sqlx::PgPool;

#[derive(Clone)]
pub struct PacienteRepo {
    pool: PgPool,
}

impl PacienteRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Paciente>, AppError> {
        let pacientes = sqlx::query_as!(
            Paciente,
            r#"
                SELECT id_paciente, nombre, fecha_nacimiento, direccion, tipo_sangre
                FROM pacientes
                ORDER BY id_paciente ASC
            "#
        )
            .fetch_all(&self.pool)
            .await?;
        Ok(pacientes)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Paciente, AppError> {
        let paciente = sqlx::query_as!(
            Paciente,
            r#"
                SELECT id_paciente, nombre, fecha_nacimiento, direccion, tipo_sangre
                FROM pacientes
                WHERE id_paciente = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(paciente)
    }

    pub async fn create(&self, data: CreatePaciente) -> Result<Paciente, AppError> {
        let paciente = sqlx::query_as!(
            Paciente,
            r#"
                INSERT INTO pacientes (nombre, fecha_nacimiento, direccion, tipo_sangre)
                VALUES ($1, $2, $3, $4)
                RETURNING id_paciente, nombre, fecha_nacimiento, direccion, tipo_sangre
            "#,
            data.nombre,
            data.fecha_nacimiento,
            data.direccion,
            data.tipo_sangre
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(paciente)
    }

    pub async fn update(&self, id: i32, data: UpdatePaciente) -> Result<Paciente, AppError> {
        let paciente = sqlx::query_as!(
            Paciente,
            r#"
                UPDATE pacientes
                SET
                    nombre = COALESCE($2, nombre),
                    fecha_nacimiento = COALESCE($3, fecha_nacimiento),
                    direccion = COALESCE($4, direccion),
                    tipo_sangre = COALESCE($5, tipo_sangre)
                WHERE id_paciente = $1
                RETURNING id_paciente, nombre, fecha_nacimiento, direccion, tipo_sangre
            "#,
            id,
            data.nombre,
            data.fecha_nacimiento,
            data.direccion,
            data.tipo_sangre
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(paciente)
    }

    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
                DELETE FROM pacientes
                WHERE id_paciente = $1
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
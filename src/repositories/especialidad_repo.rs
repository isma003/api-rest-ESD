use crate::error::AppError;
use crate::models::especialidad::{CreateEspecialidad, Especialidad, UpdateEspecialidad};
use sqlx::PgPool;

#[derive(Clone)]
pub struct EspecialidadRepo {
    pool: PgPool,
}

impl EspecialidadRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Especialidad>, AppError> {
        let especialidades = sqlx::query_as!(
            Especialidad,
            r#"
                SELECT id_especialidad, nombre_especialidad, descripcion
                FROM especialidades
                ORDER BY id_especialidad ASC
            "#
        )
            .fetch_all(&self.pool)
            .await?;
        Ok(especialidades)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Especialidad, AppError> {
        let especialidad = sqlx::query_as!(
            Especialidad,
            r#"
                SELECT id_especialidad, nombre_especialidad, descripcion
                FROM especialidades
                WHERE id_especialidad = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(especialidad)
    }

    pub async fn create(&self, data: CreateEspecialidad) -> Result<Especialidad, AppError> {
        let especialidad = sqlx::query_as!(
            Especialidad,
            r#"
                INSERT INTO especialidades (nombre_especialidad, descripcion)
                VALUES ($1, $2)
                RETURNING id_especialidad, nombre_especialidad, descripcion
            "#,
            data.nombre_especialidad,
            data.descripcion
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(especialidad)
    }

    pub async fn update(
        &self,
        id: i32,
        data: UpdateEspecialidad,
    ) -> Result<Especialidad, AppError> {
        let especialidad = sqlx::query_as!(
            Especialidad,
            r#"
                UPDATE especialidades
                SET
                    nombre_especialidad = COALESCE($2, nombre_especialidad),
                    descripcion = COALESCE($3, descripcion)
                WHERE id_especialidad = $1
                RETURNING id_especialidad, nombre_especialidad, descripcion
            "#,
            id,
            data.nombre_especialidad,
            data.descripcion
        )
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound)?;
        Ok(especialidad)
    }

    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        let result = sqlx::query!(
            r#"
                DELETE FROM especialidades
                WHERE id_especialidad = $1
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
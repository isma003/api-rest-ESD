mod db;
mod error;
mod models;
mod repositories;
mod services;
mod controllers;
mod routes;
mod utils;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .init();

    let pool = db::connect().await;

    let especialidad_repo = repositories::especialidad_repo::EspecialidadRepo::new(pool.clone());
    let especialidad_service = services::especialidad_service::EspecialidadService::new(especialidad_repo);

    let cita_repo = repositories::cita_repo::CitaRepository::new(pool.clone());
    let cita_service = services::cita_service::CitaService::new(cita_repo);

    let paciente_repo = repositories::paciente_repo::PacienteRepo::new(pool.clone());
    let paciente_service = services::paciente_service::PacienteService::new(paciente_repo);

    let diagnostico_repo = repositories::diagnostico_repo::DiagnosticoRepository::new(pool.clone());
    let diagnostico_service = services::diagnostico_service::DiagnosticoService::new(diagnostico_repo);

    let medico_repo = repositories::medico_repo::MedicoRepository::new(pool.clone());
    let medico_service = services::medico_service::MedicoService::new(medico_repo);

    let app = routes::create_router(especialidad_service, cita_service, paciente_service, diagnostico_service, medico_service);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Servidor escuchando en http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
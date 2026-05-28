use axum::{routing::get, Router};
use crate::controllers::{cita_controller, especialidad_controller, paciente_controller, diagnostico_controller, medico_controller};
use crate::services::cita_service::CitaService;
use crate::services::especialidad_service::EspecialidadService;
use crate::services::paciente_service::PacienteService;
use crate::services::diagnostico_service::DiagnosticoService;
use crate::services::medico_service::MedicoService;

pub fn create_router(
    especialidad_service: EspecialidadService,
    cita_service: CitaService,
    paciente_service: PacienteService,
    diagnostico_service: DiagnosticoService,
    medico_service: MedicoService
) -> Router {

    let especialidad_routes = Router::new()
        .route("/", get(especialidad_controller::listar_especialidades).post(especialidad_controller::crear_especialidad))
        .route("/{id}", get(especialidad_controller::obtener_especialidad)
            .put(especialidad_controller::actualizar_especialidad)
            .delete(especialidad_controller::eliminar_especialidad))
        .with_state(especialidad_service.clone());

    let cita_routes = Router::new()
        .route("/", get(cita_controller::obtener_todas).post(cita_controller::crear_cita))
        .route("/{id}", get(cita_controller::obtener_por_id)
            .put(cita_controller::actualizar_cita)
            .delete(cita_controller::eliminar_cita))
        .with_state(cita_service.clone());

    let paciente_routes = Router::new()
        .route("/", get(paciente_controller::listar_pacientes).post(paciente_controller::crear_paciente))
        .route("/{id}", get(paciente_controller::obtener_paciente)
            .put(paciente_controller::actualizar_paciente)
            .delete(paciente_controller::eliminar_paciente))
        .with_state(paciente_service.clone());

    let diagnostico_routes = Router::new()
        .route("/", get(diagnostico_controller::obtener_todos).post(diagnostico_controller::crear_diagnostico))
        .route("/{id}", get(diagnostico_controller::obtener_por_id)
            .put(diagnostico_controller::actualizar_diagnostico)
            .delete(diagnostico_controller::eliminar_diagnostico))
        .with_state(diagnostico_service.clone());

    let medico_routes = Router::new()
        .route("/", get(medico_controller::obtener_todos).post(medico_controller::crear_medico))
        .route("/{id}", get(medico_controller::obtener_por_id)
            .put(medico_controller::actualizar_medico)
            .delete(medico_controller::eliminar_medico))
        .with_state(medico_service.clone());


    Router::new()
        .nest("/especialidades", especialidad_routes)
        .nest("/citas", cita_routes)
        .nest("/pacientes", paciente_routes)
        .nest("/medicos", medico_routes)
        .nest("/diagnosticos", diagnostico_routes)
}
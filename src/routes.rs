use axum::{routing::get, Router};
use crate::controllers::{cita_controller, especialidad_controller, paciente_controller}; // Agregamos paciente_controller
use crate::services::cita_service::CitaService;
use crate::services::especialidad_service::EspecialidadService;
use crate::services::paciente_service::PacienteService;

pub fn create_router(
    especialidad_service: EspecialidadService,
    cita_service: CitaService,
    paciente_service: PacienteService // Recibimos el servicio
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

    // --- Router de Pacientes (NUEVO) ---
    let paciente_routes = Router::new()
        .route("/", get(paciente_controller::listar_pacientes).post(paciente_controller::crear_paciente))
        .route("/{id}", get(paciente_controller::obtener_paciente)
            .put(paciente_controller::actualizar_paciente)
            .delete(paciente_controller::eliminar_paciente))
        .with_state(paciente_service.clone());

    // TODO: Agregar rutas de otros módulos aquí cuando estén listos.
    // Ejemplo:
    // let medico_routes = Router::new()
    //     ...

    Router::new()
        .nest("/especialidades", especialidad_routes)
        .nest("/citas", cita_routes)
        .nest("/pacientes", paciente_routes) // Activamos la ruta de pacientes
    // .nest("/medicos", medico_routes)
    // .nest("/diagnosticos", diagnostico_routes)
}
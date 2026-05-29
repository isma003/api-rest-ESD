use actix_web::{web, HttpResponse, Responder, Scope};
use crate::models::medico::CreateMedicoDto;
use crate::services::medico_service::MedicoService;

pub fn medico_routes(service: web::Data<MedicoService>) -> Scope {
    web::scope("/api/medicos")
        .app_data(service)
        .route("", web::get().to(listar_medicos))
        .route("/{id}", web::get().to(obtener_medico))
        .route("", web::post().to(crear_medico))
        .route("/{id}", web::put().to(actualizar_medico))
        .route("/{id}", web::delete().to(eliminar_medico))
}

async fn listar_medicos(service: web::Data<MedicoService>) -> impl Responder {
    let lista = service.obtener_todos();
    HttpResponse::Ok().json(lista)
}

async fn obtener_medico(id: web::Path<u32>, service: web::Data<MedicoService>) -> impl Responder {
    match service.obtener_por_id(id.into_inner()) {
        Some(medico) => HttpResponse::Ok().json(medico),
        None => HttpResponse::NotFound().json("Médico no encontrado"),
    }
}

async fn crear_medico(dto: web::Json<CreateMedicoDto>, service: web::Data<MedicoService>) -> impl Responder {
    let nuevo = service.registrar_medico(dto.into_inner());
    HttpResponse::Created().json(nuevo)
}

async fn actualizar_medico(id: web::Path<u32>, dto: web::Json<CreateMedicoDto>, service: web::Data<MedicoService>) -> impl Responder {
    match service.actualizar_medico(id.into_inner(), dto.into_inner()) {
        Some(actualizado) => HttpResponse::Ok().json(actualizado),
        None => HttpResponse::NotFound().json("No se pudo actualizar, médico no encontrado"),
    }
}

async fn eliminar_medico(id: web::Path<u32>, service: web::Data<MedicoService>) -> impl Responder {
    if service.eliminar_medico(id.into_inner()) {
        HttpResponse::Ok().json("Médico eliminado correctamente")
    } else {
        HttpResponse::NotFound().json("Médico no encontrado")
    }
}
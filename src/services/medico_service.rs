use crate::models::medico::{Medico, CreateMedicoDto};
use crate::repositories::medico_repo::MedicoRepository;

pub struct MedicoService {
    repo: MedicoRepository,
}

impl MedicoService {
    pub fn new(repo: MedicoRepository) -> Self {
        MedicoService { repo }
    }

    pub fn obtener_todos(&self) -> Vec<Medico> {
        self.repo.find_all()
    }

    pub fn obtener_por_id(&self, id: u32) -> Option<Medico> {
        self.repo.find_by_id(id)
    }

    pub fn registrar_medico(&self, dto: CreateMedicoDto) -> Medico {
        let medicos = self.repo.find_all();
        let nuevo_id = medicos.iter().map(|m| m.id).max().unwrap_or(0) + 1;

        let nuevo_medico = Medico {
            id: nuevo_id,
            nombre: dto.nombre,
            especialidad: dto.space_especialidad,
            junta_vigilancia: dto.junta_vigilancia,
            disponible: dto.disponible,
        };

        self.repo.create(nuevo_medico)
    }

    pub fn actualizar_medico(&self, id: u32, dto: CreateMedicoDto) -> Option<Medico> {
        let medico_existente = self.repo.find_by_id(id)?;
        
        let medico_actualizado = Medico {
            id: medico_existente.id,
            nombre: dto.nombre,
            especialidad: dto.space_especialidad,
            junta_vigilancia: dto.junta_vigilancia,
            disponible: dto.disponible,
        };

        self.repo.update(id, medico_actualizado)
    }

    pub fn eliminar_medico(&self, id: u32) -> bool {
        self.repo.delete(id)
    }
}
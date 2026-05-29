use crate::models::medico::Medico;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct MedicoRepository {
    db: Arc<Mutex<Vec<Medico>>>,
}

impl MedicoRepository {
    pub fn new(db: Arc<Mutex<Vec<Medico>>>) -> Self {
        MedicoRepository { db }
    }

    pub fn find_all(&self) -> Vec<Medico> {
        let pool = self.db.lock().unwrap();
        pool.clone()
    }

    pub fn find_by_id(&self, id: u32) -> Option<Medico> {
        let pool = self.db.lock().unwrap();
        pool.iter().find(|m| m.id == id).cloned()
    }

    pub fn create(&self, nuevo: Medico) -> Medico {
        let mut pool = self.db.lock().unwrap();
        pool.push(nuevo.clone());
        nuevo
    }

    pub fn update(&self, id: u32, actualizado: Medico) -> Option<Medico> {
        let mut pool = self.db.lock().unwrap();
        if let Some(index) = pool.iter().position(|m| m.id == id) {
            pool[index] = actualizado.clone();
            Some(actualizado)
        } else {
            None
        }
    }

    pub fn delete(&self, id: u32) -> bool {
        let mut pool = self.db.lock().unwrap();
        if let Some(index) = pool.iter().position(|m| m.id == id) {
            pool.remove(index);
            true
        } else {
            false
        }
    }
}
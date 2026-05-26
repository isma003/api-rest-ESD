# Clínica API — Guía del Equipo

---

## Qué le toca a cada integrante

### Integrante 1 — Base del proyecto + Especialidades
`src/main.rs` · `src/db.rs` · `src/routes.rs` · `src/error.rs`
`src/models/especialidad.rs` · `src/repositories/especialidad_repo.rs` · `src/services/especialidad_service.rs` · `src/controllers/especialidad_controller.rs`

### Integrante 2 — Médicos
`src/models/medico.rs` · `src/repositories/medico_repo.rs` · `src/services/medico_service.rs` · `src/controllers/medico_controller.rs`
> FK a `Especialidades` — el `id_especialidad` debe existir en la DB.

### Integrante 3 — Pacientes
`src/models/paciente.rs` · `src/repositories/paciente_repo.rs` · `src/services/paciente_service.rs` · `src/controllers/paciente_controller.rs`
> `fecha_nacimiento` es `DATE` en PostgreSQL → usa `chrono::NaiveDate` en Rust.

### Integrante 4 — Citas
`src/models/cita.rs` · `src/repositories/cita_repo.rs` · `src/services/cita_service.rs` · `src/controllers/cita_controller.rs`
> FK a `Pacientes` y `Medicos`. `hora_cita` es `TIME` → usa `chrono::NaiveTime`.

### Integrante 5 — Diagnósticos
`src/models/diagnostico.rs` · `src/repositories/diagnostico_repo.rs` · `src/services/diagnostico_service.rs` · `src/controllers/diagnostico_controller.rs`
> FK a `Citas` — la cita debe existir antes de crear un diagnóstico.

---

## Instalar Docker

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update && sudo apt-get install -y ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update && sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo usermod -aG docker $USER && newgrp docker
```

### Windows
1. Descargar Docker Desktop desde https://www.docker.com/products/docker-desktop/
2. Instalarlo y reiniciar
3. Abrir Docker Desktop y esperar a que inicie

---

## Clonar el repositorio

```bash
git clone <URL_DEL_REPO>
cd clinica_api
cp .env.example .env
```

---

## Git — subir y traer cambios

```bash
# Traer lo último antes de ponerte a trabajar
git pull origin main

# Subir tus cambios
git add .
git commit -m "feat: descripción de lo que hiciste"
git push origin main
```

Si al hacer `pull` hay conflictos, Git te avisa qué archivos tienen problema. Resuélvelos y luego:
```bash
git add .
git commit -m "fix: resolver conflicto"
git push origin main
```

---

## Levantar Docker

```bash
# Solo la base de datos
docker compose up db -d

# Solo pgAdmin (para ver la DB visualmente)
docker compose up pgadmin -d

# Ambos a la vez
docker compose up db pgadmin -d

# Apagar todo
docker compose down
```

---

## Conectar pgAdmin a la base de datos

1. Entrar a `http://localhost:8080`
2. Login: `admin@admin.com` / `admin`
3. Click derecho en **Servers** → **Register** → **Server**
4. En la pestaña **General** pon cualquier nombre, ej: `clinica`
5. En la pestaña **Connection** usa estos datos:

| Campo    | Valor        |
|----------|--------------|
| Host     | `clinica_db` |
| Port     | `5432`       |
| Database | `clinica_db` |
| Username | `clinica_user` |
| Password | `clinica_pass` |

> El host es `clinica_db` (nombre del contenedor) y el puerto es `5432` porque pgAdmin se comunica con postgres por la red interna de Docker, no por el puerto de tu máquina.

---

## Instalar sqlx-cli y ejecutar migraciones

```bash
# Instalar sqlx-cli (solo la primera vez)
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Verificar migraciones pendientes
sqlx migrate info

# Ejecutar migraciones
sqlx migrate run
```

> Requiere que la DB esté corriendo y que `DATABASE_URL` esté en el `.env`.

---

## Ejecutar el proyecto

```bash
# Compilar
cargo build

# Ejecutar
cargo run

# Con hot-reload (se reinicia solo al cambiar código)
# Instalar una sola vez:
cargo install cargo-watch
# Luego:
cargo watch -q -c -w src/ -x run
```

La API queda en `http://localhost:3030`. Prueba: `http://localhost:3000/health` → debe responder `OK`.
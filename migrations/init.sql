CREATE TABLE Especialidades (
                                id_especialidad SERIAL PRIMARY KEY,
                                nombre_especialidad VARCHAR(100) NOT NULL,
                                descripcion TEXT
);

CREATE TABLE Medicos (
                         id_medico SERIAL PRIMARY KEY,
                         nombre VARCHAR(100) NOT NULL,
                         id_especialidad INT REFERENCES Especialidades(id_especialidad),
                         numero_licencia VARCHAR(50) UNIQUE,
                         telefono VARCHAR(20)
);

CREATE TABLE Pacientes (
                           id_paciente SERIAL PRIMARY KEY,
                           nombre VARCHAR(100) NOT NULL,
                           fecha_nacimiento DATE,
                           direccion TEXT,
                           tipo_sangre VARCHAR(5)
);

CREATE TABLE Citas (
                       id_cita SERIAL PRIMARY KEY,
                       id_paciente INT REFERENCES Pacientes(id_paciente),
                       id_medico INT REFERENCES Medicos(id_medico),
                       fecha_cita DATE NOT NULL,
                       hora_cita TIME NOT NULL,
                       motivo_consulta TEXT
);

CREATE TABLE Diagnosticos (
                              id_diagnostico SERIAL PRIMARY KEY,
                              id_cita INT REFERENCES Citas(id_cita),
                              descripcion_diagnostico TEXT,
                              tratamiento_sugerido TEXT
);
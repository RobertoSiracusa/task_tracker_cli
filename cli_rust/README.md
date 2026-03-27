# Task Tracker CLI

Un gestor de tareas por línea de comandos escrito en **Rust**, con sistema de autenticación, roles de usuario y persistencia en archivos JSON.

---

## Descripción General

Task Tracker CLI es una aplicación de terminal que permite a los usuarios crear, listar, actualizar y eliminar tareas personales. Cada operación requiere iniciar sesión con credenciales. El sistema implementa un modelo de permisos donde:

- **Administrador** (ID 1): puede gestionar usuarios y modificar/eliminar cualquier tarea.
- **Usuarios regulares**: solo pueden modificar/eliminar sus propias tareas.

Los datos se almacenan en archivos JSON locales (`tareas.json` y `usuarios.json`), que se crean automáticamente en la primera ejecución.

---

## Requisitos previos

- [Rust](https://www.rust-lang.org/tools/install) (edición 2024 o superior)
- Cargo (incluido con Rust)

---

## Instalación y ejecución

```bash
# 1. Clonar el repositorio
git clone <url-del-repositorio>
cd cli_rust/task_tracker_cli

# 2. Compilar el proyecto
cargo build

# 3. Ejecutar (mostrará el menú de ayuda)
cargo run
```

En la primera ejecución, se crea automáticamente la carpeta `src/storage/` con dos archivos:
- `usuarios.json` — con las cuentas predeterminadas:
  - `admin` / `123` (ID 1, administrador)
  - `invitado` / `456` (ID 2, usuario regular)
- `tareas.json` — lista vacía de tareas

---

## Autenticación

**Todos los comandos** requieren inicio de sesión. Al ejecutar cualquier comando, el sistema solicita usuario y contraseña por consola:

```
=== INICIO DE SESIÓN ===
Usuario: admin
Contraseña: 123

¡Bienvenido de nuevo, admin!
```

---

## Comandos disponibles

### `agregar` — Crear una nueva tarea

Agrega una tarea con descripción. El ID se asigna automáticamente y el estado inicial es `todo`.

```bash
cargo run agregar "Comprar materiales para el proyecto"
```

```
¡Tarea agregada con éxito! (ID: 1)
```

---

### `listar` — Ver tareas

Muestra tareas filtradas por estado. Incluye el nombre del autor de cada tarea.

```bash
cargo run listar <filtro>
```

| Filtro        | Descripción                        |
|---------------|------------------------------------|
| `todas`       | Muestra todas las tareas           |
| `pendientes`  | Solo tareas con estado `todo`      |
| `en-proceso`  | Solo tareas con estado `in-progress` |
| `completadas` | Solo tareas con estado `done`      |

**Ejemplo:**

```bash
cargo run listar todas
```

```
--- RESULTADOS PARA: TODAS ---
ID   | DESCRIPCIÓN                    | ESTADO       | AUTOR
-----------------------------------------------------------------------------
1    | Comprar materiales             | Pendiente    | admin
2    | Revisar documentación          | En Proceso   | invitado
-----------------------------------------------------------------------------
```

---

### `actualizar` — Cambiar estado de una tarea

Actualiza el estado de una tarea por su ID. Solo el creador de la tarea o el administrador pueden hacerlo.

```bash
cargo run actualizar <ID_TAREA> <nuevo_estado>
```

| Estado         | Significado       |
|----------------|-------------------|
| `todo`         | Pendiente         |
| `in-progress`  | En proceso        |
| `done`         | Completada        |

**Ejemplo:**

```bash
cargo run actualizar 1 in-progress
```

```
Tarea 1 actualizada a 'in-progress' correctamente.
```

---

### `eliminar` — Borrar una tarea

Elimina una tarea por su ID. Solo el creador o el administrador pueden eliminarla.

```bash
cargo run eliminar <ID_TAREA>
```

**Ejemplo:**

```bash
cargo run eliminar 3
```

```
Tarea 3 eliminada con éxito.
```

---

### `gestionar-usuarios` — Administración de usuarios (Solo Admin)

Requiere estar logueado como `admin` (ID 1). Tiene los siguientes subcomandos:

```bash
cargo run gestionar-usuarios <subcomando>
```

| Subcomando  | Descripción                                                |
|-------------|------------------------------------------------------------|
| `crear`     | Crea un nuevo usuario (pide nombre y contraseña por consola) |
| `modificar` | Modifica nombre/contraseña de un usuario por ID            |
| `eliminar`  | Elimina un usuario por ID                                  |
| `limpiar`   | Elimina **todos** los usuarios excepto admin e invitado     |

>  Los usuarios con ID 1 (`admin`) y 2 (`invitado`) son permanentes y no se pueden modificar ni eliminar.

**Ejemplos:**

```bash
# Crear usuario (interactivo)
cargo run gestionar-usuarios crear

# Modificar usuario con ID 5
cargo run gestionar-usuarios modificar 5

# Eliminar usuario con ID 3
cargo run gestionar-usuarios eliminar 3

# Borrar todos los usuarios adicionales
cargo run gestionar-usuarios limpiar
```

---

## Estructura de carpetas

```
task_tracker_cli/
├── Cargo.toml                  # Configuración del proyecto y dependencias
├── Cargo.lock                  # Versiones fijadas de dependencias
├── README.md                   # Este archivo
├── src/
│   ├── main.rs                 # Punto de entrada, enrutador de comandos
│   ├── composables/            # Modelos de datos (structs)
│   │   ├── mod.rs              # Declaración de submódulos
│   │   ├── tarea.rs            # Struct Tarea (id, descripcion, estado, etc.)
│   │   └── usuario.rs          # Struct Usuario (id, nombre, password)
│   ├── process/                # Lógica de negocio por comando
│   │   ├── mod.rs              # Declaración de submódulos
│   │   ├── agregar.rs          # Lógica para crear tareas
│   │   ├── listar.rs           # Lógica para filtrar y mostrar tareas
│   │   ├── actualizar.rs       # Lógica para cambiar estado de tareas
│   │   ├── eliminar.rs         # Lógica para borrar tareas
│   │   ├── auth.rs             # Proceso de inicio de sesión
│   │   ├── file_process.rs     # Lectura/escritura de archivos JSON
│   │   └── usuarios_process.rs # CRUD de usuarios (solo admin)
│   ├── storage/                # Almacenamiento persistente (JSON)
│   │   ├── tareas.json         # Base de datos de tareas
│   │   └── usuarios.json       # Base de datos de usuarios
│   └── utils/                  # Utilidades (reservado para futuro uso)
│       └── mod.rs
└── target/                     # Artefactos de compilación (generado por Cargo)
```

---

## Estructura de funciones

### `main.rs` — Punto de entrada

| Función                  | Descripción                                                                 |
|--------------------------|-----------------------------------------------------------------------------|
| `main()`                 | Lee argumentos de CLI, inicializa datos, autentica al usuario y enruta al comando correspondiente |
| `mostrar_menu_principal()` | Imprime el menú de ayuda con los comandos disponibles                     |

### `composables/tarea.rs` — Modelo `Tarea`

| Función / Método              | Descripción                                               |
|-------------------------------|-----------------------------------------------------------|
| `Tarea::new(id, desc, uid)`   | Constructor: crea tarea con estado `todo` y timestamps actuales |
| `get_id()`                    | Retorna el ID de la tarea                                 |
| `get_descripcion()`           | Retorna la descripción                                    |
| `get_estado()`                | Retorna el estado actual (`todo`, `in-progress`, `done`)  |
| `get_usuario_id()`            | Retorna el ID del usuario creador                         |
| `cambiar_estado(nuevo)`       | Actualiza el estado y el timestamp `actualizado_en`       |

### `composables/usuario.rs` — Modelo `Usuario`

| Función / Método                  | Descripción                          |
|-----------------------------------|--------------------------------------|
| `Usuario::new(id, nombre, pass)`  | Constructor del usuario              |
| `get_id()`, `get_nombre()`, `get_password()` | Getters de los campos    |
| `set_nombre(nuevo)`, `set_password(nueva)`    | Setters para modificación |

### `process/file_process.rs` — Persistencia

| Función                    | Descripción                                                        |
|----------------------------|--------------------------------------------------------------------|
| `inicializar_usuarios()`   | Carga usuarios desde JSON o crea los predeterminados               |
| `inicializar_memoria()`    | Carga tareas desde JSON o crea una lista vacía                     |
| `guardar_tareas(tareas)`   | Serializa y guarda el vector de tareas en `tareas.json`            |
| `guardar_usuarios(users)`  | Serializa y guarda el vector de usuarios en `usuarios.json`        |

### `process/auth.rs` — Autenticación

| Función                      | Descripción                                                    |
|------------------------------|----------------------------------------------------------------|
| `iniciar_sesion(usuarios)`   | Solicita credenciales por consola y valida contra la lista de usuarios. Retorna `Some(Usuario)` o `None` |

### `process/agregar.rs` — Agregar tarea

| Función                           | Descripción                                            |
|-----------------------------------|--------------------------------------------------------|
| `ejecutar(tareas, args, uid)`     | Valida args, genera ID automático, crea la tarea y la guarda |

### `process/listar.rs` — Listar tareas

| Función                                | Descripción                                                  |
|----------------------------------------|--------------------------------------------------------------|
| `ejecutar(tareas, args, usuarios)`     | Filtra tareas por estado y las muestra en formato tabla con nombre del autor |

### `process/actualizar.rs` — Actualizar tarea

| Función                           | Descripción                                                              |
|-----------------------------------|--------------------------------------------------------------------------|
| `ejecutar(tareas, args, uid)`     | Valida permisos del usuario, busca la tarea por ID y cambia su estado    |

### `process/eliminar.rs` — Eliminar tarea

| Función                           | Descripción                                                              |
|-----------------------------------|--------------------------------------------------------------------------|
| `ejecutar(tareas, args, uid)`     | Valida permisos, busca la tarea por ID, la elimina del vector y guarda   |

### `process/usuarios_process.rs` — Gestión de usuarios

| Función                              | Descripción                                              |
|--------------------------------------|----------------------------------------------------------|
| `ejecutar(usuarios, args)`           | Enrutador de subcomandos (crear, modificar, eliminar, limpiar) |
| `crear_usuario(lista)`               | Proceso interactivo para agregar un usuario               |
| `modificar_usuario(lista, args)`     | Modifica nombre/contraseña por ID (protege IDs 1 y 2)    |
| `eliminar_usuario(lista, args)`      | Elimina un usuario por ID (protege IDs 1 y 2)            |
| `limpiar_usuarios(lista)`            | Elimina todos los usuarios excepto los predeterminados    |
| `actualizar_archivo(lista, msg)`     | Helper: guarda en disco y muestra mensaje de resultado    |

---

## Dependencias

| Crate         | Versión | Uso                                              |
|---------------|---------|--------------------------------------------------|
| `serde`       | 1.0     | Serialización/deserialización de structs          |
| `serde_json`  | 1.0     | Formato JSON para persistencia en archivos        |
| `chrono`      | 0.4.44  | Manejo de fechas y timestamps para las tareas     |

---

## Flujo general de ejecución

```
cargo run <comando> [args]
        │
        ├─ Sin argumentos → Muestra menú de ayuda y sale
        │
        ├─ Carga usuarios.json
        ├─ Solicita credenciales (login interactivo)
        ├─ Carga tareas.json
        │
        └─ Ejecuta el comando:
             ├── agregar "desc"             → Crea tarea
             ├── listar <filtro>            → Muestra tareas filtradas
             ├── actualizar <id> <estado>   → Cambia estado
             ├── eliminar <id>              → Borra tarea
             └── gestionar-usuarios <sub>   → CRUD de usuarios (solo admin)
```

---

## Licencia

Proyecto académico — Universidad / Materia: Lenguajes de Programación.
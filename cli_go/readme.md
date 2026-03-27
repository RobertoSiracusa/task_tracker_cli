# Task Tracker CLI (Go)

Un gestor de tareas por linea de comandos escrito en **Go**, con sistema de registro de usuarios, inicio de sesion persistente y almacenamiento en archivos JSON.

---

## Descripcion General

Task Tracker CLI es una aplicacion de terminal que permite registrar usuarios, iniciar sesion y gestionar tareas personales. A diferencia de pedir credenciales en cada comando, este proyecto utiliza un sistema de **sesion persistente**: una vez que el usuario inicia sesion con `log`, los demas comandos (`add`, `listTask`, `act`, `del`) leen la sesion guardada automaticamente.

El sistema implementa un modelo de permisos donde:

- **Administrador** (usuario `admin`): puede gestionar (modificar/eliminar) otros usuarios mediante el comando `gest`.
- **Usuarios regulares**: pueden crear, listar, actualizar y eliminar tareas.

Los datos se almacenan en tres archivos JSON dentro de `src/storage/`:
- `user.json` -- Base de datos de usuarios registrados.
- `task.json` -- Base de datos de tareas.
- `session.json` -- Sesion activa del usuario logueado.

---

## Requisitos previos

- [Go](https://go.dev/dl/) version 1.20 o superior

---

## Instalacion y ejecucion

```bash
# 1. Clonar el repositorio
git clone <url-del-repositorio>
cd cli/task_tracker_cli/cli_go/src

# 2. Ejecutar (mostrara el menu de ayuda)
go run main.go
```

No se necesitan dependencias externas. El proyecto usa unicamente la libreria estandar de Go.

---

## Flujo de uso

El flujo tipico de trabajo es:

1. Registrar un usuario con `reg`
2. Iniciar sesion con `log` (esto guarda la sesion en `session.json`)
3. Usar los comandos de tareas (`add`, `listTask`, `act`, `del`)

---

## Comandos disponibles

### `reg` -- Registrar un nuevo usuario

Crea un nuevo usuario de forma interactiva. Se le asigna un ID numerico automaticamente.

```bash
go run main.go reg
```

```
- Nombre de usuario: carlos
- Clave a asignar: mi_clave

Registro de usuario completado
```

---

### `log` -- Iniciar sesion

Solicita credenciales y, si son correctas, guarda la sesion en `session.json`. Los comandos posteriores usan esta sesion.

```bash
go run main.go log
```

```
- Usuario: carlos
- Clave: mi_clave

-> Sesion iniciada como: carlos
```

---

### `list` -- Listar usuarios registrados

Muestra todos los usuarios registrados en el sistema con su ID y nombre.

```bash
go run main.go list
```

```
-------------------------------------
   ID   |   USUARIO
-------------------------------------
1       | admin
2       | carlos
-------------------------------------
```

---

### `add` -- Crear una nueva tarea

Agrega una tarea asociada al usuario de la sesion activa. Se le asigna un ID con formato `T-001`, `T-002`, etc.

**Requiere sesion activa** (haber ejecutado `log` previamente).

```bash
go run main.go add "Revisar documentacion del proyecto"
```

```
Tarea 'Revisar documentacion del proyecto' creada (ID User: 2)
```

---

### `listTask` -- Listar tareas

Muestra las tareas en formato tabla con estadisticas. Se puede filtrar por estado.

```bash
go run main.go listTask <filtro>
```

| Filtro                    | Descripcion                              |
|---------------------------|------------------------------------------|
| `todas` (o sin filtro)    | Muestra todas las tareas                 |
| `pendiente` / `pendientes`| Solo tareas con estado `pendiente`       |
| `en-proceso` / `proceso`  | Solo tareas con estado `en-proceso`      |
| `lista` / `listas`        | Solo tareas completadas                  |

**Ejemplo:**

```bash
go run main.go listTask todas
```

```
Visualizando: [todas]
--------------------------------------------------------------------------------
ID         | DESCRIPCION               | ESTADO          | USUARIO
--------------------------------------------------------------------------------
T-001      | Revisar documentacion     | pendiente       | carlos
T-002      | Preparar presentacion     | en-proceso      | admin
--------------------------------------------------------------------------------
*-*-*-*-* ESTADISTICAS TOTALES: *-*-*-*-*
-> Total: 2
-> Pendientes: 1
-> En Proceso: 1
-> Listas: 0
--------------------------------------------------------------------------------
```

---

### `act` -- Actualizar estado de una tarea

Cambia el estado de una tarea dado su ID.

```bash
go run main.go act <id_tarea> <nuevo_estado>
```

| Estado        | Significado       |
|---------------|-------------------|
| `pendiente`   | Pendiente         |
| `en-proceso`  | En proceso        |
| `completo`    | Completada        |

**Ejemplo:**

```bash
go run main.go act T-001 en-proceso
```

```
-> Tarea T-001 actualizada a estado: 'en-proceso'
```

---

### `del` -- Eliminar una tarea

Elimina una tarea por su ID. Solicita confirmacion antes de borrar.

```bash
go run main.go del <id_tarea>
```

**Ejemplo:**

```bash
go run main.go del T-002
```

```
Estas seguro de que deseas borrar la tarea T-002? (s/n): s
Tarea T-002 eliminada con exito.
```

---

### `gest` -- Gestion de usuarios (Solo Admin)

Requiere haber iniciado sesion como `admin`. Permite modificar o eliminar un usuario por su ID.

```bash
go run main.go gest <id_usuario>
```

Al ejecutarlo, el sistema pregunta que accion realizar:

- `borrar` -- Elimina al usuario (no se puede borrar la cuenta admin).
- `modificar` -- Permite cambiar nombre y/o contrasena (dejar vacio para omitir).

**Ejemplo:**

```bash
go run main.go gest 3
```

```
Usuario seleccionado: carlos
Que accion desea realizar? (borrar / modificar): modificar
Nuevo nombre (actual: carlos, Enter para omitir): carlos_v2
Nueva contrasena (Enter para omitir):
-> Informacion de usuario actualizada.
```

---

## Estructura de carpetas

```
cli_go/
├── readme.md                              # Este archivo
└── src/
    ├── go.mod                             # Modulo Go y version
    ├── main.go                            # Punto de entrada, enrutador de comandos
    ├── composables/                       # Modelos de datos y logica de negocio
    │   ├── composables.go                 # Struct Login (usuario), registro, lectura/escritura
    │   ├── session.go                     # Guardar y obtener sesion activa
    │   └── taskComposables.go             # Struct Task, CRUD de tareas
    └── storage/                           # Almacenamiento persistente (JSON)
        ├── user.json                      # Base de datos de usuarios
        ├── task.json                      # Base de datos de tareas
        └── session.json                   # Sesion del usuario logueado
```

---

## Estructura de funciones

### `main.go` -- Punto de entrada

| Funcion                  | Descripcion                                                                 |
|--------------------------|-----------------------------------------------------------------------------|
| `main()`                 | Lee argumentos del CLI y enruta al comando correspondiente (`reg`, `log`, `list`, `add`, `listTask`, `act`, `del`, `gest`) |
| `menuComandos()`         | Imprime el menu de ayuda con los comandos de tareas disponibles             |
| `mostrarInicioSesion()`  | Imprime la cabecera decorativa para las secciones de login/registro         |

### `composables/composables.go` -- Modelo `Login` (usuarios)

| Funcion / Metodo                  | Descripcion                                                          |
|-----------------------------------|----------------------------------------------------------------------|
| `NewLogin(name, pass)`            | Constructor: crea una instancia de Login con nombre y contrasena     |
| `Login.SaveData(archivo)`         | Agrega el usuario al archivo JSON con ID autoincremental             |
| `GetAllUsers(archivo)`            | Lee todos los usuarios del archivo JSON y los retorna como slice     |
| `SaveAllUsers(archivo, users)`    | Sobrescribe el archivo JSON con la lista completa de usuarios        |

### `composables/session.go` -- Gestion de sesion

| Funcion                    | Descripcion                                                        |
|----------------------------|--------------------------------------------------------------------|
| `GuardarSession(usuario)`  | Serializa el usuario logueado y lo guarda en `session.json`        |
| `ObtenerSession()`         | Lee `session.json` y retorna el usuario de la sesion activa        |

### `composables/taskComposables.go` -- Modelo `Task` (tareas)

| Funcion / Metodo                | Descripcion                                                          |
|---------------------------------|----------------------------------------------------------------------|
| `NewTask(name, idUser, state, userName)` | Constructor: crea una nueva tarea con los datos proporcionados |
| `Task.SaveData(archivo)`        | Agrega la tarea al archivo JSON con ID formato `T-xxx` y fecha actual |
| `GetAllTasks(archivo)`          | Lee todas las tareas del archivo JSON y las retorna como slice       |
| `SaveAllTasks(archivo, tasks)`  | Sobrescribe el archivo JSON con la lista completa de tareas          |

---

## Modelo de datos

### Login (usuario)

```json
{
  "id": "1",
  "username": "admin",
  "password": "123"
}
```

### Task (tarea)

```json
{
  "id tarea": "T-001",
  "tdescricpcion de tarea": "Revisar documentacion",
  "id usuario": "2",
  "nombre del usuario": "carlos",
  "estado de la tarea": "pendiente",
  "fecha de creacion": "27-03-2026 15:30:00",
  "fecha de actualizacion": "27-03-2026 15:30:00"
}
```

---

## Diferencias con la version en Rust

| Aspecto              | Go                                      | Rust                                    |
|----------------------|-----------------------------------------|-----------------------------------------|
| Autenticacion        | Sesion persistente en archivo JSON      | Login interactivo en cada comando       |
| IDs de tarea         | Formato string `T-001`                  | Numerico `u32`                          |
| IDs de usuario       | Formato string `"1"`, `"2"`             | Numerico `u32`                          |
| Registro de usuarios | Comando `reg` (cualquier usuario)       | Solo admin via `gestionar-usuarios`     |
| Dependencias         | Solo libreria estandar                  | serde, serde_json, chrono               |
| Estados de tarea     | `pendiente`, `en-proceso`, `completo`   | `todo`, `in-progress`, `done`           |

---

## Licencia

Proyecto academico -- Universidad / Materia: Lenguajes de Programacion.

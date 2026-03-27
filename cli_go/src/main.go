package main

import (
	"fmt"
	"os"
	"strings"
	"task_tracker_cli/composables"
	"time"
)

// main es el punto de entrada principal del programa Task Tracker CLI.
// Gestiona la lógica de comandos (reg, log, add, listTask, act, del, gest)
// y coordina la persistencia de datos y la sesión de usuario.
func main() {
	var name, password, comando string
	var usuarioLogueado composables.Login
	var total, pends, procs, listas int
	var tareasAMostrar []composables.Task

	if len(os.Args) < 2 {
		fmt.Println("\nUso: go run main.go [comando]")
		fmt.Println("\n   Comando\t\tSentencia")
		fmt.Println("-> reg       | Registrar un nuevo usuario")
		fmt.Println("-> log       | Iniciar sesion con el usuario")
		fmt.Println("-> list      | Listar los usuarios registrados\n")
		return
	}
	comando = os.Args[1]

	switch comando {
	case "reg":
		mostrarInicioSesion()
		fmt.Print("- Nombre de usuario: ")
		fmt.Scanln(&name)
		fmt.Print("- Clave a asignar: ")
		fmt.Scanln(&password)
		newSession := composables.NewLogin(name, password)
		err := newSession.SaveData("storage/user.json")
		if err != nil {
			fmt.Println("Error al guardar los datos del usuario:", err)
		} else {
			fmt.Println("\nRegistro de usuario completado\n")
		}
		menuComandos()
		return
	case "log":
		mostrarInicioSesion()
		fmt.Print("\n- Usuario: ")
		fmt.Scanln(&name)
		fmt.Print("- Clave: ")
		fmt.Scanln(&password)
		encontrado := false

		usuarios, err := composables.GetAllUsers("storage/user.json")
		if err != nil {
			fmt.Println("Error al leer la base de datos de usuarios:", err)
			return
		}

		for _, u := range usuarios {
			if u.Username == name && u.Password == password {
				usuarioLogueado = u
				encontrado = true
				break
			}
		}

		if encontrado {
			err := composables.GuardarSession(usuarioLogueado)
			if err != nil {
				fmt.Println(" Error al crear sesión:", err)
			} else {
				fmt.Printf("\n-> Sesión iniciada como: %s\n", usuarioLogueado.Username)

				menuComandos()

				if usuarioLogueado.Username == "admin" {
					fmt.Println("-> gest <\"id usuario\">\t\t | Gestion de usuarios registrados: borrar, modificar\n")
				}
			}
		} else {
			fmt.Println("Usuario o contraseña incorrectos.")
		}
		return

	case "list":
		mostrarInicioSesion()
		fmt.Println("\nListado de usuarios registrados:\n")
		usuarios, err := composables.GetAllUsers("storage/user.json")
		if err != nil {
			fmt.Println("Error al obtener usuarios:", err)
			return
		}
		if len(usuarios) == 0 {
			fmt.Println("No hay usuarios registrados aún.")
		}

		fmt.Println("-------------------------------------")
		fmt.Println("   ID   | \tUSUARIO")
		fmt.Println("-------------------------------------")
		for _, u := range usuarios {
			fmt.Printf("%s\t| %s\n", u.Id, u.Username)
		}
		fmt.Println("-------------------------------------")
		menuComandos()

	case "add":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Creación de tareas")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")

		if len(os.Args) < 3 {
			fmt.Println("Uso: go run main.go add \"nombre tarea\"")
			return
		}
		userSession, err := composables.ObtenerSession()
		if err != nil {
			fmt.Println("Error: Debes iniciar sesión primero con 'log'.")
			return
		}

		taskName := os.Args[2]

		newTask := composables.NewTask(
			taskName,
			userSession.Id,
			"pendiente",
			userSession.Username,
		)

		err = newTask.SaveData("storage/task.json")
		if err != nil {
			fmt.Println("Error:", err)
		} else {
			fmt.Printf("\n Tarea '%s' creada (ID User: %s)\n", taskName, userSession.Id)
		}
		return

	case "listTask":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Listado de tareas")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")

		filtro := "todas"
		if len(os.Args) >= 3 {
			filtro = strings.ToLower(strings.TrimSpace(os.Args[2]))
		}

		tareas, err := composables.GetAllTasks("storage/task.json")
		if err != nil {
			fmt.Println("Error al leer las tareas:", err)
			return
		}

		for _, t := range tareas {
			est := strings.ToLower(strings.TrimSpace(t.State))

			if est == "pendiente" {
				pends++
			} else if est == "en-proceso" {
				procs++
			} else if est == "completo" || est == "lista" || est == "listo" {
				listas++
			}
			total++

			match := false
			if filtro == "todas" {
				match = true
			} else if (filtro == "pendiente" || filtro == "pendientes") && est == "pendiente" {
				match = true
			} else if (filtro == "en-proceso" || filtro == "proceso") && est == "en-proceso" {
				match = true
			} else if (filtro == "lista" || filtro == "listas") && (est == "completo" || est == "lista" || est == "listo") {
				match = true
			}

			if match {
				tareasAMostrar = append(tareasAMostrar, t)
			}
		}

		fmt.Printf("\nVisualizando: [%s]\n", filtro)
		fmt.Println("--------------------------------------------------------------------------------")
		fmt.Printf("%-10s | %-25s | %-15s | %-15s\n", "ID", "DESCRIPCIÓN", "ESTADO", "USUARIO")
		fmt.Println("--------------------------------------------------------------------------------")

		for _, t := range tareasAMostrar {
			fmt.Printf("%-10s | %-25s | %-15s | %-15s\n", t.IdTask, t.Name, t.State, t.UserName)
		}

		if len(tareasAMostrar) == 0 {
			fmt.Println("   (No hay tareas para mostrar con este filtro)")
		}

		fmt.Println("--------------------------------------------------------------------------------")
		fmt.Println("*-*-*-*-* ESTADÍSTICAS TOTALES: *-*-*-*-*")
		fmt.Printf("-> Total: %d \n->Pendientes: %d \n->En Proceso: %d \n->Listas: %d\n",
			total, pends, procs, listas)
		fmt.Println("--------------------------------------------------------------------------------\n")
		return

	case "act":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Actualizar tarea")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")

		if len(os.Args) < 4 {
			fmt.Println("Uso: go run main.go act <id_tarea> <nuevo_estado>")
			fmt.Println("Ejemplo: go run main.go act T-001 en-proceso")
			return
		}

		targetId := os.Args[2]
		nuevoEstado := os.Args[3]

		tareas, err := composables.GetAllTasks("storage/task.json")
		if err != nil {
			fmt.Println("Error al leer las tareas:", err)
			return
		}

		encontrado := false
		for i := range tareas {
			if tareas[i].IdTask == targetId {
				tareas[i].State = nuevoEstado
				tareas[i].Update = time.Now().Format("02-01-2006 15:04:05")
				encontrado = true
				break
			}
		}

		if !encontrado {
			fmt.Printf("No se encontró la tarea con ID: %s\n", targetId)
			return
		}

		err = composables.SaveAllTasks("storage/task.json", tareas)
		if err != nil {
			fmt.Println("Error al guardar los cambios:", err)
		} else {
			fmt.Printf("-> Tarea %s actualizada a estado: '%s'\n", targetId, nuevoEstado)
		}
		return
	case "del":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Eliminar tarea")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")

		if len(os.Args) < 3 {
			fmt.Println("Uso: go run main.go del <id_tarea>")
			return
		}
		targetId := os.Args[2]

		var confirmar string
		fmt.Printf("¿Estás seguro de que deseas borrar la tarea %s? (s/n): ", targetId)
		fmt.Scanln(&confirmar)
		if confirmar != "s" && confirmar != "S" {
			fmt.Println("Operación cancelada.")
			return
		}

		tareas, err := composables.GetAllTasks("storage/task.json")
		if err != nil {
			fmt.Println("Error al acceder a las tareas:", err)
			return
		}

		nuevaLista := []composables.Task{}
		encontrado := false

		for _, t := range tareas {
			if t.IdTask == targetId {
				encontrado = true
				continue
			}
			nuevaLista = append(nuevaLista, t)
		}

		if !encontrado {
			fmt.Printf("No se encontró ninguna tarea con el ID: %s\n", targetId)
			return
		}

		err = composables.SaveAllTasks("storage/task.json", nuevaLista)
		if err != nil {
			fmt.Println("Error al actualizar el archivo:", err)
		} else {
			fmt.Printf("Tarea %s eliminada con éxito.\n", targetId)
		}
		return

	case "gest":
		userSession, _ := composables.ObtenerSession()
		if userSession.Username != "admin" {
			fmt.Println("-> Acceso denegado. Solo el administrador puede usar 'gest'.")
			return
		}

		if len(os.Args) < 3 {
			fmt.Println("Uso: go run main.go gest <id_usuario>")
			return
		}
		targetUserId := os.Args[2]

		usuarios, _ := composables.GetAllUsers("storage/user.json")
		index := -1
		for i, u := range usuarios {
			if u.Id == targetUserId {
				index = i
				break
			}
		}

		if index == -1 {
			fmt.Printf("El usuario con ID %s no existe.\n", targetUserId)
			return
		}

		var accion string
		fmt.Printf("\nUsuario seleccionado: %s\n", usuarios[index].Username)
		fmt.Print("¿Qué acción desea realizar? (borrar / modificar): ")
		fmt.Scanln(&accion)

		switch accion {
		case "borrar":
			if targetUserId == userSession.Id {
				fmt.Println("Error: No puedes borrar tu propia cuenta de administrador.\n")
				return
			}

			var conf string
			fmt.Printf("--> ¿Desea eliminar permanentemente al usuario %s? (s/n): ", usuarios[index].Username)
			fmt.Scanln(&conf)
			if conf == "s" || conf == "S" {
				// Filtrar la lista para quitar al usuario
				nuevaLista := append(usuarios[:index], usuarios[index+1:]...)
				composables.SaveAllUsers("storage/user.json", nuevaLista)
				fmt.Println("-> Usuario eliminado correctamente.\n")
			} else {
				fmt.Println("Operación cancelada.")
			}

		case "modificar":
			var nuevoNombre, nuevaClave string
			fmt.Printf("Nuevo nombre (actual: %s, Enter para omitir): ", usuarios[index].Username)
			fmt.Scanln(&nuevoNombre)
			fmt.Printf("Nueva clave (Enter para omitir): ")
			fmt.Scanln(&nuevaClave)

			if nuevoNombre != "" {
				usuarios[index].Username = nuevoNombre
			}
			if nuevaClave != "" {
				usuarios[index].Password = nuevaClave
			}

			composables.SaveAllUsers("storage/user.json", usuarios)
			fmt.Println("-> Información de usuario actualizada.")

		default:
			fmt.Println("-> Acción no reconocida. Use 'borrar' o 'modificar'.")
		}
		return
	default:
		fmt.Println("\nNo se ha registrado un comando válido, saliendo del CLI...\n")
	}
}

// menuComandos imprime en consola el menú de ayuda con los comandos disponibles,
// sus argumentos y una breve descripción de su funcionalidad.
func menuComandos() {
	fmt.Println("\n*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	fmt.Println("\tBienvenido al Task Tracker CLI")
	fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	fmt.Println("\nUso: go run main.go [comando] <argumentos>")
	fmt.Println("\n   Comando\t\t\tSentencia")
	fmt.Println("-> add	<\"tarea\">		  | Crear nueva tarea")
	fmt.Println("-> listTask <\"filtro\"> | Listar la tareas: pendientes, en-proceso, completadas, todas")
	fmt.Println("-> act <\"id\"> <\"estado\">      | Cambio de estado de la tarea: en-proceso, pendiente, completo")
	fmt.Println("-> del <\"id\"> 		  | Borrar una tarea\n")
}

// mostrarInicioSesion imprime una cabecera decorativa en la consola
// para las secciones de inicio de sesión o registro de usuario.
func mostrarInicioSesion() {
	fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	fmt.Println("\t\t Inicio de sesión")
	fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
}

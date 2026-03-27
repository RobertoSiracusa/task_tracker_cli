package main

import (
	"fmt"
	"os"
	"task_tracker_cli/composables"
)

func main() {
	var name, password, comando string
	isAdmin := false

	fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	fmt.Println("\t\tInicio de sesión")
	fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")

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
		fmt.Print("\n- Usuario: ")
		fmt.Scanln(&name)
		fmt.Print("- Clave: ")
		fmt.Scanln(&password)
		if name == "admin" && password == "admin123" {
			isAdmin = true
			if isAdmin {
				menuComandos()
				fmt.Println("-> gest <arg>        | Gestion de usuarios registrados: borrar, modificar\n")
				return
			}
			fmt.Println("\nSesión iniciada como ADMINISTRADOR")
		} else {
			// Aquí iría tu lógica de validar usuario normal en el JSON
			fmt.Println("\nSesión iniciada como usuario estándar")
		}
		menuComandos()
		return
	case "list":
		fmt.Println("\nListado de usuarios registrados:\n")
		usuarios, err := composables.ListUsernames("storage/user.json")
		if err != nil {
			fmt.Println("Error al obtener usuarios:", err)
			return
		}

		// Si no hay usuarios
		if len(usuarios) == 0 {
			fmt.Println("No hay usuarios registrados aún.")
		}

		// Recorremos la lista de nombres y los numeramos
		for i, nombre := range usuarios {
			fmt.Printf("%d. %s\n", i+1, nombre)
		}
		fmt.Println("-------------------------------------")
	default:
		fmt.Println("\nNo se ha registrado un comando válido, saliendo del CLI...\n")
	}
}

func menuComandos() {
	var comando, id, name, state string

	fmt.Println("\n*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	fmt.Println("\tBienvenido al Task Tracker CLI")
	fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	fmt.Println("\nUso: go run main.go [comando] <argumentos>")
	fmt.Println("\n   Comando\t\t\tSentencia")
	fmt.Println("-> add	<tarea>		  | Crear nueva tarea")
	fmt.Println("-> list <filtro> 	  | Listar la tareas: pendientes, en-proceso, completadas, todas")
	fmt.Println("-> act <id> <estado> | Cambio de estado de la tarea: en-proceso, pendiente, completo")
	fmt.Println("-> del <id> 		  | Borrar una tarea")

	if len(os.Args) < 2 {
		return
	}

	comando = os.Args[1]

	switch comando {
	case "add":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Creación de tareas")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Print("\n- Nombre de la tarea: ")
		fmt.Scanln(&name)
		fmt.Print("- Indique el estado de la tarea:")
		fmt.Scanln(&state)
		newSession := composables.NewTask(name, id, state)
		err := newSession.SaveData("storage/task.json")
		if err != nil {
			fmt.Println("Error al guardar los datos del usuario:", err)
		} else {
			fmt.Println("\nRegistro de usuario completado\n")
		}
		return
	case "list":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Listado de tareas")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	case "act":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Actaulizar tareas")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	case "del":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Elminar tareas")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	case "gest":
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
		fmt.Println("\t\t Gestión de Usuarios")
		fmt.Println("*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*")
	}
}

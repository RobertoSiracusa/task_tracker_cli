package main

import (
	"fmt"
	"task_tracker_cli/src/composables"
	"task_tracker_cli/src/process"
	"task_tracker_cli/src/utils"
)

func main() {
	fmt.Println("Hasta aqui programo")
	process.ProcessFunction()
	composables.ComposableFunction()
	utils.UtilitariaFunction()
}

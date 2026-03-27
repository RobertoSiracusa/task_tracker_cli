package composables

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
	"time"
)

type Task struct {
	IdTask   string `json:"id tarea"`
	Name     string `json:"tdescricpcion de tarea"`
	IdUser   string `json:"id usuario"`
	UserName string `json:"nombre del usuario"`
	State    string `json:"estado de la tarea"`
	Create   string `json:"fecha de creación"`
	Update   string `json:"fecha de actualización"`
}

func NewTask(name, idUser, state, userName string) Task {
	return Task{
		Name:     name,
		IdUser:   idUser,
		UserName: userName,
		State:    state,
	}
}

func (t Task) SaveData(nameArchive string) error {
	var tasks []Task
	file, err := os.OpenFile(nameArchive, os.O_RDWR|os.O_CREATE, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	cont, _ := io.ReadAll(file)
	if len(cont) > 0 {
		json.Unmarshal(cont, &tasks)
	}

	// --- AUTO GENERACIÓN ---
	t.IdTask = fmt.Sprintf("T-%03d", len(tasks)+1) // ID único basado en cuenta
	fechaActual := time.Now().Format("02-01-2006 15:04:05")
	t.Create = fechaActual
	t.Update = fechaActual

	tasks = append(tasks, t)
	newData, _ := json.MarshalIndent(tasks, "", "  ")

	file.Truncate(0)
	file.Seek(0, 0)
	_, err = file.Write(newData)
	return err
}

// GetAllTasks lee el archivo JSON y retorna un slice de tareas
func GetAllTasks(nameArchive string) ([]Task, error) {
	var tasks []Task
	file, err := os.ReadFile(nameArchive)
	if err != nil {
		// Si el archivo no existe, retornamos un slice vacío sin error
		if os.IsNotExist(err) {
			return tasks, nil
		}
		return nil, err
	}

	if len(file) > 0 {
		err = json.Unmarshal(file, &tasks)
		if err != nil {
			return nil, err
		}
	}
	return tasks, nil
}

// SaveAllTasks sobrescribe el archivo JSON con la lista completa proporcionada
func SaveAllTasks(nameArchive string, tasks []Task) error {
	data, err := json.MarshalIndent(tasks, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(nameArchive, data, 0644)
}

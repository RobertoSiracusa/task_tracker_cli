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

// NewTask inicializa una nueva estructura Task con el nombre, ID de usuario,
// estado inicial y nombre del usuario proporcionado.
func NewTask(name, idUser, state, userName string) Task {
	return Task{
		Name:     name,
		IdUser:   idUser,
		UserName: userName,
		State:    state,
	}
}

// SaveData añade una nueva tarea al archivo JSON. Se encarga de generar
// automáticamente un ID único (T-xxx) y registra las fechas de creación y actualización.
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

	t.IdTask = fmt.Sprintf("T-%03d", len(tasks)+1)
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

// GetAllTasks lee el archivo JSON de tareas, deserializa su contenido a una lista
// de objetos Task y los devuelve. Si el archivo no existe, retorna una lista vacía.
func GetAllTasks(nameArchive string) ([]Task, error) {
	var tasks []Task
	file, err := os.ReadFile(nameArchive)
	if err != nil {
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

// SaveAllTasks sobrescribe el archivo JSON con la lista completa de tareas proporcionada,
// formateando el contenido de manera legible.
func SaveAllTasks(nameArchive string, tasks []Task) error {
	data, err := json.MarshalIndent(tasks, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(nameArchive, data, 0644)
}

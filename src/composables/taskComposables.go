package composables

import (
	"encoding/json"
	"io"
	"os"
)

type Task struct {
	Name  string `json:"task name"`
	Id    string `json:"id"`
	State string `json:"state"`
}

func NewTask(name string, id string, state string) Task {
	return Task{
		Name:  name,
		Id:    id,
		State: state,
	}
}

func (t Task) SaveData(nameArchive string) error {
	tasks := []Task{}
	file, err := os.OpenFile(nameArchive, os.O_RDWR|os.O_CREATE, 0644)
	if err != nil {
		return err
	}
	cont, err := io.ReadAll(file)
	if err != nil {
		return err
	}
	if len(cont) > 0 {
		errUnmarshal := json.Unmarshal(cont, &tasks)
		if errUnmarshal != nil {
			// Si el JSON estaba mal formado o era el formato viejo (objeto único),
			// reiniciamos 'tasks' como lista vacía para sobreescribir con el formato correcto.
			tasks = []Task{}
		}
	}

	tasks = append(tasks, t)
	newData, err := json.MarshalIndent(tasks, "", "  ")
	if err != nil {
		return err
	}

	// 7. Limpiamos el archivo y escribimos desde el principio
	err = file.Truncate(0) // Borra el contenido previo
	if err != nil {
		return err
	}
	_, err = file.Seek(0, 0) // Mueve el puntero al inicio
	if err != nil {
		return err
	}

	// 8. Escribimos los nuevos datos (la lista actualizada)
	_, err = file.Write(newData)
	return err
}

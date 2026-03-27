package composables

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
)

type Login struct {
	Id       string `json:"id"`
	Username string `json:"username"`
	Password string `json:"password"`
}

// NewLogin crea e inicializa una nueva instancia de la estructura Login (usuario)
// con el nombre de usuario y contraseña proporcionados.
func NewLogin(name string, pass string) Login {
	return Login{
		Username: name,
		Password: pass,
	}
}

// SaveData guarda la información de un usuario individual en un archivo JSON.
// Si el archivo ya contiene usuarios, añade el nuevo usuario a la lista existente
// y le asigna un ID incremental automático antes de reescribir todo el archivo.
func (l Login) SaveData(nameArchive string) error {
	users := []Login{}
	file, err := os.OpenFile(nameArchive, os.O_RDWR|os.O_CREATE, 0644)
	if err != nil {
		return err
	}
	cont, err := io.ReadAll(file)
	if err != nil {
		return err
	}
	if len(cont) > 0 {
		errUnmarshal := json.Unmarshal(cont, &users)
		if errUnmarshal != nil {
			users = []Login{}
		}
	}

	l.Id = fmt.Sprintf("%d", len(users)+1)

	users = append(users, l)
	newData, err := json.MarshalIndent(users, "", "  ")
	if err != nil {
		return err
	}

	err = file.Truncate(0)
	if err != nil {
		return err
	}
	_, err = file.Seek(0, 0) 
	if err != nil {
		return err
	}

	_, err = file.Write(newData)
	return err
}

// GetAllUsers lee el archivo JSON especificado, deserializa su contenido a una lista
// de objetos Login y los retorna. Retorna un error si no se puede leer o parsear.
func GetAllUsers(nameArchive string) ([]Login, error) {
	var allUsers []Login

	content, err := os.ReadFile(nameArchive)
	if err != nil {
		return nil, err
	}

	if err := json.Unmarshal(content, &allUsers); err != nil {
		return nil, err
	}

	return allUsers, nil
}
// SaveAllUsers toma una lista completa de usuarios y sobrescribe el archivo JSON
// especificado con estos nuevos datos en un formato legible (identado).
func SaveAllUsers(nameArchive string, users []Login) error {
	data, err := json.MarshalIndent(users, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(nameArchive, data, 0644)
}

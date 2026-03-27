package composables

import (
	"encoding/json"
	"io"
	"os"
)

type Login struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

func NewLogin(name string, pass string) Login {
	return Login{
		Username: name,
		Password: pass,
	}
}

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
			// Si el JSON estaba mal formado o era el formato viejo (objeto único),
			// reiniciamos 'users' como lista vacía para sobreescribir con el formato correcto.
			users = []Login{}
		}
	}

	users = append(users, l)
	newData, err := json.MarshalIndent(users, "", "  ")
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

func ListUsernames(nameArchive string) ([]string, error) {
	var allUsers []Login // Lista para decodificar todo el JSON
	var names []string   // Lista para guardar solo los nombres

	content, err := os.ReadFile(nameArchive)
	if err != nil {
		return nil, err
	}

	// Decodificamos el contenido del archivo en allUsers
	if err := json.Unmarshal(content, &allUsers); err != nil {
		return nil, err
	}

	// Llenamos la lista de nombres
	for _, u := range allUsers {
		if u.Username != "" { // Evita agregar nombres vacíos
			names = append(names, u.Username)
		}
	}

	return names, nil
}

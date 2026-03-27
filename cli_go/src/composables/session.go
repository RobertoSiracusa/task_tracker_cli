package composables

import (
	"encoding/json"
	"os"
)

// GuardarSession serializa los datos del objeto Login (usuario) a formato JSON
// y los guarda en el archivo 'storage/session.json' para mantener la sesión activa.
func GuardarSession(u Login) error {
	data, err := json.Marshal(u)
	if err != nil {
		return err
	}
	return os.WriteFile("storage/session.json", data, 0644)
}

// ObtenerSession lee el archivo 'storage/session.json', deserializa el contenido
// y retorna un objeto de tipo Login con los datos del usuario que ha iniciado sesión.
func ObtenerSession() (Login, error) {
	var u Login
	data, err := os.ReadFile("storage/session.json")
	if err != nil {
		return u, err
	}
	err = json.Unmarshal(data, &u)
	return u, err
}

package composables

import (
	"encoding/json"
	"os"
)

// GuardarSession escribe los datos del usuario actual
func GuardarSession(u Login) error {
	data, err := json.Marshal(u)
	if err != nil {
		return err
	}
	return os.WriteFile("storage/session.json", data, 0644)
}

// ObtenerSession lee quién está logueado
func ObtenerSession() (Login, error) {
	var u Login
	data, err := os.ReadFile("storage/session.json")
	if err != nil {
		return u, err
	}
	err = json.Unmarshal(data, &u)
	return u, err
}

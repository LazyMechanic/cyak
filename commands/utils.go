package commands

import (
	"os"
	"path/filepath"
)

func mkdirIfNotExist(path string, joins ...string) error {
	finalPath := path
	for i, _ := range joins {
		finalPath = filepath.Join(finalPath, joins[i])
	}

	if _, err := os.Stat(finalPath); os.IsNotExist(err) {
		return os.MkdirAll(finalPath, os.ModePerm)
	}
	return nil
}
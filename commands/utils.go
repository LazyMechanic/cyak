package commands

import (
	"os"
	"path/filepath"
)

func isDirExist(path string, joins ...string) bool {
	finalPath := path
	for i, _ := range joins {
		finalPath = filepath.Join(finalPath, joins[i])
	}

	_, err := os.Stat(finalPath)
	return !os.IsNotExist(err)
}

func mkdirIfNotExist(path string, joins ...string) error {
	finalPath := path
	for i, _ := range joins {
		finalPath = filepath.Join(finalPath, joins[i])
	}

	if !isDirExist(finalPath) {
		return os.MkdirAll(finalPath, os.ModePerm)
	}
	return nil
}
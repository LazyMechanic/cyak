package config

import (
	"log"
	"os"
	"path/filepath"
)

func binDir() string {
	binDir, err := filepath.Abs(filepath.Dir(os.Args[0]))
	if err != nil {
		log.Fatal(err)
	}
	return binDir
}

var (
	PresetsFolder = filepath.Join(binDir(), "../", "presets")
)

const (
	ExecutableTemplate   string = "executable.template"
	InterfaceTemplate    string = "interface.template"
	LibraryTemplate      string = "library.template"
	ProjectTemplate      string = "project.template"
	TargetConfigTemplate string = "config.template"
	TestTemplate         string = "test.template"
	TemplatesFolder      string = "templates"
	AsIsFolder           string = "asis"
	Version                     = "0.1"
)

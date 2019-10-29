package config

import (
	"log"
	"os/user"
)

func userDir() string {
	usr, err := user.Current()
	if err != nil {
		log.Fatal(err)
	}
	return usr.HomeDir
}

var (
	PresetsFolder = "./presets" //filepath.Join(userDir(), ".cyak", "presets")
)

const (
	ExecutableTemplate   string = "executable.template"
	InterfaceTemplate    string = "interface.template"
	LibraryTemplate      string = "library.template"
	ProjectTemplate      string = "project.template"
	TargetConfigTemplate string = "config.template"
	Version                     = "0.1"
)

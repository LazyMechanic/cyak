package template

import (
	"bytes"
	"fmt"
	"github.com/LazyMechanic/cyak/internal/config"
	"github.com/LazyMechanic/cyak/internal/types"
	"io/ioutil"
	"text/template"
)

const (
	Executable types.TargetType = "executable"
	Library    types.TargetType = "library"
	Interface  types.TargetType = "interface"
)

func GetTargetTemplateFileName(targetType types.TargetType) string {
	switch targetType {
	case Executable:
		return config.ExecutableTemplate
	case Library:
		return config.LibraryTemplate
	case Interface:
		return config.InterfaceTemplate
	default:
		panic(fmt.Errorf("Invalid target type"))
	}
}

func applyTarget(content string, targetConfig *types.TargetConfig) string {
	var tmpl = template.Must(template.New(targetConfig.Name).Parse(content))

	var tmplBuffer bytes.Buffer
	var err = tmpl.Execute(&tmplBuffer, *targetConfig)
	if err != nil {
		panic(err)
	}

	return tmplBuffer.String()
}

func applyProject(content string, projectConfig *types.ProjectConfig) string {
	var tmpl = template.Must(template.New(projectConfig.Name).Parse(content))

	var tmplBuffer bytes.Buffer
	var err = tmpl.Execute(&tmplBuffer, *projectConfig)
	if err != nil {
		panic(err)
	}

	return tmplBuffer.String()
}

func TargetFile(file string, targetConfig *types.TargetConfig) string {
	fileBytes, err := ioutil.ReadFile(file)
	if err != nil {
		panic(err)
	}

	fileContent := string(fileBytes)
	return applyTarget(fileContent, targetConfig)
}

func ProjectFile(file string, projectConfig *types.ProjectConfig) string {
	fileBytes, err := ioutil.ReadFile(file)
	if err != nil {
		panic(err)
	}

	fileContent := string(fileBytes)
	return applyProject(fileContent, projectConfig)
}

package template

import (
	"bytes"
	"fmt"
	"github.com/LazyMechanic/cyak/internal/config"
	"github.com/LazyMechanic/cyak/internal/types"
	"io/ioutil"
	"os"
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

func applyTarget(str string, targetConfig *types.TargetConfig) string {
	content := str

	tmpl, err := template.New(targetConfig.Name).Parse(content)
	if err != nil {
		panic(err)
	}

	var tmplBuffer bytes.Buffer
	err = tmpl.Execute(&tmplBuffer, *targetConfig)

	return tmplBuffer.String()
}

func applyProject(str string, projectConfig *types.ProjectConfig) string {
	content := str

	tmpl, err := template.New(projectConfig.Name).Parse(content)
	if err != nil {
		panic(err)
	}

	var tmplBuffer bytes.Buffer
	err = tmpl.Execute(&tmplBuffer, *projectConfig)

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

func WriteToDisk(file string, content string) {
	if err := ioutil.WriteFile(file, []byte(content), os.ModePerm); err != nil {
		panic(err)
	}
}

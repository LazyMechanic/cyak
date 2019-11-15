package preset

import (
	"fmt"
	"github.com/LazyMechanic/cyak/internal/cli/flags"
	"io/ioutil"
	"os"
	"path/filepath"
	"strings"
)

func IsPresetValid(preset string) error {
	var errors []string

	var presetDir = filepath.Join(flags.PresetFlagValue, preset)
	var templatesDir = filepath.Join(presetDir, "templates")
	var configFile = filepath.Join(templatesDir, "config.template")
	var executableFile = filepath.Join(templatesDir, "executable.template")
	var interfaceFile = filepath.Join(templatesDir, "interface.template")
	var libraryFile = filepath.Join(templatesDir, "library.template")
	var projectFile = filepath.Join(templatesDir, "project.template")
	var testFile = filepath.Join(templatesDir, "test.template")

	if _, err := os.Stat(templatesDir); os.IsNotExist(err) {
		errors = append(errors, fmt.Sprintf("%q missing", templatesDir))
	}

	if _, err := os.Stat(configFile); os.IsNotExist(err) {
		errors = append(errors, fmt.Sprintf("%q missing", configFile))
	}

	if _, err := os.Stat(executableFile); os.IsNotExist(err) {
		errors = append(errors, fmt.Sprintf("%q missing", executableFile))
	}

	if _, err := os.Stat(interfaceFile); os.IsNotExist(err) {
		errors = append(errors, fmt.Sprintf("%q missing", interfaceFile))
	}

	if _, err := os.Stat(libraryFile); os.IsNotExist(err) {
		errors = append(errors, fmt.Sprintf("%q missing", libraryFile))
	}

	if _, err := os.Stat(projectFile); os.IsNotExist(err) {
		errors = append(errors, fmt.Sprintf("%q missing", projectFile))
	}

	if _, err := os.Stat(testFile); os.IsNotExist(err) {
		errors = append(errors, fmt.Sprintf("%q missing", testFile))
	}

	if len(errors) == 0 {
		return nil
	}

	return fmt.Errorf("Preset %q invalid:\n%s", presetDir, strings.Join(errors, "\n"))
}

func IsPresetsValid(presets []string) error {
	var errors []string

	for _, preset := range presets {
		err := IsPresetValid(preset)
		if err != nil {
			errors = append(errors, err.Error())
		}
	}

	if len(errors) == 0 {
		return nil
	}

	return fmt.Errorf("%s", strings.Join(errors, "\n"))
}

func PresetsNames() []string {
	presetsDirs, err := ioutil.ReadDir(flags.PresetFlagValue)
	if err != nil {
		panic(err)
	}

	var presets []string
	for _, preset := range presetsDirs {
		if preset.IsDir() {
			presets = append(presets, preset.Name())
		}
	}

	if len(presets) == 0 {
		panic(fmt.Errorf("In presets directory not found presets"))
	}

	return presets
}

func Exist(name string) bool {
	_, err := os.Stat(filepath.Join(flags.PresetFlagValue, name))
	return !os.IsNotExist(err)
}

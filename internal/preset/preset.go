package preset

import (
	"fmt"
	"github.com/LazyMechanic/cyak/internal/config"
	"io/ioutil"
	"os"
	"path/filepath"
)

func PresetsNames() []string {
	presetFiles, err := ioutil.ReadDir(config.PresetsFolder)
	if err != nil {
		panic(err)
	}

	if len(presetFiles) == 0 {
		panic(fmt.Errorf("Presets not found"))
	}

	var presets []string
	for _, preset := range presetFiles {
		presets = append(presets, preset.Name())
	}

	return presets
}

func Exist(name string) bool {
	_, err := os.Stat(filepath.Join(config.PresetsFolder, name))
	return !os.IsNotExist(err)
}

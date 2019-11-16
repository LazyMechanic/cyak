package flags

import (
	gocli "github.com/urfave/cli"
	"os"
	"path/filepath"
)

var (
	PresetFlagValue string
	GitFlagValue    bool
)

func binDir() string {
	binDir, err := os.Executable()
	if err != nil {
		panic(err)
	}

	return filepath.Dir(binDir)
}

func presetDefaultDir() string {
	abs, err := filepath.Abs(filepath.Join(binDir(), "..", "share", "cyak", "presets"))
	if err != nil {
		panic(err)
	}
	
	return abs
}

var (
	PresetFlag gocli.Flag = &gocli.StringFlag{
		Name:        "p, preset",
		Usage:       "presets folder",
		Destination: &PresetFlagValue,
		Value:       presetDefaultDir(),
	}

	GitFlag gocli.Flag = &gocli.BoolFlag{
		Name:        "g, git",
		Usage:       "force git initialization",
		Destination: &GitFlagValue,
	}
)

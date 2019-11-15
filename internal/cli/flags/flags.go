package flags

import (
	gocli "github.com/urfave/cli"
	"log"
	"os"
	"path/filepath"
)

var (
	PresetFlagValue string
	GitFlagValue    bool
)

func binDir() string {
	binDir, err := filepath.Abs(filepath.Dir(os.Args[0]))
	if err != nil {
		log.Fatal(err)
	}
	return binDir
}

var (
	PresetFlag gocli.Flag = &gocli.StringFlag{
		Name:        "p, preset",
		Usage:       "presets folder",
		Destination: &PresetFlagValue,
		Value:       filepath.Join(binDir(), "../", "presets"),
	}

	GitFlag gocli.Flag = &gocli.BoolFlag{
		Name:        "g, git",
		Usage:       "force git initialization",
		Destination: &GitFlagValue,
	}
)

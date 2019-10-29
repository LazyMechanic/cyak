package flags

import (
	gocli "github.com/urfave/cli"
)

var (
	DefaultFlagValue    bool
	PresetFlagValue     string
	ForceFlagValue      bool
	GitFlagValue        string
	ExecutableFlagValue bool
	LibraryFlagValue    bool
	InterfaceFlagValue  bool
	ProjectFlagValue    bool
)

var (
	DefaultFlag = gocli.BoolFlag{
		Name:        "d, default",
		Usage:       "skip prompts and use default preset",
		Destination: &DefaultFlagValue,
	}

	PresetFlag = gocli.StringFlag{
		Name:        "p, preset",
		Usage:       "skip prompts and use saved preset `name`",
		Destination: &PresetFlagValue,
	}

	ForceFlag = gocli.BoolFlag{
		Name:        "f, force",
		Usage:       "overwrite target directory if it exists, for new project",
		Destination: &ForceFlagValue,
	}

	GitFlag = gocli.StringFlag{
		Name:        "g, git",
		Usage:       "force git initialization with initial commit `message`",
		Destination: &GitFlagValue,
	}

	ExecutableFlag = gocli.BoolFlag{
		Name:        "e, executable",
		Usage:       "create executable cmake list only, overrides exist file",
		Destination: &ExecutableFlagValue,
	}

	LibraryFlag = gocli.BoolFlag{
		Name:        "l, library",
		Usage:       "create library cmake list only, overrides exist file",
		Destination: &LibraryFlagValue,
	}

	InterfaceFlag = gocli.BoolFlag{
		Name:        "i, interface",
		Usage:       "create interface cmake list only, overrides exist file",
		Destination: &InterfaceFlagValue,
	}

	ProjectFlag = gocli.BoolFlag{
		Name:        "P, project",
		Usage:       "create project cmake list only, overrides exist file",
		Destination: &ProjectFlagValue,
	}
)

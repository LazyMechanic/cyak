package flags

import (
	gocli "github.com/urfave/cli"
)

var (
	DefaultFlagValue    bool
	PresetFlagValue     string
	GitFlagValue        bool
	ExecutableFlagValue bool
	LibraryFlagValue    bool
	InterfaceFlagValue  bool
	ProjectFlagValue    bool
)

var (
	DefaultFlag gocli.Flag = &gocli.BoolFlag{
		Name:        "d, default",
		Usage:       "skip prompts and use default preset",
		Destination: &DefaultFlagValue,
	}

	PresetFlag gocli.Flag = &gocli.StringFlag{
		Name:        "p, preset",
		Usage:       "skip prompts and use saved preset `name`",
		Destination: &PresetFlagValue,
	}

	GitFlag gocli.Flag = &gocli.BoolFlag{
		Name:        "g, git",
		Usage:       "force git initialization",
		Destination: &GitFlagValue,
	}

	ExecutableFlag gocli.Flag = &gocli.BoolFlag{
		Name:        "e, executable",
		Usage:       "create executable cmake list only, overrides exist file",
		Destination: &ExecutableFlagValue,
	}

	LibraryFlag gocli.Flag = &gocli.BoolFlag{
		Name:        "l, library",
		Usage:       "create library cmake list only, overrides exist file",
		Destination: &LibraryFlagValue,
	}

	InterfaceFlag gocli.Flag = &gocli.BoolFlag{
		Name:        "i, interface",
		Usage:       "create interface cmake list only, overrides exist file",
		Destination: &InterfaceFlagValue,
	}

	ProjectFlag gocli.Flag = &gocli.BoolFlag{
		Name:        "P, project",
		Usage:       "create project cmake list only, overrides exist file",
		Destination: &ProjectFlagValue,
	}
)

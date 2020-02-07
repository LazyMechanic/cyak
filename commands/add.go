package commands

import "github.com/urfave/cli/v2"

var (
	Add = &cli.Command{
		Name:                   "add",
		Aliases:                []string{"a"},
		Usage:                  "Add target to existed project",
		ArgsUsage:              "[project_folder]",
		Action:                 nil,
		Before:                 nil,
		Flags:                  nil,
		SkipFlagParsing:        false,
		HideHelp:               false,
		Hidden:                 false,
		UseShortOptionHandling: true,
	}
)

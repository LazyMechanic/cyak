package commands

import "github.com/urfave/cli/v2"

var (
	Create = &cli.Command{
		Name:                   "create",
		Aliases:                []string{"c"},
		Usage:                  "Create new cmake project",
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

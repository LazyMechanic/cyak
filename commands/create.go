package commands

import "github.com/urfave/cli/v2"

var (
	Create = &cli.Command{
		Name:                   "create",
		Aliases:                []string{"c"},
		Usage:                  "Create new cmake project",
		ArgsUsage:              "[project_folder]",
		Before:                 createBefore,
		Action:                 createAction,
		After:                  createAfter,
		Flags:                  nil,
		SkipFlagParsing:        false,
		HideHelp:               false,
		Hidden:                 false,
		UseShortOptionHandling: true,
	}
)

func createBefore(c *cli.Context) error {
	return nil
}

func createAction(c *cli.Context) error {
	return nil
}

func createAfter(c *cli.Context) error {
	return nil
}

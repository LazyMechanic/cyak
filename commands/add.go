package commands

import "github.com/urfave/cli/v2"

type Add struct {
	command *cli.Command
}

func NewAddCommand() Command {
	add := &Add{
		command: &cli.Command{
			Name:                   "add",
			Aliases:                []string{"a"},
			Usage:                  "Add target to existed project",
			ArgsUsage:              "[project_folder]",
			Flags:                  nil,
			SkipFlagParsing:        false,
			HideHelp:               false,
			Hidden:                 false,
			UseShortOptionHandling: true,
		},
	}

	add.command.Before = add.before
	add.command.Action = add.action
	add.command.After = add.after

	return add
}

func (c *Add) CliCommand() *cli.Command {
	return c.command
}

func (c *Add) before(ctx *cli.Context) error { return nil }
func (c *Add) action(ctx *cli.Context) error { return nil }
func (c *Add) after(ctx *cli.Context) error  { return nil }

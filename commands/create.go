package commands

import "github.com/urfave/cli/v2"

type Create struct {
	command *cli.Command
}

func NewCreateCommand() Command {
	create := &Create{
		command: &cli.Command{
			Name:                   "create",
			Aliases:                []string{"c"},
			Usage:                  "Create new cmake project",
			ArgsUsage:              "[project_folder]",
			Flags:                  nil,
			SkipFlagParsing:        false,
			HideHelp:               false,
			Hidden:                 false,
			UseShortOptionHandling: true,
		},
	}

	create.command.Before = create.before
	create.command.Action = create.action
	create.command.After = create.after

	return create
}

func (c *Create) CliCommand() *cli.Command {
	return c.command
}

func (c *Create) before(ctx *cli.Context) error { return nil }
func (c *Create) action(ctx *cli.Context) error { return nil }
func (c *Create) after(ctx *cli.Context) error  { return nil }

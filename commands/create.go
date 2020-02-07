package commands

import (
	"errors"
	"fmt"
	"github.com/urfave/cli/v2"
)

type Create struct {
	command    *cli.Command
	projectDir string
}

func NewCreateCommand() *Create {
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

func (c *Create) before(ctx *cli.Context) error {
	if ctx.NArg() != 1 {
		return errors.New("invalid argument count")
	}

	c.projectDir = ctx.Args().Get(0)
	if isDirExist(c.projectDir) {
		fmt.Println("Project directory already exist")
	} else {
		fmt.Println("Project directory not exist")
	}

	return nil
}

func (c *Create) action(ctx *cli.Context) error { return nil }
func (c *Create) after(ctx *cli.Context) error  { return nil }

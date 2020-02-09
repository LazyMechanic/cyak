package commands

import (
	"errors"
	"github.com/LazyMechanic/cyak/targets"
	"github.com/urfave/cli/v2"
)

type Create struct {
	command   *cli.Command
	isRunning bool

	project              *targets.Project
	projectDir           string
	needRemoveProjectDir bool
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
		project:                   targets.NewProject(),
		isRunning:                 true,
		projectDir:                "",
		needRemoveProjectDir:      false,
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
		return errors.New("Invalid argument count")
	}

	c.projectDir = ctx.Args().Get(0)
	if isDirExist(c.projectDir) {
		err := c.qProjectDirAlreadyExist()
		if err != nil {
			return err
		}
	}

	return nil
}

func (c *Create) action(ctx *cli.Context) error {
	err := c.qCreateProject()
	if err != nil {
		return err
	}

	for c.isRunning {
		err := c.qMainMenu()
		if err != nil {
			return err
		}
	}
	return nil
}

func (c *Create) after(ctx *cli.Context) error { return nil }

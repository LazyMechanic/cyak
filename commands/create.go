package commands

import (
	"errors"
	"github.com/AlecAivazis/survey/v2"
	"github.com/urfave/cli/v2"
)

type Create struct {
	command *cli.Command

	projectDir           string
	needRemoveProjectDir bool

	execInheritedQuestions    []*survey.Question
	execNonInheritedQuestions []*survey.Question

	libInheritedQuestions    []*survey.Question
	libNonInheritedQuestions []*survey.Question

	intInheritedQuestions    []*survey.Question
	intNonInheritedQuestions []*survey.Question
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
		projectDir:                "",
		needRemoveProjectDir:      false,
		execInheritedQuestions:    newExecInheritedQuestions(),
		execNonInheritedQuestions: newExecNonInheritedQuestions(),
		libInheritedQuestions:     newLibInheritedQuestions(),
		libNonInheritedQuestions:  newLibNonInheritedQuestions(),
		intInheritedQuestions:     newIntInheritedQuestions(),
		intNonInheritedQuestions:  newIntNonInheritedQuestions(),
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
		action, err := qProjectDirAlreadyExist()
		if err != nil {
			return err
		}

		switch action {
		case "Merge":
			c.needRemoveProjectDir = false
		case "Overwrite":
			c.needRemoveProjectDir = true
		case "Cancel":
			return nil
		}
	}

	return nil
}

func (c *Create) action(ctx *cli.Context) error { return nil }
func (c *Create) after(ctx *cli.Context) error  { return nil }

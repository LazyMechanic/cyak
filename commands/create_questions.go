package commands

import (
	"github.com/AlecAivazis/survey/v2"
	"github.com/LazyMechanic/cyak/targets"
	"github.com/urfave/cli/v2"
)

func (c *Create) qProjectDirAlreadyExist() error {
	actions := []*action{
		{
			Option: "Merge",
			Func: func() error {
				c.needRemoveProjectDir = false
				return nil
			},
		},
		{
			Option: "Clear directory",
			Func: func() error {
				c.needRemoveProjectDir = true
				return nil
			},
		},
		{
			Option: "Cancel",
			Func: func() error {
				return cli.NewExitError("Nothing done", 0)
			},
		},
	}

	return qSelect(
		actions,
		actions[0],
		"Project directory already exist:",
		"Merge - append files and overwrite if conflict\nOverwrite - clear directory and create project\nCancel - exit")
}

func (c *Create) qCreateProject() error {
	return survey.Ask(c.project.Questions(), c.project)
}

func (c *Create) qMainMenu() error {
	actions := []*action{
		{
			Option: "Add",
			Func: func() error {
				err := c.qAdd()
				switch err.(type) {
				case *cancelError:
					return nil
				default:
					return err
				}
			},
		},
		{
			Option: "Remove",
			Func: func() error {
				return nil
			},
		},
		{
			Option: "Show",
			Func: func() error {
				return nil
			},
		},
		{
			Option: "Save",
			Func: func() error {
				return nil
			},
		},
		{
			Option: "Cancel",
			Func: func() error {
				return cli.NewExitError("Nothing done", 0)
			},
		},
	}

	return qSelect(
		actions,
		nil,
		"Select action:",
		"")
}

func (c *Create) qAdd() error {
	qInheritedFunc := func() (bool, error) {
		prompt := &survey.Confirm{
			Message: "Inherit project properties:",
			Default: true,
			Help:    "Properties: namespace, lang standard, lang extensions, lang standard required, version",
		}

		var answer bool
		err := survey.AskOne(prompt, &answer)
		return answer, err
	}

	qIsInformationCorrect := func() (bool, error) {
		prompt := &survey.Confirm{
			Message: "Is information correct:",
			Default: true,
			Help:    "If true, then adds to project, else discard target",
		}

		var answer bool
		err := survey.AskOne(prompt, &answer)
		return answer, err
	}

	actions := []*action{
		{
			Option: "Executable",
			Func: func() error {
				inherits, err := qInheritedFunc()
				if err != nil {
					return err
				}

				// Create target
				target := targets.NewExecutable()
				target.IsInherited = inherits

				if !target.IsInherited {
					err := survey.Ask(target.InheritedQuestions(), target)
					if err != nil {
						return err
					}
				} else {
					target.Inherit(c.project)
				}

				err = survey.Ask(target.NonInheritedQuestions(), target)
				if err != nil {
					return err
				}

				// Discard target
				isCorrect, err := qIsInformationCorrect()
				if err != nil {
					return err
				}

				if !isCorrect {
					return nil
				}

				// Add target and test
				err = c.project.AddTarget(target)
				if err != nil {
					return err
				}

				if target.HasTest {
					c.project.AddTest(target.CreateTestTarget())
				}

				return nil
			},
		},
		{
			Option: "Library",
			Func: func() error {
				inherits, err := qInheritedFunc()
				if err != nil {
					return err
				}

				// Create target
				target := targets.NewLibrary()
				target.IsInherited = inherits

				if !target.IsInherited {
					err := survey.Ask(target.InheritedQuestions(), target)
					if err != nil {
						return err
					}
				} else {
					target.Inherit(c.project)
				}

				err = survey.Ask(target.NonInheritedQuestions(), target)
				if err != nil {
					return err
				}

				// Discard target
				isCorrect, err := qIsInformationCorrect()
				if err != nil {
					return err
				}

				if !isCorrect {
					return nil
				}

				// Add target and test
				err = c.project.AddTarget(target)
				if err != nil {
					return err
				}

				if target.HasTest {
					c.project.AddTest(target.CreateTestTarget())
				}

				return nil
			},
		},
		{
			Option: "Interface",
			Func: func() error {
				inherits, err := qInheritedFunc()
				if err != nil {
					return err
				}

				// Create target
				target := targets.NewInterface()
				target.IsInherited = inherits

				if !target.IsInherited {
					err := survey.Ask(target.InheritedQuestions(), target)
					if err != nil {
						return err
					}
				} else {
					target.Inherit(c.project)
				}

				err = survey.Ask(target.NonInheritedQuestions(), target)
				if err != nil {
					return err
				}

				// Discard target
				isCorrect, err := qIsInformationCorrect()
				if err != nil {
					return err
				}

				if !isCorrect {
					return nil
				}

				// Add target and test
				err = c.project.AddTarget(target)
				if err != nil {
					return err
				}

				if target.HasTest {
					c.project.AddTest(target.CreateTestTarget())
				}

				return nil
			},
		},
		{
			Option: "Cancel",
			Func: func() error {
				return newCancelError()
			},
		},
	}

	return qSelect(
		actions,
		nil,
		"Select target for adds to project:",
		"")
}

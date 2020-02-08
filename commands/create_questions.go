package commands

import (
	"errors"
	"github.com/AlecAivazis/survey/v2"
)

func newExecInheritedQuestions() []*survey.Question {
	return []*survey.Question{
		{
			Name: "standard",
			Prompt: &survey.Input{
				Message: "Enter standard:",
				Default: "17",
			},
			Validate: isInt,
		},
		{
			Name: "extensions",
			Prompt: &survey.Select{
				Message: "Enable extensions:",
				Options: []string{
					"On",
					"Off",
				},
				Default: "Off",
			},
		},
		{
			Name: "standardrequired",
			Prompt: &survey.Select{
				Message: "Enable standard required:",
				Options: []string{
					"Yes",
					"Off",
				},
				Default: "Yes",
			},
		},
		{
			Name: "major",
			Prompt: &survey.Input{
				Message: "Enter major version:",
				Default: "0",
			},
			Validate: isInt,
		},
		{
			Name: "minor",
			Prompt: &survey.Input{
				Message: "Enter minor version:",
				Default: "1",
			},
			Validate: isInt,
		},
		{
			Name: "patch",
			Prompt: &survey.Input{
				Message: "Enter patch version:",
				Default: "0",
			},
			Validate: isInt,
		},
	}
}

func newExecNonInheritedQuestions() []*survey.Question {
	return []*survey.Question{
		{
			Name: "name",
			Prompt: &survey.Input{
				Message: "Enter target name:",
			},
			Validate: survey.Required,
		},
	}
}

func newLibInheritedQuestions() []*survey.Question {
	return []*survey.Question{
		{
			Name: "namespace",
			Prompt: &survey.Input{
				Message: "Enter namespace:",
			},
			Validate: survey.Required,
		},
		{
			Name: "standard",
			Prompt: &survey.Input{
				Message: "Enter standard:",
				Default: "17",
			},
			Validate: isInt,
		},
		{
			Name: "extensions",
			Prompt: &survey.Select{
				Message: "Enable extensions:",
				Options: []string{
					"On",
					"Off",
				},
				Default: "Off",
			},
		},
		{
			Name: "standardrequired",
			Prompt: &survey.Select{
				Message: "Enable standard required:",
				Options: []string{
					"Yes",
					"Off",
				},
				Default: "Yes",
			},
		},
		{
			Name: "major",
			Prompt: &survey.Input{
				Message: "Enter major version:",
				Default: "0",
			},
			Validate: isInt,
		},
		{
			Name: "minor",
			Prompt: &survey.Input{
				Message: "Enter minor version:",
				Default: "1",
			},
			Validate: isInt,
		},
		{
			Name: "patch",
			Prompt: &survey.Input{
				Message: "Enter patch version:",
				Default: "0",
			},
			Validate: isInt,
		},
	}
}

func newLibNonInheritedQuestions() []*survey.Question {
	return []*survey.Question{
		{
			Name: "name",
			Prompt: &survey.Input{
				Message: "Enter target name:",
			},
			Validate: survey.Required,
		},
	}
}

func newIntInheritedQuestions() []*survey.Question {
	return []*survey.Question{
		{
			Name: "namespace",
			Prompt: &survey.Input{
				Message: "Enter namespace:",
			},
			Validate: survey.Required,
		},
		{
			Name: "standard",
			Prompt: &survey.Input{
				Message: "Enter standard:",
				Default: "17",
			},
			Validate: isInt,
		},
		{
			Name: "extensions",
			Prompt: &survey.Select{
				Message: "Enable extensions:",
				Options: []string{
					"On",
					"Off",
				},
				Default: "Off",
			},
		},
		{
			Name: "standardrequired",
			Prompt: &survey.Select{
				Message: "Enable standard required:",
				Options: []string{
					"Yes",
					"Off",
				},
				Default: "Yes",
			},
		},
		{
			Name: "major",
			Prompt: &survey.Input{
				Message: "Enter major version:",
				Default: "0",
			},
			Validate: isInt,
		},
		{
			Name: "minor",
			Prompt: &survey.Input{
				Message: "Enter minor version:",
				Default: "1",
			},
			Validate: isInt,
		},
		{
			Name: "patch",
			Prompt: &survey.Input{
				Message: "Enter patch version:",
				Default: "0",
			},
			Validate: isInt,
		},
	}
}

func newIntNonInheritedQuestions() []*survey.Question {
	return []*survey.Question{
		{
			Name: "name",
			Prompt: &survey.Input{
				Message: "Enter target name:",
			},
			Validate: survey.Required,
		},
	}
}

func qSelect(actions []*action, def *action, msg string, help string) error {
	var options []string
	for i, _ := range actions {
		options = append(options, actions[i].Option)
	}

	prompt := &survey.Select{
		Message: msg,
		Options: options,
		Default: def,
		Help:    help,
	}

	var answer string
	err := survey.AskOne(prompt, &answer)
	if err != nil {
		return err
	}

	for i, _ := range actions {
		if actions[i].Option == answer {
			return actions[i].Func()
		}
	}

	return errors.New("Action function not found")
}

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
				return &exit{"Nothing done"}
			},
		},
	}

	return qSelect(
		actions,
		actions[0],
		"Project directory already exist:",
		"Merge - append files and overwrite if conflict\nOverwrite - clear directory and create project\nCancel - exit")
}

func (c *Create) qMainMenu() error {
	actions := []*action{
		{
			Option: "Add",
			Func: func() error {
				c.needRemoveProjectDir = false
				return nil
			},
		},
		{
			Option: "Remove",
			Func: func() error {
				c.needRemoveProjectDir = true
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
				return &exit{"Nothing done"}
			},
		},
	}

	return qSelect(
		actions,
		nil,
		"Select action:",
		"")
}

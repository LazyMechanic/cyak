package commands

import (
	"errors"
	"github.com/AlecAivazis/survey/v2"
)

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
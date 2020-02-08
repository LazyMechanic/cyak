package commands

import "github.com/AlecAivazis/survey/v2"

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
				Message:       "Enter major version:",
				Default:       "0",
			},
			Validate: isInt,
		},
		{
			Name:     "minor",
			Prompt: &survey.Input{
				Message:       "Enter minor version:",
				Default:       "1",
			},
			Validate: isInt,
		},
		{
			Name:     "patch",
			Prompt: &survey.Input{
				Message:       "Enter patch version:",
				Default:       "0",
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
				Message:       "Enter major version:",
				Default:       "0",
			},
			Validate: isInt,
		},
		{
			Name:     "minor",
			Prompt: &survey.Input{
				Message:       "Enter minor version:",
				Default:       "1",
			},
			Validate: isInt,
		},
		{
			Name:     "patch",
			Prompt: &survey.Input{
				Message:       "Enter patch version:",
				Default:       "0",
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
				Message:       "Enter major version:",
				Default:       "0",
			},
			Validate: isInt,
		},
		{
			Name:     "minor",
			Prompt: &survey.Input{
				Message:       "Enter minor version:",
				Default:       "1",
			},
			Validate: isInt,
		},
		{
			Name:     "patch",
			Prompt: &survey.Input{
				Message:       "Enter patch version:",
				Default:       "0",
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

func (c *Create) qProjectDirAlreadyExist() (string, error) {
	var answer string
	prompt := &survey.Select{
		Message:       "Project directory already exist:",
		Options:       []string{
			"Merge",
			"Overwrite",
			"Cancel",
		},
		Default:       "Merge",
		Help:          "Merge - append files and overwrite if conflict\nOverwrite - clear directory and create project\nCancel - exit",
	}

	err := survey.AskOne(prompt, &answer)
	return answer, err
}
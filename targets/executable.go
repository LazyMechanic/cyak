package targets

import "github.com/AlecAivazis/survey/v2"

type Executable struct {
	Name                 string `survey:"name"`
	Lang                 string `survey:"lang"`
	LangStandard         string `survey:"standard"`
	LangExtensions       string `survey:"extensions"`
	LangStandardRequired string `survey:"standardrequired"`
	MajorVersion         string `survey:"major"`
	MinorVersion         string `survey:"minor"`
	PatchVersion         string `survey:"patch"`
	HasTest              bool   `survey:"hastest"`
	IsInherited          bool   `survey:"inherit"`
}

func (e *Executable) NonInheritedQuestions() []*survey.Question {
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

func (e *Executable) InheritedQuestions() []*survey.Question {
	return []*survey.Question{
		{
			Name: "name",
			Prompt: &survey.Input{
				Message: "Enter target name:",
			},
			Validate: survey.Required,
		},
		{
			Name: "hastest",
			Prompt: &survey.Confirm{
				Message: "Create test target:",
				Default: true,
			},
		},
	}
}

func (e *Executable) CreateTestTarget() *Test {
	test := &Test{
		Name:                 e.Name,
		Lang:                 e.Lang,
		LangStandard:         e.LangStandard,
		LangExtensions:       e.LangExtensions,
		LangStandardRequired: e.LangStandardRequired,
	}

	return test
}

func (e *Executable) Inherit(p *Project) {
	e.Lang = p.Lang
	e.LangStandard = p.LangStandard
	e.LangExtensions = p.LangExtensions
	e.LangStandardRequired = p.LangStandardRequired
	e.MajorVersion = p.MajorVersion
	e.MinorVersion = p.MinorVersion
	e.PatchVersion = p.PatchVersion
}

func NewExecutable() *Executable {
	return &Executable{}
}

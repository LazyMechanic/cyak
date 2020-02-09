package targets

import "github.com/AlecAivazis/survey/v2"

type Library struct {
	Name                 string `survey:"name"`
	Lang                 string `survey:"lang"`
	Namespace            string `survey:"namespace"`
	LangStandard         string `survey:"standard"`
	LangExtensions       string `survey:"extensions"`
	LangStandardRequired string `survey:"standardrequired"`
	MajorVersion         string `survey:"major"`
	MinorVersion         string `survey:"minor"`
	PatchVersion         string `survey:"patch"`
	HasTest              bool   `survey:"hastest"`
	IsInherited          bool   `survey:"inherit"`
}

func (l *Library) NonInheritedQuestions() []*survey.Question {
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

func (l *Library) InheritedQuestions() []*survey.Question {
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

func (l *Library) CreateTestTarget() *Test {
	test := &Test{
		Name:                 l.Name,
		Lang:                 l.Lang,
		LangStandard:         l.LangStandard,
		LangExtensions:       l.LangExtensions,
		LangStandardRequired: l.LangStandardRequired,
	}

	return test
}

func (l *Library) Inherit(p *Project) {
	l.Lang = p.Lang
	l.Namespace = p.Namespace
	l.LangStandard = p.LangStandard
	l.LangExtensions = p.LangExtensions
	l.LangStandardRequired = p.LangStandardRequired
	l.MajorVersion = p.MajorVersion
	l.MinorVersion = p.MinorVersion
	l.PatchVersion = p.PatchVersion
}

func NewLibrary() *Library {
	return &Library{}
}

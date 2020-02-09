package targets

import (
	"errors"
	"github.com/AlecAivazis/survey/v2"
)

type Project struct {
	Name                 string `survey:"name"`
	Lang                 string `survey:"lang"`
	Namespace            string `survey:"namespace"`
	LangStandard         string `survey:"standard"`
	LangExtensions       string `survey:"extensions"`
	LangStandardRequired string `survey:"standardrequired"`
	MajorVersion         string `survey:"major"`
	MinorVersion         string `survey:"minor"`
	PatchVersion         string `survey:"patch"`

	Executables []*Executable
	Libraries   []*Library
	Interfaces  []*Interface
	Tests       []*Test
}

func (p *Project) Questions() []*survey.Question {
	return []*survey.Question{
		{
			Name: "name",
			Prompt: &survey.Input{
				Message: "Enter project name:",
			},
			Validate: survey.Required,
		},
		{
			Name: "namespace",
			Prompt: &survey.Input{
				Message: "Enter general namespace:",
			},
			Validate: survey.Required,
		},
		{
			Name: "lang",
			Prompt: &survey.Select{
				Message: "Select project language:",
				Options: []string{
					"CXX",
					"C",
				},
				Default: "CXX",
			},
		},
		{
			Name: "standard",
			Prompt: &survey.Input{
				Message: "Enter general lang standard:",
				Default: "17",
			},
			Validate: isInt,
		},
		{
			Name: "extensions",
			Prompt: &survey.Select{
				Message: "Enable general lang extensions:",
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
				Message: "Enable general lang standard required:",
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
				Message: "Enter project major version:",
				Default: "0",
			},
			Validate: isInt,
		},
		{
			Name: "minor",
			Prompt: &survey.Input{
				Message: "Enter project minor version:",
				Default: "1",
			},
			Validate: isInt,
		},
		{
			Name: "patch",
			Prompt: &survey.Input{
				Message: "Enter project patch version:",
				Default: "0",
			},
			Validate: isInt,
		},
	}
}

func (p *Project) AddTarget(t Targeter) error {
	switch t.(type) {
	case *Executable:
		p.Executables = append(p.Executables, t.(*Executable))
	case *Library:
		p.Libraries = append(p.Libraries, t.(*Library))
	case *Interface:
		p.Interfaces = append(p.Interfaces, t.(*Interface))
	default:
		return errors.New("Unexpected target type")
	}

	return nil
}

func (p *Project) AddTest(t *Test) {
	p.Tests = append(p.Tests, t)
}

func NewProject() *Project {
	return &Project{}
}

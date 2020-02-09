package targets

import "errors"

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

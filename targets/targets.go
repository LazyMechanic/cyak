package targets

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

func (p *Project) AddExecutable(e *Executable) {
	p.Executables = append(p.Executables, e)
}

func (p *Project) AddLibrary(l *Library) {
	p.Libraries = append(p.Libraries, l)
}

func (p *Project) AddInterface(i *Interface) {
	p.Interfaces = append(p.Interfaces, i)
}

func (p *Project) AddTest(t *Test) {
	p.Tests = append(p.Tests, t)
}

func NewProject() *Project {
	return &Project{}
}

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

func (e *Executable) Inherit(p *Project) {
	e.Lang = p.Lang
	e.LangStandard = p.LangStandard
	e.LangExtensions = p.LangExtensions
	e.LangStandardRequired = p.LangStandardRequired
	e.MajorVersion = p.MajorVersion
	e.MinorVersion = p.MinorVersion
	e.PatchVersion = p.PatchVersion
}

func (e *Executable) CreateTest() *Test {
	test := &Test{
		Name:                 e.Name,
		Lang:                 e.Lang,
		LangStandard:         e.LangStandard,
		LangExtensions:       e.LangExtensions,
		LangStandardRequired: e.LangStandardRequired,
	}

	return test
}

func NewExecutable() *Executable {
	return &Executable{}
}

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

func (l *Library) CreateTest() *Test {
	test := &Test{
		Name:                 l.Name,
		Lang:                 l.Lang,
		LangStandard:         l.LangStandard,
		LangExtensions:       l.LangExtensions,
		LangStandardRequired: l.LangStandardRequired,
	}

	return test
}

func NewLibrary() *Library {
	return &Library{}
}

type Interface struct {
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

func (i *Interface) Inherit(p *Project) {
	i.Lang = p.Lang
	i.Namespace = p.Namespace
	i.LangStandard = p.LangStandard
	i.LangExtensions = p.LangExtensions
	i.LangStandardRequired = p.LangStandardRequired
	i.MajorVersion = p.MajorVersion
	i.MinorVersion = p.MinorVersion
	i.PatchVersion = p.PatchVersion
}

func (i *Interface) CreateTest() *Test {
	test := &Test{
		Name:                 i.Name,
		Lang:                 i.Lang,
		LangStandard:         i.LangStandard,
		LangExtensions:       i.LangExtensions,
		LangStandardRequired: i.LangStandardRequired,
	}

	return test
}

func NewInterface() *Interface {
	return &Interface{}
}

type Test struct {
	Name                 string `survey:"name"`
	Lang                 string `survey:"lang"`
	LangStandard         string `survey:"standard"`
	LangExtensions       string `survey:"extensions"`
	LangStandardRequired string `survey:"standardrequired"`
}

func NewTest() *Test {
	return &Test{}
}

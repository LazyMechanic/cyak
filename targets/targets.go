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

type Test struct {
	Name                 string `survey:"name"`
	Lang                 string `survey:"lang"`
	LangStandard         string `survey:"standard"`
	LangExtensions       string `survey:"extensions"`
	LangStandardRequired string `survey:"standardrequired"`
}

package types

type TargetType string
type YesNoType string
type OnOffType string
type LanguageType string
type Version int
type Task string

const (
	Yes YesNoType = "YES"
	No  YesNoType = "NO"
)

const (
	On  OnOffType = "ON"
	Off OnOffType = "OFF"
)

const (
	Cxx LanguageType = "CXX"
	C   LanguageType = "C"
)

type CreateConfig struct {
	WorkingDirectory string
	PresetDir        string
	CleanDirectory   bool
	CopyAsIs         bool
	LicenseText      string
	Project          *ProjectConfig
	Targets          []*TargetConfig
}

type TargetConfig struct {
	Name                string
	Namespace           string
	Type                TargetType
	CxxStandard         int
	CxxExtensions       OnOffType
	CxxStandardRequired YesNoType
	MajorVersion        Version
	MinorVersion        Version
	PatchVersion        Version
	CreateTest          bool
}

type ProjectConfig struct {
	Name                string
	Language            LanguageType
	CxxStandard         int
	CxxExtensions       OnOffType
	CxxStandardRequired YesNoType
	MajorVersion        Version
	MinorVersion        Version
	PatchVersion        Version
	Targets             []*TargetConfig
}

type LicenseConfig struct {
	Owner string
	Year  int
}

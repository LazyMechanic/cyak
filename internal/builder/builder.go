package builder

import (
	"bytes"
	"github.com/LazyMechanic/cyak/internal"
	"github.com/LazyMechanic/cyak/internal/templates"
	"log"
	"text/template"
)

type BuildType string
type Enabled string
type EnabledState string
type LanguageType string

const (
	Library    BuildType = "Library"
	Interface  BuildType = "Interface"
	Executable BuildType = "Executable"
)

const (
	Yes Enabled = "YES"
	No  Enabled = "NO"
)

const (
	On  EnabledState = "ON"
	Off EnabledState = "OFF"
)

const (
	Cxx LanguageType = "CXX"
	C   LanguageType = "C"
)

type Subproject struct {
	Name                string
	Namespace           string
	Type                BuildType
	CxxStandard         int
	CxxExtensions       EnabledState
	CxxStandardRequired Enabled
	MajorVersion        int
	MinorVersion        int
	PatchVersion        int
}

type Project struct {
	Name         string
	Subprojects  string
	Language     LanguageType
	MajorVersion int
	MinorVersion int
	PatchVersion int
}

type File struct {
	PathPrefix string
	Name       string
	Content    []byte
}

func checkName(v string) {
	if len(v) == 0 {
		log.Fatalf("invalid project name, expected length > 0, but handled 0")
	}
}

func checkLanguage(v LanguageType) {
	if v != Cxx && v != C {
		log.Fatalf("invalid cxx standard, expected [CXX, C], but handled \"%s\"", v)
	}
}

func checkCxxStandard(v int) {
	if v < 11 || (v-11)%3 != 0 {
		log.Fatalf("invalid cxx standard, expected [11, 14, 17, 20, etc.], but handled \"%d\"", v)
	}
}

func checkCxxExtensions(v EnabledState) {
	if v != On && v != Off {
		log.Fatalf("invalid cxx extensions, expected [ON, OFF], but handled \"%s\"", v)
	}
}

func checkCxxStandardRequired(v Enabled) {
	if v != Yes && v != No {
		log.Fatalf("invalid cxx extensions, expected [YES, NO], but handled \"%s\"", v)
	}
}

func checkSubproject(subproject Subproject) {
	checkName(subproject.Name)
	checkCxxStandard(subproject.CxxStandard)
	checkCxxExtensions(subproject.CxxExtensions)
	checkCxxStandardRequired(subproject.CxxStandardRequired)
}

func checkProject(project Project) {
	checkName(project.Name)
	checkLanguage(project.Language)
}

func CreateGeneral(config Project) File {
	checkProject(config)

	fileBytes, err := templates.Asset(internal.GeneralFile)
	internal.Check(err)

	content := string(fileBytes)

	tmpl, err := template.New("General").Parse(content)
	internal.Check(err)

	var tmplBuffer bytes.Buffer
	err = tmpl.Execute(&tmplBuffer, config)
	internal.Check(err)

	return File{
		PathPrefix: "",
		Name:       "CMakeList.txt",
		Content:    tmplBuffer.Bytes(),
	}
}

func CreateExecutable(config Subproject) File {
	checkSubproject(config)

	if config.Type != Executable {
		log.Fatalf("invalid subproject config type, expected \"Executable\", but handled \"%s\"", config.Type)
	}

	fileBytes, err := templates.Asset(internal.ExecutableFile)
	internal.Check(err)

	content := string(fileBytes)

	tmpl, err := template.New("Executable").Parse(content)
	internal.Check(err)

	var tmplBuffer bytes.Buffer
	err = tmpl.Execute(&tmplBuffer, config)
	internal.Check(err)

	return File{
		PathPrefix: "src/" + config.Name,
		Name:       "CMakeList.txt",
		Content:    tmplBuffer.Bytes(),
	}
}

func CreateLibrary(config Subproject) File {
	checkSubproject(config)

	if config.Type != Library {
		log.Fatalf("invalid function, expected config.Type==\"Library\", but handled config.Type==\"%s\"", config.Type)
	}

	fileBytes, err := templates.Asset(internal.LibraryFile)
	internal.Check(err)

	content := string(fileBytes)

	tmpl, err := template.New("Library").Parse(content)
	internal.Check(err)

	var tmplBuffer bytes.Buffer
	err = tmpl.Execute(&tmplBuffer, config)
	internal.Check(err)

	return File{
		PathPrefix: "src/" + config.Name,
		Name:       "CMakeList.txt",
		Content:    tmplBuffer.Bytes(),
	}
}

func CreateInterface(config Subproject) File {
	checkSubproject(config)

	if config.Type != Interface {
		log.Fatalf("invalid function, expected config.Type==\"Interface\", but handled config.Type==\"%s\"", config.Type)
	}

	fileBytes, err := templates.Asset(internal.InterfaceFile)
	internal.Check(err)

	content := string(fileBytes)

	tmpl, err := template.New("Interface").Parse(content)
	internal.Check(err)

	var tmplBuffer bytes.Buffer
	err = tmpl.Execute(&tmplBuffer, config)
	internal.Check(err)

	return File{
		PathPrefix: "include/" + config.Name,
		Name:       "CMakeList.txt",
		Content:    tmplBuffer.Bytes(),
	}
}

func CreateTargetConfig(config Subproject) File {
	fileBytes, err := templates.Asset(internal.TargetConfigInFile)
	internal.Check(err)

	content := string(fileBytes)

	return File{
		PathPrefix: "cmake/",
		Name:       config.Name + "-config.cmake.in",
		Content:    []byte(content),
	}
}

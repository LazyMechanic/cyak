package internal

import (
	"github.com/LazyMechanic/cyak/internal/templates"
	"strings"
)

func filterFilesAsIs(ss []string) (ret []string) {
	for _, s := range ss {
		if !strings.HasSuffix(s, ".templates") {
			ret = append(ret, s)
		}
	}
	return
}

var (
	CmakeFiles = filterFilesAsIs(templates.AssetNames())
)

const (
	ExecutableFile     = "Executable.txt.template"
	InterfaceFile      = "Interface.txt.template"
	GeneralFile        = "General.txt.template"
	LibraryFile        = "Library.txt.template"
	TargetConfigInFile = "TargetConfig.cmake.in.template"
)

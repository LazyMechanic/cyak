package cli

import (
	"fmt"
	"github.com/LazyMechanic/cyak/internal"
	"github.com/LazyMechanic/cyak/internal/builder"
	"github.com/abiosoft/ishell"
	"io/ioutil"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

func printErr(c *ishell.Context, err error) {
	c.Err(err)
}

func readName(c *ishell.Context) (string, error) {
	name := c.ReadLine()
	if name == "" {
		return name, fmt.Errorf("invalid name, expected name length > 0")
	}
	return name, nil
}

func readNamespace(c *ishell.Context) (string, error) {
	namespace := c.ReadLine()
	if namespace == "" {
		return namespace, fmt.Errorf("invalid namespace, expected namespace length > 0")
	}
	return namespace, nil
}

func readVersion(c *ishell.Context) (int, error) {
	versionString := c.ReadLine()
	if versionString == "" {
		return 0, nil
	}

	patchVersion, err := strconv.Atoi(versionString)
	if err != nil {
		return patchVersion, fmt.Errorf("invalid version, expected integer")
	}
	return patchVersion, nil
}

func readSubprojects(c *ishell.Context, buildType builder.BuildType) string {
	var result string

	space := regexp.MustCompile(`\s+`)
	subprojects := strings.TrimSpace(space.ReplaceAllString(c.ReadLine(), " "))

	if len(subprojects) != 0 {
		subprojectsSlice := strings.Split(subprojects, " ")

		// src or include
		var prefix string
		switch buildType {
		case builder.Executable:
			prefix = "  src/"
		case builder.Library:
			prefix = "  src/"
		case builder.Interface:
			prefix = "  include/"
		}

		for _, name := range subprojectsSlice {
			result += prefix + name + "\n"
		}
	}

	return result
}

func readCxxStandard(c *ishell.Context) (int, error) {
	cxxStandardString := c.ReadLine()
	if cxxStandardString == "" {
		return 17, nil
	}

	cxxStandard, err := strconv.Atoi(cxxStandardString)
	if err != nil {
		return 0, err
	}

	if cxxStandard < 11 || (cxxStandard-11)%3 != 0 {
		return 0, fmt.Errorf("invalid cxx standard, expected [11, 14, 17, 20, etc.], but handled \"%d\"", cxxStandard)
	}

	return cxxStandard, nil
}

func readCxxExtensions(c *ishell.Context) (builder.EnabledState, error) {
	cxxExtensionsString := c.ReadLine()
	if cxxExtensionsString == "" {
		return builder.Off, nil
	}

	cxxExtensions := builder.EnabledState(cxxExtensionsString)
	if cxxExtensions != builder.On && cxxExtensions != builder.Off {
		return cxxExtensions, fmt.Errorf("invalid cxx extensions, expected [ON, OFF], but handled \"%s\"", cxxExtensions)
	}

	return cxxExtensions, nil
}

func readCxxStandardRequired(c *ishell.Context) (builder.Enabled, error) {
	cxxStandardRequiredString := c.ReadLine()
	if cxxStandardRequiredString == "" {
		return builder.Yes, nil
	}

	cxxStandardRequired := builder.Enabled(cxxStandardRequiredString)
	if cxxStandardRequired != builder.Yes && cxxStandardRequired != builder.No {
		return cxxStandardRequired, fmt.Errorf("invalid cxx standard required, expected [YES, NO], but handled \"%s\"", cxxStandardRequired)
	}

	return cxxStandardRequired, nil
}

func Run() {
	var workingDir string = "./"

	// create new shell.
	// by default, new shell includes 'exit', 'help' and 'clear' commands.
	shell := ishell.New()

	// display welcome info.
	shell.Println("Utils for generate cmake files, enter \"help\" for show help")

	// register a function for "directory" command.
	shell.AddCmd(&ishell.Cmd{
		Name:    "directory",
		Aliases: []string{"dir"},
		Help:    "{dir} [directory] change working directory, create if it not exists. Show current without argument",
		Func: func(c *ishell.Context) {
			if len(c.Args) > 0 {
				if len(c.Args) != 1 {
					c.Println("Invalid argument count")
					return
				}

				workingDir = c.Args[0]

				err := os.MkdirAll(workingDir, os.ModePerm)
				if err != nil {
					printErr(c, err)
				}
			}

			abs := internal.AbsDir(workingDir)
			c.Println("Current working directory is " + abs)
			c.Println()
		},
	})

	// register a function for "general" command.
	shell.AddCmd(&ishell.Cmd{
		Name:    "general",
		Aliases: []string{"gen"},
		Help:    "{gen} generate general cmake list file",
		Func: func(c *ishell.Context) {
			c.ShowPrompt(false)
			defer c.ShowPrompt(true)

			var project = builder.Project{
				Name:         "",
				Language:     builder.Cxx,
				MajorVersion: 0,
				MinorVersion: 0,
				PatchVersion: 0,
				Subprojects:  "\n",
			}

			var err error

			c.Print("Project name: ")
			project.Name, err = readName(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// TODO: add generator for C project

			// Read major version
			c.Print("Major version (0 by default): ")
			project.MajorVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read minor version
			c.Print("Minor version (0 by default): ")
			project.MinorVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read patch version
			c.Print("Patch version (0 by default): ")
			project.PatchVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read subprojects
			c.Print("Executable subprojects (name separating by a space): ")
			project.Subprojects += readSubprojects(c, builder.Executable)
			c.Print("Library subprojects (name separating by a space): ")
			project.Subprojects += readSubprojects(c, builder.Library)
			c.Print("Interface subprojects (name separating by a space): ")
			project.Subprojects += readSubprojects(c, builder.Interface)

			if project.Subprojects == "\n" {
				project.Subprojects = "\n\"\""
			}

			// Create file
			projectFile := builder.CreateGeneral(project)

			folderPath := workingDir
			err = os.MkdirAll(folderPath, os.ModePerm)
			if err != nil {
				printErr(c, err)
			}

			fullPath := filepath.Join(folderPath, projectFile.Name)
			err = ioutil.WriteFile(fullPath, projectFile.Content, 0644)
			if err != nil {
				printErr(c, err)
				return
			}

			abs := internal.AbsDir(workingDir)
			c.Println("Generate general cmake file at: " + abs)
			c.Println()
		},
	})

	// register a function for "executable" command.
	shell.AddCmd(&ishell.Cmd{
		Name:    "executable",
		Aliases: []string{"exec"},
		Help:    "{exec} generate executable cmake list file",
		Func: func(c *ishell.Context) {
			c.ShowPrompt(false)
			defer c.ShowPrompt(true)

			var subproject = builder.Subproject{
				Name:                "",
				Namespace:           "",
				Type:                builder.Executable,
				CxxStandard:         17,
				CxxExtensions:       builder.Off,
				CxxStandardRequired: builder.Yes,
				MajorVersion:        0,
				MinorVersion:        0,
				PatchVersion:        0,
			}

			var err error

			c.Print("Subproject name: ")
			subproject.Name, err = readName(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read major version
			c.Print("Major version (0 by default): ")
			subproject.MajorVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read minor version
			c.Print("Minor version (0 by default): ")
			subproject.MinorVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read patch version
			c.Print("Patch version (0 by default): ")
			subproject.PatchVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx standard
			c.Print("Cxx standard (17 by default): ")
			subproject.CxxStandard, err = readCxxStandard(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx extensions
			c.Print("Cxx extensions (OFF by default): ")
			subproject.CxxExtensions, err = readCxxExtensions(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx standard required
			c.Print("Cxx standard required (YES by default): ")
			subproject.CxxStandardRequired, err = readCxxStandardRequired(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Create file
			subprojectFile := builder.CreateExecutable(subproject)

			folderPath := filepath.Join(workingDir, subprojectFile.PathPrefix)
			err = os.MkdirAll(folderPath, os.ModePerm)
			if err != nil {
				printErr(c, err)
			}

			fullPath := filepath.Join(workingDir, subprojectFile.PathPrefix, subprojectFile.Name)
			err = ioutil.WriteFile(fullPath, subprojectFile.Content, 0644)
			if err != nil {
				printErr(c, err)
				return
			}

			abs := internal.AbsDir(workingDir)
			c.Println("Generate executable cmake file at: " + abs)
			c.Println()
		},
	})

	// register a function for "library" command.
	shell.AddCmd(&ishell.Cmd{
		Name:    "library",
		Aliases: []string{"lib"},
		Help:    "{lib} generate library cmake list file",
		Func: func(c *ishell.Context) {
			c.ShowPrompt(false)
			defer c.ShowPrompt(true)

			var subproject = builder.Subproject{
				Name:                "",
				Namespace:           "",
				Type:                builder.Library,
				CxxStandard:         17,
				CxxExtensions:       builder.Off,
				CxxStandardRequired: builder.Yes,
				MajorVersion:        0,
				MinorVersion:        0,
				PatchVersion:        0,
			}

			var err error

			// Read subproject name
			c.Print("Subproject name: ")
			subproject.Name, err = readName(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read namespace
			c.Print("Namespace: ")
			subproject.Namespace, err = readNamespace(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read major version
			c.Print("Major version (0 by default): ")
			subproject.MajorVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read minor version
			c.Print("Minor version (0 by default): ")
			subproject.MinorVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read patch version
			c.Print("Patch version (0 by default): ")
			subproject.PatchVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx standard
			c.Print("Cxx standard (17 by default): ")
			subproject.CxxStandard, err = readCxxStandard(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx extensions
			c.Print("Cxx extensions (OFF by default): ")
			subproject.CxxExtensions, err = readCxxExtensions(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx standard required
			c.Print("Cxx standard required (YES by default): ")
			subproject.CxxStandardRequired, err = readCxxStandardRequired(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Create file
			subprojectFile := builder.CreateLibrary(subproject)

			folderPath := filepath.Join(workingDir, subprojectFile.PathPrefix)
			err = os.MkdirAll(folderPath, os.ModePerm)
			if err != nil {
				printErr(c, err)
			}

			fullPath := filepath.Join(folderPath, subprojectFile.Name)
			err = ioutil.WriteFile(fullPath, subprojectFile.Content, 0644)
			if err != nil {
				printErr(c, err)
				return
			}

			abs := internal.AbsDir(workingDir)
			c.Println("Generate library cmake file at: " + abs)
			c.Println()
		},
	})

	// register a function for "interface" command.
	shell.AddCmd(&ishell.Cmd{
		Name:    "interface",
		Aliases: []string{"int"},
		Help:    "{int} generate interface cmake list file",
		Func: func(c *ishell.Context) {
			c.ShowPrompt(false)
			defer c.ShowPrompt(true)

			var subproject = builder.Subproject{
				Name:                "",
				Namespace:           "",
				Type:                builder.Interface,
				CxxStandard:         17,
				CxxExtensions:       builder.Off,
				CxxStandardRequired: builder.Yes,
				MajorVersion:        0,
				MinorVersion:        0,
				PatchVersion:        0,
			}

			var err error

			// Read subproject name
			c.Print("Subproject name: ")
			subproject.Name, err = readName(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read namespace
			c.Print("Namespace: ")
			subproject.Namespace, err = readNamespace(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read major version
			c.Print("Major version (0 by default): ")
			subproject.MajorVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read minor version
			c.Print("Minor version (0 by default): ")
			subproject.MinorVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read patch version
			c.Print("Patch version (0 by default): ")
			subproject.PatchVersion, err = readVersion(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx standard
			c.Print("Cxx standard (17 by default): ")
			subproject.CxxStandard, err = readCxxStandard(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx extensions
			c.Print("Cxx extensions (OFF by default): ")
			subproject.CxxExtensions, err = readCxxExtensions(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Read cxx standard required
			c.Print("Cxx standard required (YES by default): ")
			subproject.CxxStandardRequired, err = readCxxStandardRequired(c)
			if err != nil {
				printErr(c, err)
				return
			}

			// Create file
			subprojectFile := builder.CreateInterface(subproject)

			folderPath := filepath.Join(workingDir, subprojectFile.PathPrefix)
			err = os.MkdirAll(folderPath, os.ModePerm)
			if err != nil {
				printErr(c, err)
			}

			fullPath := filepath.Join(folderPath, subprojectFile.Name)
			err = ioutil.WriteFile(fullPath, subprojectFile.Content, 0644)
			if err != nil {
				printErr(c, err)
				return
			}

			abs := internal.AbsDir(workingDir)
			c.Println("Generate interface cmake file at: " + abs)
			c.Println()
		},
	})

	// run shell
	shell.Run()
}

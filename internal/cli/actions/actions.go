package actions

import (
	"fmt"
	"github.com/LazyMechanic/cyak/internal/cli/dialog"
	"github.com/LazyMechanic/cyak/internal/cli/flags"
	"github.com/LazyMechanic/cyak/internal/cli/questions"
	"github.com/LazyMechanic/cyak/internal/config"
	"github.com/LazyMechanic/cyak/internal/template"
	"github.com/LazyMechanic/cyak/internal/types"
	"github.com/otiai10/copy"
	gocli "github.com/urfave/cli"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"path/filepath"
)

func toSurvey(createConfig *types.CreateConfig) error {
	// If working directory already exists
	if _, err := os.Stat(createConfig.WorkingDirectory); !os.IsNotExist(err) {
		ans := questions.AskDirectoryAlreadyExists(createConfig.WorkingDirectory)
		switch ans {
		case dialog.Overwrite:
			sure := questions.AskAreYouSure()
			if !sure {
				return nil
			}
			createConfig.CleanDirectory = true
		case dialog.Merge:
			/* continue to survey */
		case dialog.Cancel:
			return nil
		}
	}

	// Set preset
	createConfig.PresetDir = filepath.Join(config.PresetsFolder, questions.AskPresetName())

	// While not done
	for {
		whatCreate := questions.AskTask()
		switch whatCreate {
		case dialog.ConfigureProject:
			createConfig.Project = questions.ProjectConfigureSurvey()
		case dialog.AddExecutable:
			createConfig.Targets = append(createConfig.Targets, questions.AddTargetSurvey(createConfig.Project, template.Executable))
		case dialog.AddLibrary:
			createConfig.Targets = append(createConfig.Targets, questions.AddTargetSurvey(createConfig.Project, template.Library))
		case dialog.AddInterface:
			createConfig.Targets = append(createConfig.Targets, questions.AddTargetSurvey(createConfig.Project, template.Interface))
		case dialog.Save:
			/* Save all to disk */
			return nil
		case dialog.Cancel:
			/* Close app */
			return fmt.Errorf("All changes are discarded, nothing was saved to disk")
		}
	}

	return nil
}

func clearDir(dir string) error {
	files, err := filepath.Glob(filepath.Join(dir, "*"))
	if err != nil {
		return err
	}
	for _, file := range files {
		err = os.RemoveAll(file)
		if err != nil {
			return err
		}
	}
	return nil
}

func doCreate(createConfig *types.CreateConfig) error {
	if createConfig == nil {
		panic(fmt.Errorf("Create config is nil"))
	}

	// If need clean folder for overwriting
	if createConfig.CleanDirectory {
		if err := clearDir(createConfig.WorkingDirectory); err != nil {
			return err
		}
	}

	// Check if working directory doesn't exists and create it
	if err := createDirIfNotExist(createConfig.WorkingDirectory); err != nil {
		return err
	}

	// Git init if needs
	if flags.GitFlagValue {
		var cmd = exec.Command("git", "init", createConfig.WorkingDirectory)
		if err := cmd.Run(); err != nil {
			return err
		}
	}

	// Process project config
	if createConfig.Project != nil {
		// If there are no targets set ""
		if len(createConfig.Targets) == 0 {
			createConfig.Project.Targets = "\"\""
		} else {
			for _, target := range createConfig.Targets {
				createConfig.Project.Targets += fmt.Sprintf("\n  %s/%s", getTargetDirSuffix(target.Type), target.Name)
			}
		}

		// Read and fill template and write to disk
		var templateFile = filepath.Join(createConfig.PresetDir, config.ProjectTemplate)
		var templateContent = template.ProjectFile(templateFile, createConfig.Project)
		var destFile = filepath.Join(createConfig.WorkingDirectory, "CMakeLists.txt")
		template.WriteToDisk(destFile, templateContent)
	}

	// Process targets
	var cmakeWorkingDir = filepath.Join(createConfig.WorkingDirectory, "cmake")
	for _, target := range createConfig.Targets {
		var targetDestDir = filepath.Join(createConfig.WorkingDirectory, getTargetDirSuffix(target.Type), target.Name)
		// Create target directory
		if err := createDirIfNotExist(targetDestDir); err != nil {
			return err
		}
		var targetTemplateFile = filepath.Join(createConfig.PresetDir, template.GetTargetTemplateFileName(target.Type))
		var targetTemplateContent = template.TargetFile(targetTemplateFile, target)
		var targetDestFile = filepath.Join(targetDestDir, "CMakeLists.txt")
		template.WriteToDisk(targetDestFile, targetTemplateContent)

		// Create config file for library and interface
		if target.Type == template.Library || target.Type == template.Interface {
			// Create cmake directory
			if err := createDirIfNotExist(cmakeWorkingDir); err != nil {
				return err
			}
			// Copy config file
			var configTemplateFile = filepath.Join(createConfig.PresetDir, config.TargetConfigTemplate)
			// Read config template file
			fileBytes, err := ioutil.ReadFile(configTemplateFile)
			if err != nil {
				panic(err)
			}
			var configTemplateContent = string(fileBytes)
			var configDestFile = filepath.Join(cmakeWorkingDir, fmt.Sprintf("%s-config.cmake.in", target.Name))
			template.WriteToDisk(configDestFile, configTemplateContent)
		}
	}

	// Process cmake files
	var cmakePresetDir = filepath.Join(createConfig.PresetDir, "cmake")
	if _, err := os.Stat(cmakePresetDir); !os.IsNotExist(err) {
		if err := copy.Copy(cmakePresetDir, cmakeWorkingDir); err != nil {
			return err
		}
	}

	return nil
}

func createDirIfNotExist(dir string) error {
	if _, err := os.Stat(dir); os.IsNotExist(err) {
		// Make working directory
		return createDir(dir)
	}

	return nil
}

func createDir(dir string) error {
	return os.MkdirAll(dir, os.ModePerm)
}

func getTargetDirSuffix(targetType types.TargetType) string {
	switch targetType {
	case template.Executable:
		return "src"
	case template.Library:
		return "src"
	case template.Interface:
		return "include"
	default:
		panic(fmt.Errorf("Invalid target type"))
	}
	return ""
}

func Create(context *gocli.Context) error {
	defer func() {
		if r := recover(); r != nil {
			log.Fatal(r)
		}
	}()

	if context.NArg() != 1 {
		return fmt.Errorf("Missing required argument %s", context.Command.ArgsUsage)
	}

	if context.Args().Get(0) == "" {
		return fmt.Errorf("Working directory is empty")
	}

	var err error
	var createConfig types.CreateConfig

	// Set working directory
	createConfig.WorkingDirectory, err = filepath.Abs(context.Args().Get(0))
	if err != nil {
		return err
	}

	err = toSurvey(&createConfig)
	if err != nil {
		return err
	}

	return doCreate(&createConfig)
}

package actions

import (
	"fmt"
	"github.com/LazyMechanic/cyak/internal/cli/assist"
	"github.com/LazyMechanic/cyak/internal/cli/dialog"
	"github.com/LazyMechanic/cyak/internal/cli/flags"
	"github.com/LazyMechanic/cyak/internal/cli/questions"
	"github.com/LazyMechanic/cyak/internal/config"
	"github.com/LazyMechanic/cyak/internal/preset"
	"github.com/LazyMechanic/cyak/internal/template"
	"github.com/LazyMechanic/cyak/internal/types"
	"github.com/disiqueira/gotree"
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
	createConfig.PresetDir = filepath.Join(flags.PresetFlagValue, questions.AskPresetName())

	// While not done
	for {
		whatCreate := questions.AskTask()
		switch whatCreate {
		case dialog.ConfigureProject:
			createConfig.Project = questions.ProjectConfigureSurvey()
		case dialog.AddExecutable:
			var target = questions.TargetSurvey(createConfig.Project, template.Executable)
			if target != nil {
				createConfig.Targets = append(createConfig.Targets, target)
			} else {
				fmt.Println("Target are discard")
			}
		case dialog.AddLibrary:
			var target = questions.TargetSurvey(createConfig.Project, template.Library)
			if target != nil {
				createConfig.Targets = append(createConfig.Targets, target)
			} else {
				fmt.Println("Target are discard")
			}
		case dialog.AddInterface:
			var target = questions.TargetSurvey(createConfig.Project, template.Interface)
			if target != nil {
				createConfig.Targets = append(createConfig.Targets, target)
			} else {
				fmt.Println("Target are discard")
			}
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
		createConfig.Project.Targets = createConfig.Targets

		// Read and fill template and write to disk
		var templateFile = filepath.Join(createConfig.PresetDir, config.TemplatesFolder, config.ProjectTemplate)
		var templateContent = template.ProjectFile(templateFile, createConfig.Project)
		var destFile = filepath.Join(createConfig.WorkingDirectory, "CMakeLists.txt")
		assist.WriteToDisk(destFile, templateContent)
	}

	// Process targets
	var cmakeWorkingDir = filepath.Join(createConfig.WorkingDirectory, "cmake")
	for _, target := range createConfig.Targets {
		var targetDestDir = filepath.Join(createConfig.WorkingDirectory, getTargetDirSuffix(target.Type), target.Name)
		// Create target directory
		if err := createDirIfNotExist(targetDestDir); err != nil {
			return err
		}
		var targetTemplateFile = filepath.Join(createConfig.PresetDir, config.TemplatesFolder, template.GetTargetTemplateFileName(target.Type))
		var targetTemplateContent = template.TargetFile(targetTemplateFile, target)
		var targetDestFile = filepath.Join(targetDestDir, "CMakeLists.txt")
		assist.WriteToDisk(targetDestFile, targetTemplateContent)

		// Create config file for library and interface
		if target.Type == template.Library || target.Type == template.Interface {
			// Create cmake directory
			if err := createDirIfNotExist(cmakeWorkingDir); err != nil {
				return err
			}
			// Copy config file
			var configTemplateFile = filepath.Join(createConfig.PresetDir, config.TemplatesFolder, config.TargetConfigTemplate)
			// Read config template file
			fileBytes, err := ioutil.ReadFile(configTemplateFile)
			if err != nil {
				panic(err)
			}
			var configTemplateContent = string(fileBytes)
			var configDestFile = filepath.Join(cmakeWorkingDir, fmt.Sprintf("%s-config.cmake.in", target.Name))
			assist.WriteToDisk(configDestFile, configTemplateContent)
		}

		// If need create test
		if target.CreateTest {
			var testDestDir = filepath.Join(createConfig.WorkingDirectory, "test", target.Name)
			// Create test directory
			if err := createDirIfNotExist(testDestDir); err != nil {
				return err
			}
			var testTemplateFile = filepath.Join(createConfig.PresetDir, config.TemplatesFolder, config.TestTemplate)
			var testTemplateContent = template.TargetFile(testTemplateFile, target)
			var testDestFile = filepath.Join(testDestDir, "CMakeLists.txt")
			assist.WriteToDisk(testDestFile, testTemplateContent)
		}
	}

	// Process as is directory
	var asIsDir = filepath.Join(createConfig.PresetDir, config.AsIsFolder)
	if _, err := os.Stat(asIsDir); !os.IsNotExist(err) {
		// Copy all files and folders in asis directory to working directory
		assist.Copy(asIsDir, createConfig.WorkingDirectory)
	}

	return showDirTree(createConfig.WorkingDirectory)
}

func showDirTree(rootDir string) error {
	tree, err := subDirTree(rootDir)
	if err != nil {
		return err
	}

	fmt.Println(tree.Print())
	return nil
}

func subDirTree(file string) (gotree.Tree, error) {
	var root = gotree.New(filepath.Base(file))

	fileStat, err := os.Stat(file)
	if err != nil {
		return nil, err
	}

	if fileStat.Mode().IsDir() {
		subdirs, err := ioutil.ReadDir(file)
		if err != nil {
			return nil, err
		}
		for _, subdir := range subdirs {
			subTree, err := subDirTree(filepath.Join(file, subdir.Name()))
			if err != nil {
				return nil, err
			}

			root.AddTree(subTree)
		}
	}

	return root, nil
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
	if err = preset.IsPresetsValid(preset.PresetsNames()); err != nil {
		return fmt.Errorf("Invalid presets:\n%v", err)
	}

	var createConfig types.CreateConfig

	// Set working directory
	createConfig.WorkingDirectory = context.Args().Get(0)

	err = toSurvey(&createConfig)
	if err != nil {
		return err
	}

	return doCreate(&createConfig)
}

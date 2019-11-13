package questions

import (
	"fmt"
	"github.com/AlecAivazis/survey"
	"github.com/LazyMechanic/cyak/internal/cli/dialog"
	"github.com/LazyMechanic/cyak/internal/preset"
	"github.com/LazyMechanic/cyak/internal/template"
	"github.com/LazyMechanic/cyak/internal/types"
	"strconv"
)

func ProjectConfigureSurvey() *types.ProjectConfig {
	var project = &types.ProjectConfig{
		Name:                AskName(),
		Language:            AskLanguage(),
		CxxStandard:         AskCxxStandard(),
		CxxExtensions:       AskCxxExtensions(),
		CxxStandardRequired: AskCxxStandardRequired(),
		MajorVersion:        AskMajorVersion(),
		MinorVersion:        AskMinorVersion(),
		PatchVersion:        AskPatchVersion(),
	}

	return project
}

func TargetSurvey(project *types.ProjectConfig, targetType types.TargetType) *types.TargetConfig {
	var inherit bool = false
	if project != nil {
		inherit = AskInherit()
	}

	switch targetType {
	case template.Executable:
		return executableSurvey(project, inherit)
	case template.Library:
		return librarySurvey(project, inherit)
	case template.Interface:
		return interfaceSurvey(project, inherit)
	default:
		panic(fmt.Errorf("Invalid target type"))
	}

	return nil
}

func executableSurvey(project *types.ProjectConfig, inherit bool) *types.TargetConfig {
	var target = &types.TargetConfig{
		Name: AskName(),
		CreateTest: AskCreateTest(),
		Type: template.Executable,
	}

	inheritOptions(project, inherit, target)

	return target
}

func librarySurvey(project *types.ProjectConfig, inherit bool) *types.TargetConfig {
	var target = &types.TargetConfig{
		Name:      AskName(),
		Namespace: AskNamespace(),
		CreateTest: AskCreateTest(),
		Type:      template.Library,
	}

	inheritOptions(project, inherit, target)

	return target
}

func interfaceSurvey(project *types.ProjectConfig, inherit bool) *types.TargetConfig {
	var target = &types.TargetConfig{
		Name:      AskName(),
		Namespace: AskNamespace(),
		CreateTest: AskCreateTest(),
		Type:      template.Interface,
	}

	inheritOptions(project, inherit, target)

	return target
}

func inheritOptions(project *types.ProjectConfig, inherit bool, target *types.TargetConfig) {
	if inherit {
		if project == nil {
			panic("Project config is nil")
		}

		// Inherit options
		target.CxxStandard = project.CxxStandard
		target.CxxExtensions = project.CxxExtensions
		target.CxxStandardRequired = project.CxxStandardRequired
		target.MajorVersion = project.MajorVersion
		target.MinorVersion = project.MinorVersion
		target.PatchVersion = project.PatchVersion
	} else {
		// Ask options
		target.CxxStandard = AskCxxStandard()
		target.CxxExtensions = AskCxxExtensions()
		target.CxxStandardRequired = AskCxxStandardRequired()
		target.MajorVersion = AskMajorVersion()
		target.MinorVersion = AskMinorVersion()
		target.PatchVersion = AskPatchVersion()
	}
}

func AskInherit() bool {
	var answer bool
	var prompt = &survey.Confirm{
		Message: "Inherit project options:",
		Default: true,
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}
	return answer
}

func AskPresetName() string {
	var presets = preset.PresetsNames()

	var answer string
	var prompt = &survey.Select{
		Message: "Pick a preset:",
		Options: presets,
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}
	return answer
}

func AskTask() types.Task {
	var answer string
	var prompt = &survey.Select{
		Message: "What to do:",
		Options: []string{
			string(dialog.ConfigureProject),
			string(dialog.AddExecutable),
			string(dialog.AddLibrary),
			string(dialog.AddInterface),
			string(dialog.Save),
			string(dialog.Cancel),
		},
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}

	return types.Task(answer)
}

func AskLanguage() types.LanguageType {
	var answer string
	var prompt = &survey.Select{
		Message: "Pick project language:",
		Options: []string{
			string(types.Cxx),
			string(types.C),
		},
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}

	return types.LanguageType(answer)
}

func AskName() string {
	var answer string
	var prompt = &survey.Input{
		Message: "Enter name:",
	}

	err := survey.AskOne(prompt, &answer, survey.WithValidator(survey.Required))
	if err != nil {
		panic(err)
	}

	return answer
}

func AskNamespace() string {
	var answer string
	var prompt = &survey.Input{
		Message: "Enter namespace:",
	}

	err := survey.AskOne(prompt, &answer, survey.WithValidator(survey.Required))
	if err != nil {
		panic(err)
	}

	return answer
}

func integerValidator(val interface{}) error {
	if _, ok := val.(string); !ok {
		return fmt.Errorf("Invalid answer, cannot cast to string")
	}

	_, err := strconv.Atoi(val.(string))
	if err != nil {
		return fmt.Errorf("Answer can be only is integer")
	}

	return nil
}

func cxxStandardValidator(val interface{}) error {
	if _, ok := val.(string); !ok {
		return fmt.Errorf("Invalid answer, cannot cast to string")
	}

	valInt, err := strconv.Atoi(val.(string))
	if err != nil {
		return fmt.Errorf("Answer can be only is integer")
	}

	if (valInt-11)%3 != 0 {
		return fmt.Errorf("Invalid C++ standard, expected [11, 14, 17, 20, etc.]")
	}

	return nil
}

func AskCxxStandard() int {
	var answer string
	var prompt = &survey.Input{
		Message: "Enter C++ standard:",
		Default: "17",
	}

	err := survey.AskOne(prompt, &answer, survey.WithValidator(cxxStandardValidator))
	if err != nil {
		panic(err)
	}

	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return answerInt
}

func AskCxxExtensions() types.OnOffType {
	var answer string
	var prompt = &survey.Select{
		Message: "Turn on C++ extensions:",
		Options: []string{
			string(types.On),
			string(types.Off),
		},
		Default: string(types.Off),
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}

	return types.OnOffType(answer)
}

func AskCxxStandardRequired() types.YesNoType {
	var answer string
	var prompt = &survey.Select{
		Message: "Required C++ standard:",
		Options: []string{
			string(types.Yes),
			string(types.No),
		},
		Default: string(types.Yes),
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}

	return types.YesNoType(answer)
}

func AskMajorVersion() types.Version {
	var answer string
	var prompt = &survey.Input{
		Message: "Enter major version:",
		Default: "0",
	}

	err := survey.AskOne(prompt, &answer, survey.WithValidator(integerValidator))
	if err != nil {
		panic(err)
	}

	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return types.Version(answerInt)
}

func AskMinorVersion() types.Version {
	var answer string
	var prompt = &survey.Input{
		Message: "Enter minor version:",
		Default: "0",
	}

	err := survey.AskOne(prompt, &answer, survey.WithValidator(integerValidator))
	if err != nil {
		panic(err)
	}

	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return types.Version(answerInt)
}

func AskPatchVersion() types.Version {
	var answer string
	var prompt = &survey.Input{
		Message: "Enter patch version:",
		Default: "0",
	}

	err := survey.AskOne(prompt, &answer, survey.WithValidator(integerValidator))
	if err != nil {
		panic(err)
	}

	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return types.Version(answerInt)
}

func AskCreateTest() bool {
	var answer bool
	var prompt = &survey.Confirm{
		Message: "Create test target:",
		Default: true,
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}

	return answer
}

func AskDirectoryAlreadyExists(dir string) types.Task {
	var answer string
	var prompt = &survey.Select{
		Message: fmt.Sprintf("Project direcrory %s already exists. Pick an action:", dir),
		Options: []string{
			string(dialog.Overwrite),
			string(dialog.Merge),
			string(dialog.Cancel),
		},
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}

	return types.Task(answer)
}

func AskAreYouSure() bool {
	var answer bool
	var prompt = &survey.Confirm{
		Message: "Are you sure:",
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}

	return answer
}

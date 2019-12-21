package questions

import (
	"fmt"
	"github.com/AlecAivazis/survey"
	"github.com/LazyMechanic/cyak/internal/cli/dialog"
	"github.com/LazyMechanic/cyak/internal/preset"
	"github.com/LazyMechanic/cyak/internal/template"
	"github.com/LazyMechanic/cyak/internal/types"
	"github.com/LazyMechanic/cyak/internal/licenses"
	"strconv"
	"time"
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

	var isCorrect = AskIsInformationCorrect()
	if isCorrect {
		return project
	}

	return nil
}

func TargetSurvey(project *types.ProjectConfig, targetType types.TargetType) *types.TargetConfig {
	var target *types.TargetConfig

	var inherit bool = false
	if project != nil {
		inherit = AskInherit()
	}

	switch targetType {
	case template.Executable:
		target = executableSurvey(project, inherit)
	case template.Library:
		target = librarySurvey(project, inherit)
	case template.Interface:
		target = interfaceSurvey(project, inherit)
	default:
		panic(fmt.Errorf("Invalid target type"))
	}

	var isCorrect = AskIsInformationCorrect()
	if isCorrect {
		return target
	}

	return nil
}

func executableSurvey(project *types.ProjectConfig, inherit bool) *types.TargetConfig {
	var target = &types.TargetConfig{
		Name:       AskName(),
		CreateTest: AskCreateTest(),
		Type:       template.Executable,
	}

	inheritOptions(project, inherit, target)

	return target
}

func librarySurvey(project *types.ProjectConfig, inherit bool) *types.TargetConfig {
	var target = &types.TargetConfig{
		Name:       AskName(),
		Namespace:  AskNamespace(),
		CreateTest: AskCreateTest(),
		Type:       template.Library,
	}

	inheritOptions(project, inherit, target)

	return target
}

func interfaceSurvey(project *types.ProjectConfig, inherit bool) *types.TargetConfig {
	var target = &types.TargetConfig{
		Name:       AskName(),
		Namespace:  AskNamespace(),
		CreateTest: AskCreateTest(),
		Type:       template.Interface,
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

func confirm(msg string, defaultValue bool) bool {
	var answer bool
	var prompt = &survey.Confirm{
		Message: msg,
		Default: defaultValue,
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}
	return answer
}

func selectOptions(msg string, defaultValue string, options []string) string {
	var answer string
	var prompt = &survey.Select{
		Message: msg,
		Options: options,
		Default: defaultValue,
	}

	err := survey.AskOne(prompt, &answer)
	if err != nil {
		panic(err)
	}
	return answer
}

func input(msg string, defaultValue string, opts ...survey.AskOpt) string {
	var answer string
	var prompt = &survey.Input{
		Message: msg,
		Default: defaultValue,
	}

	err := survey.AskOne(prompt, &answer, opts...)
	if err != nil {
		panic(err)
	}

	return answer
}

func AskIsInformationCorrect() bool {
	return confirm("Is information correct:", true)
}

func AskInherit() bool {
	return confirm("Inherit project options:", true)
}

func AskPresetName() string {
	var presets = preset.PresetsNames()
	return selectOptions("Pick a preset:", presets[0], presets)
}

func AskTask(config *types.CreateConfig) types.Task {
	return types.Task(selectOptions("What to do:", string(dialog.ConfigureProject), []string{
		string(dialog.ConfigureProject),
		string(dialog.AddExecutable),
		string(dialog.AddLibrary),
		string(dialog.AddInterface),
		fmt.Sprintf("%s [%v]", dialog.CopyAsIs, config.CopyAsIs),
		string(dialog.AddLicense),
		string(dialog.Save),
		string(dialog.Cancel),
	}))
}

func AskLanguage() types.LanguageType {
	return types.LanguageType(selectOptions("Pick project language:", string(types.Cxx), []string{
		string(types.Cxx),
		string(types.C),
	}))
}

func AskName() string {
	return input("Enter name:", "", survey.WithValidator(survey.Required))
}

func AskNamespace() string {
	return input("Enter namespace:", "", survey.WithValidator(survey.Required))
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
	var answer = input("Enter C++ standard:", "17", survey.WithValidator(cxxStandardValidator))
	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return answerInt
}

func AskCxxExtensions() types.OnOffType {
	return types.OnOffType(selectOptions("Turn on C++ extensions:", string(types.Off), []string{
		string(types.On),
		string(types.Off),
	}))
}

func AskCxxStandardRequired() types.YesNoType {

	return types.YesNoType(selectOptions("Required C++ standard:", string(types.Yes), []string{
		string(types.Yes),
		string(types.No),
	}))
}

func AskMajorVersion() types.Version {
	var answer = input("Enter major version:", "0", survey.WithValidator(integerValidator))
	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return types.Version(answerInt)
}

func AskMinorVersion() types.Version {
	var answer = input("Enter minor version:", "0", survey.WithValidator(integerValidator))
	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return types.Version(answerInt)
}

func AskPatchVersion() types.Version {
	var answer = input("Enter patch version:", "0", survey.WithValidator(integerValidator))
	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return types.Version(answerInt)
}

func AskChooseTheLicense() string {
	return selectOptions("ChooseTheLicense:", licenses.List[0], licenses.List)
}

func AskLicenseOwner() string {
	return input("Enter owner name:", "", survey.WithValidator(survey.Required))
}

func AskLicenseYear() int {
	var answer = input("Enter year:", fmt.Sprint(time.Now().Year()), survey.WithValidator(integerValidator))
	answerInt, err := strconv.Atoi(answer)
	if err != nil {
		panic(err)
	}

	return answerInt
}

func AskCreateTest() bool {
	return confirm("Create test target:", true)
}

func AskDirectoryAlreadyExists(dir string) types.Task {
	return types.Task(selectOptions(fmt.Sprintf("Project direcrory %s already exists. Pick an action:", dir), string(dialog.Overwrite), []string{
		string(dialog.Overwrite),
		string(dialog.Merge),
		string(dialog.Cancel),
	}))
}

func AskAreYouSure() bool {
	return confirm("Are you sure:", true)
}

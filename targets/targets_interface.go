package targets

import "github.com/AlecAivazis/survey/v2"

func GetAllQuestions(t Targeter) []*survey.Question {
	return append(t.InheritedQuestions(), t.NonInheritedQuestions()...)
}

type Targeter interface {
	NonInheritedQuestions() []*survey.Question
	InheritedQuestions() []*survey.Question
	Inherit(p *Project)
	CreateTestTarget() *Test
}
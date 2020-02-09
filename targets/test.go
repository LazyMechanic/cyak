package targets

type Test struct {
	Name                 string `survey:"name"`
	Lang                 string `survey:"lang"`
	LangStandard         string `survey:"standard"`
	LangExtensions       string `survey:"extensions"`
	LangStandardRequired string `survey:"standardrequired"`
}

func NewTest() *Test {
	return &Test{}
}

package assist

import (
	"testing"
)

var trueTestsOneTrueOf = [][]bool{
	[]bool{true},
	[]bool{true, false},
	[]bool{true, false, false},
	[]bool{false, true},
	[]bool{false, true, false},
	[]bool{false, false, true},
}

var falseTestsOneTrueOf = [][]bool{
	[]bool{true, true},
	[]bool{true, true, false},
	[]bool{false, true, true},
	[]bool{true, true, true},
}

func TestOneTrueOf(t *testing.T) {
	for _, test := range trueTestsOneTrueOf {
		v := OneTrueOf(test...)
		if v != true {
			t.Error("for", test, "expected", true, "got", v)
		}
	}


	for _, test := range falseTestsOneTrueOf {
		v := OneTrueOf(test...)
		if v != false {
			t.Error("for", test, "expected", false, "got", v)
		}
	}
}

var trueTestsNoTrueOf = [][]bool{
	[]bool{false},
	[]bool{false, false},
	[]bool{false, false, false},
}

var falseTestsNoTrueOf = [][]bool{
	[]bool{true},
	[]bool{true, false},
	[]bool{true, false, false},

	[]bool{false, true},
	[]bool{false, true, false},

	[]bool{false, false, true},

	[]bool{true, true},
	[]bool{true, true, false},

	[]bool{false, true, true},
}

func TestNoTrueOf(t *testing.T) {
	for _, test := range trueTestsNoTrueOf {
		v := NoTrueOf(test...)
		if v != true {
			t.Error("for", test, "expected", true, "got", v)
		}
	}

	for _, test := range falseTestsNoTrueOf {
		v := NoTrueOf(test...)
		if v != false {
			t.Error("for", test, "expected", false, "got", v)
		}
	}
}

var trueTestsOneOrNoTrueOf = [][]bool{
	[]bool{true},
	[]bool{true, false},
	[]bool{true, false, false},
	[]bool{false, true},
	[]bool{false, true, false},
	[]bool{false, false, true},

	[]bool{false},
	[]bool{false, false},
	[]bool{false, false, false},
}

var falseTestsOneOrNoTrueOf = [][]bool{
	[]bool{true, true},
	[]bool{true, true, false},

	[]bool{false, true, true},
}

func TestOneOrNoTrueOf(t *testing.T) {
	for _, test := range trueTestsOneOrNoTrueOf {
		v := OneOrNoTrueOf(test...)
		if v != true {
			t.Error("for", test, "expected", true, "got", v)
		}
	}

	for _, test := range falseTestsOneOrNoTrueOf {
		v := OneOrNoTrueOf(test...)
		if v != false {
			t.Error("for", test, "expected", false, "got", v)
		}
	}
}

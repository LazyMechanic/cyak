package assist

import (
	"github.com/otiai10/copy"
	"io/ioutil"
	"os"
)

func OneTrueOf(states ...bool) (result bool) {
	for i, currentState := range states {
		var temp bool = currentState
		for k, state := range states {
			if k != i {
				temp = temp && !state
			}
		}

		result = result || temp
	}

	return result
}

func NoTrueOf(states ...bool) bool {
	var result bool = true
	for _, state := range states {
		result = result && !state
	}

	return result
}

func OneOrNoTrueOf(states ...bool) bool {
	return OneTrueOf(states...) || NoTrueOf(states...)
}

func WriteToDisk(file string, content string) {
	if err := ioutil.WriteFile(file, []byte(content), os.ModePerm); err != nil {
		panic(err)
	}
}

func Copy(from string, to string) {
	if err := copy.Copy(from, to); err != nil {
		panic(err)
	}
}
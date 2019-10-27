package internal

import (
	"log"
	"path/filepath"
)

func CheckAndReturn(str string, err error) string {
	Check(err)
	return str
}

func Check(err error) {
	if err != nil {
		log.Fatal(err)
	}
}


func AbsDir(v string) string {
	abs, err := filepath.Abs(v)
	Check(err)

	return abs
}
package commands

import (
	"fmt"
	"github.com/urfave/cli/v2"
	"os"
	"path/filepath"
)

type exit struct {
	Text string
}

func (e *exit) Error() string {
	return e.Text
}

func onUsageError (c *cli.Context, err error, isSubcommand bool) error {
	if isSubcommand {
		return err
	}

	switch err.(type) {
	case *exit:
		fmt.Fprintf(c.App.Writer, "%#v\n", err)
	default:
		fmt.Fprintf(c.App.Writer, "ERROR: %#v\n", err)
	}

	return nil
}

func isDirExist(path string, joins ...string) bool {
	finalPath := path
	for i, _ := range joins {
		finalPath = filepath.Join(finalPath, joins[i])
	}

	_, err := os.Stat(finalPath)
	return !os.IsNotExist(err)
}

func mkdirIfNotExist(path string, joins ...string) error {
	finalPath := path
	for i, _ := range joins {
		finalPath = filepath.Join(finalPath, joins[i])
	}

	if !isDirExist(finalPath) {
		return os.MkdirAll(finalPath, os.ModePerm)
	}
	return nil
}
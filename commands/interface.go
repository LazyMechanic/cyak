package commands

import "github.com/urfave/cli/v2"

type Command interface {
	CliCommand() *cli.Command
}

type actionFunc func() error
type action struct {
	Option string
	Func   actionFunc
}
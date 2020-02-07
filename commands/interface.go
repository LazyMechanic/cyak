package commands

import "github.com/urfave/cli/v2"

type Command interface {
	CliCommand() *cli.Command
}
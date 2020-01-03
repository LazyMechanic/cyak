package commands

import (
	"github.com/LazyMechanic/cyak/cli/actions"
	"github.com/LazyMechanic/cyak/cli/flags"
	gocli "github.com/urfave/cli"
)

var (
	Create = gocli.Command{
		Name:      "create",
		Aliases:   []string{"c"},
		Usage:     "create cmake files",
		ArgsUsage: "<working-directory>",
		Flags: []gocli.Flag{
			flags.GitFlag,
			flags.PresetFlag,
		},
		Action: actions.Create,
	}
)

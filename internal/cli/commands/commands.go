package commands

import (
	"github.com/LazyMechanic/cyak/internal/cli/actions"
	gocli "github.com/urfave/cli"
)

var (
	Create = gocli.Command{
		Name:      "create",
		Aliases:   []string{"c"},
		Usage:     "create cmake files",
		ArgsUsage: "<working-directory>",
		Flags: []gocli.Flag{
		},
		Action: actions.Create,
	}
)

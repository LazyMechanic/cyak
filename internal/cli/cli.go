package cli

import (
	"fmt"
	"github.com/LazyMechanic/cyak/internal/cli/commands"
	"github.com/LazyMechanic/cyak/internal/config"
	gocli "github.com/urfave/cli"
	"os"
)

func Run() {
	process()
}

func process() {
	fmt.Println(config.PresetsFolder)
	app := gocli.NewApp()
	app.Name = "cyak"
	app.Version = config.Version
	app.Authors = []*gocli.Author{
		&gocli.Author{
			Name:  "LazyMechanic",
			Email: "asharnrus@gmail.com",
		},
	}
	app.Usage = "utility for create cmake project or the only target cmake list file by presets"

	app.Commands = []*gocli.Command{
		&commands.Create,
	}

	err := app.Run(os.Args)
	if err != nil {
		fmt.Println(err)
	}
}

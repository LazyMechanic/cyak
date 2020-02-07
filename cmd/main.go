package main

import (
	"github.com/LazyMechanic/cyak/commands"
	"github.com/urfave/cli/v2"
	"log"
	"os"
	"time"
)

func main() {
	app := &cli.App{
		Name:    "cyak",
		Usage:   "Utility for creating an cmake entire project or cmake  targets in existed project",
		Version: "v0.7",
		Commands: []*cli.Command{
			commands.Create,
			commands.Add,
		},
		Flags:                nil,
		EnableBashCompletion: true,
		HideHelp:             false,
		HideVersion:          false,
		Compiled:             time.Now(),
		Authors: []*cli.Author{
			&cli.Author{
				Name:  "LazyMechanic",
				Email: "AsharnRus.work@gmail.com",
			},
		},
		UseShortOptionHandling: true,
	}

	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}

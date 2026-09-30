package mcpsrv

import (
	"context"
	"os"

	"k3c/tools/k3c-dev/internal/gamedata"
)

type reportIn struct {
	Name string `json:"name" jsonschema:"Dateiname aus reports_list, z. B. gamepad-2026-09-30T19-14-02-123Z.json"`
}

// reportsList ist das Tool reports_list.
func (s *Server) reportsList(context.Context, struct{}) (string, error) {
	return gamedata.ReportsList(s.cfg.Root)
}

// reportRead ist das Tool report_read.
func (s *Server) reportRead(_ context.Context, in reportIn) (string, error) {
	return gamedata.ReportRead(s.cfg.Root, in.Name)
}

// savesList ist das Tool saves_list; der Ordner folgt K3C_SAVES_DIR wie der Heimnetz-Server.
func (s *Server) savesList(context.Context, struct{}) (string, error) {
	return gamedata.SavesList(s.cfg.Root, gamedata.SavesDir(s.cfg.Root, os.Getenv(gamedata.EnvSavesDir)))
}

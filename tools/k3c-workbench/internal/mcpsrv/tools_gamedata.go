package mcpsrv

import (
	"context"
	"os"

	"k3c/tools/k3c-workbench/internal/gamedata"
)

type reportIn struct {
	Server string `json:"server,omitempty" jsonschema:"entfernter Server statt der lokalen Berichte: pi (= pi-gaming:8080), ein Name aus K3C_REMOTE_SERVERS oder host:port im Heimnetz"`
	Name   string `json:"name" jsonschema:"Dateiname aus reports_list, z. B. gamepad-2026-09-30T19-14-02-123Z.json"`
}

type reportsIn struct {
	Server string `json:"server,omitempty" jsonschema:"entfernter Server statt der lokalen Berichte: pi (= pi-gaming:8080), ein Name aus K3C_REMOTE_SERVERS oder host:port im Heimnetz"`
}

// reportsList ist das Tool reports_list.
func (s *Server) reportsList(ctx context.Context, in reportsIn) (string, error) {
	if in.Server != "" {
		return s.remoteReport(ctx, in.Server, "")
	}
	return gamedata.ReportsList(s.ws(ctx).root)
}

// reportRead ist das Tool report_read.
func (s *Server) reportRead(ctx context.Context, in reportIn) (string, error) {
	if in.Server != "" {
		return s.remoteReport(ctx, in.Server, in.Name)
	}
	return gamedata.ReportRead(s.ws(ctx).root, in.Name)
}

// savesList ist das Tool saves_list; der Ordner folgt K3C_SAVES_DIR wie der Heimnetz-Server.
func (s *Server) savesList(ctx context.Context, _ struct{}) (string, error) {
	root := s.ws(ctx).root
	return gamedata.SavesList(root, gamedata.SavesDir(root, os.Getenv(gamedata.EnvSavesDir)))
}

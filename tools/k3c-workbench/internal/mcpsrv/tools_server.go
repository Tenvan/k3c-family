package mcpsrv

import (
	"context"

	"k3c/tools/k3c-workbench/internal/serverapi"
)

type roomIn struct {
	Room string `json:"room" jsonschema:"Raum-Code aus rooms_list, z. B. FAMILIE"`
}

type statusIn struct {
	Server string `json:"server,omitempty" jsonschema:"entfernter Server statt des lokalen Spielservers: pi (= pi-gaming:8080), ein Name aus K3C_REMOTE_SERVERS oder host:port im Heimnetz"`
}

// serverStatus ist das Tool server_status; mit server liest es /api/dev/status eines entfernten Servers.
func (s *Server) serverStatus(ctx context.Context, in statusIn) (string, error) {
	if in.Server != "" {
		return s.remoteStatus(ctx, in.Server)
	}
	c, err := s.serverClient(ctx)
	if err != nil {
		return "", err
	}
	st, err := c.Status(ctx)
	if err != nil {
		return "", err
	}
	return serverapi.FormatStatus(st), nil
}

// roomsList ist das Tool rooms_list.
func (s *Server) roomsList(ctx context.Context, _ struct{}) (string, error) {
	c, err := s.serverClient(ctx)
	if err != nil {
		return "", err
	}
	st, err := c.Status(ctx)
	if err != nil {
		return "", err
	}
	return serverapi.FormatRooms(st.Rooms), nil
}

// roomSnapshot ist das Tool room_snapshot.
func (s *Server) roomSnapshot(ctx context.Context, in roomIn) (string, error) {
	c, err := s.serverClient(ctx)
	if err != nil {
		return "", err
	}
	sum, err := c.Room(ctx, in.Room)
	if err != nil {
		return "", err
	}
	return serverapi.FormatSummary(sum), nil
}

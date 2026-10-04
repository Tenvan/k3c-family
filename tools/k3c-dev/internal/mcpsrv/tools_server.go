package mcpsrv

import (
	"context"

	"k3c/tools/k3c-dev/internal/serverapi"
)

type roomIn struct {
	Room string `json:"room" jsonschema:"Raum-Code aus rooms_list, z. B. FAMILIE"`
}

// serverStatus ist das Tool server_status.
func (s *Server) serverStatus(ctx context.Context, _ struct{}) (string, error) {
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

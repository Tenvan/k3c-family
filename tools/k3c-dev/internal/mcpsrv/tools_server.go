package mcpsrv

import (
	"context"
	"os"

	"k3c/tools/k3c-dev/internal/serverapi"
)

type roomIn struct {
	Room string `json:"room" jsonschema:"Raum-Code aus rooms_list, z. B. FAMILIE"`
}

// serverClient liest Adresse und Token bei jedem Aufruf aus der Umgebung (wie der Server: K3C_STATUS_TOKEN, K3C_SERVER_URL, K3C_HTTP_PORT).
func serverClient() *serverapi.Client { return serverapi.FromEnv(os.Getenv) }

// serverStatus ist das Tool server_status.
func (s *Server) serverStatus(ctx context.Context, _ struct{}) (string, error) {
	st, err := serverClient().Status(ctx)
	if err != nil {
		return "", err
	}
	return serverapi.FormatStatus(st), nil
}

// roomsList ist das Tool rooms_list.
func (s *Server) roomsList(ctx context.Context, _ struct{}) (string, error) {
	st, err := serverClient().Status(ctx)
	if err != nil {
		return "", err
	}
	return serverapi.FormatRooms(st.Rooms), nil
}

// roomSnapshot ist das Tool room_snapshot.
func (s *Server) roomSnapshot(ctx context.Context, in roomIn) (string, error) {
	sum, err := serverClient().Room(ctx, in.Room)
	if err != nil {
		return "", err
	}
	return serverapi.FormatSummary(sum), nil
}

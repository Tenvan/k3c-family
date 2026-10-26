package mcpsrv

import (
	"context"
	"fmt"
	"time"

	"k3c/tools/k3c-workbench/internal/simrun"
)

// B-644: sim_test wartet wie check_run (check_wait.go) auf das Ende des Laufs: `start` mit wait und `wait {id}`.
// Mit progressToken meldet der Aufruf alle progressEvery ein Lebenszeichen und wartet bis zum Ende; ohne höchstens
// checkWait und nennt danach die Lauf-ID. Der Lauf selbst überlebt den Aufruf wie immer.

// simPoll ist der Abstand, in dem der wartende Aufruf den Stand des Laufs liest.
const simPoll = 250 * time.Millisecond

// simEnded: Der Lauf rechnet nicht mehr und wartet nicht auf einen Platz.
func simEnded(state string) bool { return state != simrun.Running && state != simrun.Waiting }

// awaitSim wartet auf das Ende des Laufs id und liefert den Status; der Kontext des Aufrufs beendet nur das Warten.
func (s *Server) awaitSim(ctx context.Context, root, id string) (string, error) {
	progress := progressOf(ctx)
	deadline := time.NewTimer(s.checkWait)
	defer deadline.Stop()
	if progress != nil {
		deadline.Stop() // mit Lebenszeichen wartet der Aufruf bis zum Ende (der Lauf endet spätestens mit seinem Zeitlimit)
	}
	poll, beat := time.NewTicker(simPoll), time.NewTicker(s.progressEvery)
	defer poll.Stop()
	defer beat.Stop()
	for {
		info, err := s.sims.Get(root, id)
		if err != nil {
			return "", fmt.Errorf("lauf %q unbekannt in diesem Checkout; sim_test list zeigt die Läufe", id)
		}
		if simEnded(info.State) {
			return info.Status(time.Now(), maxStatusLines), nil
		}
		select {
		case <-poll.C:
		case <-beat.C:
			if progress != nil {
				elapsed := time.Since(info.Started)
				progress(fmt.Sprintf("%s %s seit %s", info.ID, info.State, elapsed.Truncate(time.Second)), elapsed)
			}
		case <-deadline.C:
			return fmt.Sprintf("%s läuft noch seit %s · weiter warten mit sim_test {action: wait, id: %q}\n%s", info.ID,
				time.Since(info.Started).Truncate(time.Second), info.ID, info.Status(time.Now(), maxStatusLines)), nil
		case <-ctx.Done():
			return "", ctx.Err()
		}
	}
}

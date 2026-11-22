package mcpsrv

import "time"

// Test-Naht der Testläufe (WZ63.5, aus check_run der Workbench übernommen): In der Produktion rechnet simrun im Prozess
// (B-924) und s.run bleibt nil; die Tests lassen die Offline-Läufe von sim_test über s.run laufen (simsViaRun in
// simtest_test.go) und prüfen so den Weg Tool → Register ohne Rechnung der Engine.

// runSpec ist ein Prozessaufruf eines Testlaufs (Ordner, Argumente, Zeitlimit, Ausgabe).
type runSpec struct {
	dir     string
	args    []string
	timeout time.Duration
	out     func(stream, text string)
}

// runResult ist das Ergebnis eines Prozessaufrufs.
type runResult struct {
	exit     int
	timedOut bool
	err      error // Start gescheitert
}

// Warten von sim_test wie check_run der Workbench (B-644): mit progressToken alle defaultProgressEvery ein
// Lebenszeichen, ohne höchstens defaultCheckWait, knapp unter der Leerlauf-Frist von HTTP-MCP-Clients (5 min).
const (
	defaultCheckWait     = 4 * time.Minute
	defaultProgressEvery = 20 * time.Second
)

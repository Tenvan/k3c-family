// Command k3c-sim ist das Engine-CLI (B-726, B-924): Simulation, Balancing, Level- und Welten-Rechnung. Die Logik steht
// in engine/simcli, das auch die Workbench im Prozess aufruft; diese Hülle ist die EXE für Taskfile und CI (bin/k3c-sim,
// `task k3c-sim:build`, kein go run).
//
// Schnittstelle (RS2.2 baut die Workbench darauf um):
//
//	k3c-sim run     [--input DATEI|-]          JSON {"biome","seed","ticks","inputs":[Segment…]} wie sim_run
//	k3c-sim level   [--input DATEI|-]          JSON {"biome","seed"} wie level_generate
//	k3c-sim balance [--input DATEI|-] [Flags]  Flags wie bis RS2.2 bin/k3c-balance (--targets, --report-dir, --seed-list …)
//	k3c-sim world   [--input DATEI|-]          JSON {"op":"level"|"values"|"check",…} für den Welten-Editor (world.go, B-879)
//	k3c-sim hub     [--input DATEI|-]          JSON {"op":"plan","raw":…} für den Gebäude-Planer (hub.go, B-923)
//
// Eingabe: JSON aus der Datei hinter --input, bei run, level, world und hub ohne --input von stdin („-“ = stdin). Bei balance ist
// das JSON ein Objekt mit Flag-Namen als Schlüssel ({"targets": true, "seed-list": "k3c-dev", "vary": ["a=1"]});
// Flags in der Befehlszeile stehen danach: Einfache Werte gewinnen, wiederholbare (--vary, --param) kommen hinzu.
//
// Ausgabe auf stdout: nur JSON-Zeilen, je Zeile ein Objekt mit "event":
//
//	{"event":"progress","done":3,"total":6}          nach jedem fertigen Lauf (balance --targets --progress, auch --tune)
//	{"event":"result","text":"…"}                     Ergebnis von run und level (Text wie sim_run/level_generate)
//	{"event":"result","report":{…}}                   balance ohne --targets und ohne --out (Matrix-Bericht, --play); world
//	{"event":"report","files":["…json","…md","…html"]} balance --targets: geschriebene Berichtsdateien
//	{"event":"error","message":"…"}                   vor Exit 1 oder 2
//	{"event":"aborted"}                               vor Exit 130
//
// Berichte schreibt balance nach --report-dir (Workbench: reports/simtest-<id>/), Dateien und Inhalt wie bis RS2.2 bin/k3c-balance.
// Meldungen für Menschen gehen nach stderr. Exit-Codes: 0 geschafft, 1 Lauf gescheitert, 2 Aufruf falsch (Unterbefehl,
// Flags, JSON), 130 abgebrochen (Strg+C, CTRL_BREAK, SIGTERM). Abbruch heißt Prozess-Ende: Die Workbench beendet den
// Prozess (Job-Objekt wie engine/devrun); k3c-sim hat keine Kindprozesse. Berichtsdateien entstehen erst nach der
// Rechenphase; ein Ende genau beim Schreiben kann einen unvollständigen Satz (.json ohne .md/.html) hinterlassen.
package main

import (
	"context"
	"fmt"
	"io"
	"os"
	"os/signal"
	"sync/atomic"
	"syscall"

	"k3c/engine/simcli"
)

func main() {
	stop := make(chan os.Signal, 1)
	signal.Notify(stop, os.Interrupt, syscall.SIGTERM)
	out := simcli.NewEmitter(os.Stdout)
	var finished atomic.Bool
	go func() {
		<-stop
		onAbort(out, os.Stderr, &finished, os.Exit)
	}()
	code := simcli.Exec(context.Background(), os.Args[1:], os.Stdin, os.Stderr, out, nil)
	finished.Store(true)
	os.Exit(code)
}

// onAbort meldet den Abbruch (Zeile aborted, Exit 130), außer der Lauf ist schon fertig: Dann gilt dessen Exit-Code.
func onAbort(out *simcli.Emitter, stderr io.Writer, finished *atomic.Bool, exit func(int)) {
	if finished.Load() {
		return
	}
	out.Emit(map[string]string{"event": "aborted"})
	_, _ = fmt.Fprintln(stderr, "🛑 k3c-sim: abgebrochen")
	exit(simcli.ExitAborted)
}

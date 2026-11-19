# B-1006 · Gehaltenes A am Steg zahlt nach der Zielwahl erst wieder nach dem Loslassen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU61
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Seekarte am Hafen (B-962, WE4.2) wählt das Ziel mit A. A ist im Spiel auch Bezahlen. `native/k3c-game/hud/seekarte_input.go` › `updateSea` verbraucht die Eingabe des Spielers nur in dem Update, in dem er bestätigt (`Overlay.Consumed`, `native/k3c-game/hud/overlay.go` Zeile 25–27, ausgewertet in `native/k3c-game/game/play.go`). Hält der Spieler A weiter, geht die Eingabe ab dem Folgetakt wieder an die Simulation und er zahlt am Steg (Restbefund aus WE4.2 › Ergebnis: „gehaltenes A zahlt ab dem Folgetakt wieder“).

## Ziel

Eine Zielwahl auf der Seekarte kostet nie versehentlich Münzen: Nach dem Bestätigen zählt A erst wieder als Bezahlen, wenn der Spieler die Taste losgelassen hat.

## Beteiligte und Zielgruppen

Spielende am Hafen (1 bis 4, je eigenes Gerät); Agent CLI; 🧑 gibt frei.

## Anforderungen

- Nach einem Bestätigen am Steg (Wahl angenommen oder abgelehnt) bleibt die Eingabe dieses Spielers verbraucht, solange er A ohne Unterbrechung hält; mit dem Loslassen endet die Sperre.
- Die Sperre gilt je Spieler: Der Mitspieler am selben Steg zahlt und wählt unabhängig weiter.
- Kein Zustand in der Simulation, kein Spielstand-Feld; die Simulation bleibt deterministisch (die Sperre hängt nur an der Eingabe-Folge).
- Keine Allokation je Update (wie `hud/seekarte_test.go` heute prüft).
- Die Sperre endet auch, wenn der Spieler den Steg verlässt, die Krone verliert oder sein Gerät getrennt wird.

## Nicht-Ziele

Andere Taste für die Seekarte; Änderungen am Bezahlen selbst (`engine/sim`); Skill-Menü.

## Regeln und Einschränkungen

Das Spiel zeichnet und bedient nur, Regeln bleiben in `rust/crates/k3c-sim`. B bleibt unbelegt. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Agent `k3c-hud` prüft vor dem Commit.

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game` (Zielwahl am Steg liegt seit RU53 im HUD); die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013). Vor Beginn mit einem Test prüfen, ob der Fehler im Rust-Spiel auftritt.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/game/**`
- **Abhängig von:** B-962
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Spieler 1 steht am Steg, blättert zum Ziel, drückt A und hält 2 s → Ziel gewählt, sein Gold bleibt gleich. Er lässt los und hält A erneut → er zahlt wie überall.

## Ausnahme- und Fehlerfälle

Wahl abgelehnt (`ChooseIsland` liefert Fehler) → Sperre gilt genauso. Spieler 2 hält A am Steg, während Spieler 1 bestätigt → Spieler 2 zahlt weiter. Seekarte nicht offen → A zahlt wie immer.

## Akzeptanzkriterien

- **AC-01** Test in `native/k3c-game/hud`: Nach dem Bestätigen meldet `Update` für diesen Spieler `Consumed`, solange A gehalten ist, und nach einem Update ohne A nicht mehr.
- **AC-02** Test mit 2 Spielern: Die Sperre von Spieler 1 verbraucht die Eingabe von Spieler 2 nicht.
- **AC-03** Test: 0 Allokationen je Update mit aktiver Sperre.

## Offene Fragen

- Sperre bis zum Loslassen oder feste Zeit? → Default: bis zum Loslassen von A, weil es ohne Zahl auskommt und dem Menü-Verhalten entspricht (vorläufig, 🧑 bestätigt).

## Notizen

Folge aus B-962 (WE4.2 › Ergebnis). Fundorte: `hud/seekarte_input.go`, `hud/overlay.go`, `hud/seekarte.go` › `step`, `game/play.go` › `overlays`.

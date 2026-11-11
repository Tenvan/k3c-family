# B-793 · Das native Spiel rechnet mit der Go-Simulation im selben Prozess und speichert Spielstände lokal

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NG1
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Im Server taktet `engine/room` je Raum die Simulation (`Room.Tick`, 30 Hz), nimmt Befehle je Gerät (`Room.Input`, `sim.PlayerCommand`), speichert über einen `Store` und gibt den Zustand je Stufe über `Manager.Snapshot` an `Peer.State`. `engine/net` setzt `Peer` für WebSocket um und baut den Protokoll-Zustand (`stateOf`). `engine/room` kennt kein Netz. Offene Frage aus B-775: Bindet der Monolith `engine/room` ohne Netz oder direkt `engine/sim` ein?

## Ziel

Paket `native/k3c-game/local` führt ein Spiel mit 1–2 lokalen Spielern im selben Prozess, mit Inseln, Stufen, Pause und Spielständen wie der Server, und gibt je Takt einen `view.Frame` an das Zeichnen.

## Beteiligte und Zielgruppen

Claude (Umsetzung), 🧑 Ralf (Entscheidung über den Speicherort).

## Anforderungen

- `engine/room` mit einem lokalen `Peer` (ohne Netz), ohne `Manager.Run`: `game` ruft `Room.Tick` synchron im Takt der Simulation (30 Hz) auf; damit laufen Simulation und Zeichnen in einer Goroutine.
- `Manager.Snapshot` liefert den `view.Frame` mit `*sim.World` zum Lesen; keine Kopie und kein JSON.
- Befehle je Spieler (`view`-Eingabe-Typen → `sim.PlayerCommand`) über `Room.Input`; 1–2 Slots eines lokalen Geräts.
- Spielstände über `engine/store` in einem Ordner: Neues Spiel, Weiter, Laden, Liste, Löschen, automatisches Speichern im Takt des Servers; erfüllt `view.Session`.
- Deterministisch: gleicher Seed und gleiche Befehle ergeben dieselbe Welt wie ein Raum ohne native Schicht.

## Nicht-Ziele

Änderungen unter `engine/` (Abweichungen werden Ticket der Domäne SRV oder SIM); Online-Spiel, Beobachter, Dev-Aktionen, Welten-Editor; Migration alter Spielstände.

## Regeln und Einschränkungen

Entscheidung 008: Die Go-Simulation bleibt unverändert, `testdata/golden/` bleibt grün. Spielstand-Format unverändert (keine Sperre `spielstand`). Kein `math/rand`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/local/**`
- **Abhängig von:** NV3
- **Braucht 🧑:** Freigabe, Entscheidung
- **Größe:** M
- **Sperre:** –

## Beispiele

Neues Spiel mit Seed `wald-1`, zwei Spieler halten 300 Takte rechts → Monarch-Positionen und Welt-Hash gleich wie bei einem `engine/room`-Raum im Test mit denselben Befehlen. Spiel beenden und „Weiter“ → derselbe Stand.

## Ausnahme- und Fehlerfälle

Speicherordner nicht beschreibbar → Meldung, Spiel läuft ohne Speichern weiter. Spielstand kaputt oder alte Version → in der Liste als nicht ladbar, kein Absturz. Raum stürzt ab (Panic im Takt) → Meldung im Menü, letzter Spielstand bleibt.

## Akzeptanzkriterien

- **AC-01** Ein Test startet über `local` ein Spiel mit 2 Spielern, taktet 300 Mal und vergleicht den Welt-Hash mit einem direkt über `engine/room` getakteten Raum mit denselben Befehlen: gleich.
- **AC-02** Speichern, Liste, Laden und Löschen laufen im Test gegen einen Temp-Ordner; nach Laden ist der Welt-Hash gleich.
- **AC-03** `git diff` des Sprints ändert nichts unter `engine/` und `testdata/golden/`.

## Offene Fragen

keine.

Entschieden von 🧑 am 2026-11-11 (Chat):

- **Speicherort:** `saves/` neben der EXE, per Schalter `--saves` änderbar.
- **Takt und Bildrate:** Simulation 30 Hz ohne Zwischenbilder; ruckelt es sichtbar, gibt es ein eigenes Ticket.

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG1). `Room.Tick` und `Manager.Snapshot` sind exportiert (`engine/room/actions.go`, `engine/room/manager.go`); dass ein Raum ohne `Manager.Run` vollständig läuft (Autosave, Fristen), ist ungeprüft und klärt NG1.2.

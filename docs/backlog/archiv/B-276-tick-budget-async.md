# B-276 · Der Raum-Tick bleibt im Budget, Kodierung und Speichern laufen außerhalb der Raum-Sperre

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** N1
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, mit N1

## Ausgangslage

Das Spiel lagt stark. Das Server-Log (`k3c-server`, Raum BUCU, 1 Gerät, 2026-10-03) meldet laufend `🐢 Tick zu langsam`: 35–187 ms, Spitze 685 ms bei 33 ms Budget (30 Hz), p99 37–56 ms, obwohl der Mittelwert nur 3–5 ms ist. Ursachen im Code (Anteil je Ursache ungeprüft):

- `Room.Tick` (`engine/room/actions.go`) hält die Raum-Sperre über Simulation **und** Versand: `pushState` → `conn.State` (`engine/net/ws.go`) baut je Gerät mit `stateOf` (`engine/net/protocol.go`: `json.Marshal` + `json.Unmarshal` in `map[string]any`), vergleicht per `reflect.DeepEqual` (`delta.go`) und kodiert erneut mit `json.Marshal`. Das erzeugt je Tick und Gerät viel Müll (GC-Pausen) und wächst mit Geräten und Plätzen (B-263).
- `Tick` ruft bei Stufenwechsel `r.save()` (`engine/room/room.go`): `json.Marshal` des Spielstands **und** Schreiben der Datei unter der Raum-Sperre.
- Der `time.Ticker` in `run.go` verwirft Ticks, wenn einer zu lange dauert: Zustände kommen unregelmäßig, der Client bleibt stehen (keine Extrapolation).

## Ziel

Der Raum-Tick hält sein Budget auch mit mehreren Geräten: Unter der Raum-Sperre laufen nur noch die Simulation und das einmalige Erfassen des Zustands je Stufe; Delta, JSON und Datei-Schreiben laufen asynchron in eigenen Goroutinen.

## Beteiligte und Zielgruppen

Spieler (flüssiges Spiel), Betrieb auf dem Pi; SRV setzt um, 🧑 prüft am Gerät.

## Anforderungen

- Der Zustand (`stateOf`) wird je Tick **einmal je Stufe** gebaut, nicht je Gerät; Geräte derselben Stufe teilen ihn (unveränderlich).
- Delta zum zuletzt gesendeten Zustand und `json.Marshal` laufen in der Schreib-Goroutine der Verbindung, nicht unter der Raum-Sperre. Reihenfolge der Nachrichten je Verbindung bleibt erhalten (Level vor snap, snap vor delta).
- Kommt eine Verbindung nicht hinterher, verwirft sie veraltete Zustände (nur der neueste zählt, der nächste wird ein Delta zum zuletzt **gesendeten**) statt den Sendepuffer zu füllen und getrennt zu werden.
- Speichern: Unter der Sperre nur `json.Marshal` des Spielstands; das Schreiben der Datei läuft in einer Goroutine je Raum, Speichervorgänge desselben Raums nacheinander, der neueste gewinnt. `SaveNow` und Speichern beim Schließen/Herunterfahren bleiben synchron (Fehler wird gemeldet, nichts geht verloren).
- Protokoll und Spielstand-Format bleiben unverändert (keine Versionsänderung).

## Nicht-Ziele

Protokolländerung (Binärformat, Kompression, kleinere Plätze: B-263, B-208); Client-Darstellung (B-277); Leistungsziel Pi (B-042).

## Regeln und Einschränkungen

Domäne SRV. `engine/sim/` unverändert, deterministisch. `go test -race` grün. Datei ≤ 400 Zeilen, Funktion ≤ 60. Logging mit Emoji.

## Beispiele

Zwei Geräte auf Stufe 1 → ein `stateOf` je Tick, je Verbindung ein eigenes Delta in der Schreib-Goroutine. Stufenwechsel → Tick kehrt sofort zurück, die Datei wird kurz danach geschrieben.

## Ausnahme- und Fehlerfälle

Langsame Verbindung → veraltete Zustände verworfen, keine Trennung wegen Zuständen im vollen Puffer. Speichern scheitert asynchron → `💥`-Log wie bisher. Herunterfahren/Schließen → laufende Hintergrund-Speicherung wird abgewartet, letztes Speichern synchron.

## Akzeptanzkriterien

- **AC-01** Test: Zwei Geräte auf derselben Stufe lösen je Tick genau einen Aufbau des Zustands aus (`go test ./engine/...`).
- **AC-02** Test: Ein Gerät, dessen Schreib-Goroutine blockiert, wird nicht getrennt; danach erhält es einen gültigen Zustand (Delta zum zuletzt gesendeten), der Client-Zustand nach Anwenden entspricht dem Server-Zustand.
- **AC-03** Test: `Tick` schreibt keine Datei unter der Raum-Sperre; ein Stufenwechsel speichert trotzdem (Datei nach Abschluss der Goroutine vorhanden), `SaveNow` bleibt synchron.
- **AC-04** Benchmark `Tick` mit 4 Geräten auf einer Stufe: Zeit und Allokationen je Tick vorher/nachher im Session-Ergebnis, nachher deutlich kleiner.
- **AC-05** `task check:go` grün, Golden-Fixtures des Protokolls unverändert.

## Offene Fragen

keine

## Notizen

Angelegt 2026-10-04 aus dem /team-Auftrag „Netzwerk lagt“; Befund aus `logs_query` (k3c-server).

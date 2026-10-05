# N1 · SRV · Raum-Tick im Budget: Versand und Speichern asynchron

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-276
- **Start-Commit:** 33ae378
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf, /team-Auftrag: bessere asynchrone Übertragung, Client wartet nicht, UI flüssig), Revision 1, aus dem Auftrag abgeleitet; umfasst B-276

## Ausgangslage

Der Raum-Tick reißt sein 33-ms-Budget laufend (Spitzen bis 685 ms, B-276 › Ausgangslage), weil Zustandsaufbau, Delta, JSON und Speichern unter der Raum-Sperre laufen. Das Spiel lagt stark.

## Ziel

Unter der Raum-Sperre laufen nur Simulation und ein Zustandsaufbau je Stufe; Versand und Speichern sind asynchron. Am Ende sichtbar: Benchmark-Zahlen vorher/nachher im Session-Ergebnis, weniger `🐢 Tick zu langsam` im Log.

## Beteiligte und Zielgruppen

Spieler (flüssiges Spiel), Betrieb auf dem Pi; SRV-Agent setzt um, 🧑 prüft am Gerät.

## Anforderungen

`B-276 › Anforderungen`.

## Nicht-Ziele

Protokolländerung (Binärformat, Kompression, kleinere Plätze: B-263, B-208); Client-Darstellung (N2, B-277); Leistungsziel Pi (B-042).

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`, `engine/net/`, `engine/store/`). Protokoll (`docs/protocol.md`, `testdata/protocol/`) und Spielstand-Format unverändert. Simulation deterministisch, `engine/sim/` wird nicht geändert. `go test -race` muss grün sein (neue Goroutinen). Datei ≤ 400 Zeilen, Funktion ≤ 60. Logging mit Emoji.

## Beispiele

Zwei Geräte auf Stufe 1 → ein `stateOf` je Tick, je Verbindung ein eigenes Delta in der Schreib-Goroutine. Stufenwechsel → Tick kehrt sofort zurück, die Datei wird kurz danach geschrieben.

## Ausnahme- und Fehlerfälle

Langsame Verbindung → veraltete Zustände verworfen, keine Trennung wegen Zuständen im vollen Puffer. Speichern scheitert asynchron → `💥`-Log wie bisher. Herunterfahren/Schließen → laufende Hintergrund-Speicherung wird abgewartet.

## Akzeptanzkriterien

- **AC-01** B-276/AC-01
- **AC-02** B-276/AC-02
- **AC-03** B-276/AC-03
- **AC-04** B-276/AC-04
- **AC-05** B-276/AC-05

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| N1.1 | `N1.1-versand-asynchron.md` | Umsetzung | autonom | offen |
| N1.2 | `N1.2-speichern-asynchron.md` | Umsetzung | autonom | offen |
| N1.3 | `N1.3-review.md` | Review | autonom | offen |

## Abnahme

–

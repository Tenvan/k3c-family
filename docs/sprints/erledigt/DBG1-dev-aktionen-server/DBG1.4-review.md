# DBG1.4 · Review und Abnahme des Sprints DBG1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Branch:** dbg1/4-review
- **Abhängig von:** DBG1.3
- **Tickets:** B-178
- **Kriterien:** alle

## Ziel

Der Sprint DBG1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-178 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `engine/room/` (`dev.go`, `actions.go`, `stages.go`, `logging.go`), `engine/net/` (`protocol.go`, `dispatch.go`, `ws.go`), die neue Datei `engine/sim/dev.go`, `docs/protocol.md`, `testdata/protocol/` und den Client-Parser (`src/online/clientProtocol.ts`, `clientConnection.ts`, Tests). Besonders prüfen:

- Sicherheit: `dev` wirkt **nur** bei `Manager.Dev`; die Prüfung kommt vor jeder Feldprüfung; mit `K3C_DEV=0` ist jede Aktion `forbidden` und im Log gewarnt (B-098: Vor dem Release ist der Dev-Mode aus). Kein Weg an der Prüfung vorbei (Gerät ohne Raum, ersetzte Verbindung, fremder Slot).
- Eingaben von außen: `amount` 1…1000, `resource` aus der Liste, `factor` nur 1, 2, 4, 8, `slot` nur eigene Slots; kein Absturz bei fehlenden oder falsch getypten Feldern (`Slot` ist ein Zeiger).
- Spiel-Logik deterministisch: `engine/sim/dev.go` ruft kein `rng` und kein `math/rand` auf; der Zeitraffer ist ein Schleifen-Aufruf von `StepIsland` mit `1/TickHz`, kein anderes `dt`; ohne Dev-Mode und mit Faktor 1 ist das Verhalten gleich dem vor dem Sprint (Golden-Läufe in `testdata/golden/` unverändert); mit 2 Spielern richtig.
- Die Domänen-Ausnahmen sind klein und von 🧑 mit der Spec-Freigabe bestätigt oder als Ticket festgehalten: `engine/sim/dev.go` (nur zwei Funktionen) und der Client-Parser. Sonst `engine/sim/` unverändert (`git diff --stat <Start-Commit>..origin/develop -- engine/sim`).
- Beim Pausieren endet der Zeitraffer; ein Raum ohne Gerät rechnet nicht weiter.
- Bestehende Protokoll-Beispiele sind unverändert, `docs/protocol.md` und Beispiele passen zum Code, kein Test wurde gelockert; Dateien ≤ 400, Funktionen ≤ 60 Zeilen.
- Die Festlegungen aus den Sessions, die als Vorschlag gekennzeichnet waren (Beträge 1…1000, `forbidden` vor Feldprüfung, Feld `devTimescale` nur im Dev-Mode, `events` im Zeitraffer, Münzen ohne Sperre am Spieler, `Peer.State` mit drittem Argument), stehen im Ergebnis der Sessions und sind von 🧑 bestätigt oder als Ticket festgehalten.

## Erlaubte Dateien

- `engine/room/`, `engine/net/`, `engine/sim/dev.go`, `docs/protocol.md`, `testdata/protocol/`, `src/online/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Umbau, Bedienung im Client (DBG2), Dev-Tasten und Neustart (B-080), Debug-Panel (B-107).

## Schritte

1. Branch anlegen und pushen, `Status: in Arbeit`. `task check` und `task check:go` grün.
2. `git fetch && git diff <Start-Commit>..origin/develop` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln (in der Domäne beheben, außerhalb Ticket).
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen der Sessions DBG1.1 bis DBG1.3 prüfen; das Log-Verhalten (AC-06) an den Log-Tests belegen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben (Datum, Kriterien mit Verweis, behobene Befunde, neue Tickets).
5. B-178 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“); `docs/roadmap.md` anpassen. B-179 (DBG2) vermerkt, dass das Protokoll steht.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt.
- [x] `task check` und `task check:go` grün; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

2026-10-03, Review-Agent (Claude Opus 5.5, nicht an DBG1.1–DBG1.3 beteiligt), Branch `dbg1/4-review`.

- `task check:go` grün (golangci-lint 0 issues); `task check` grün nach dem Abschluss (vorher nur `planning.test.ts` wegen „in Arbeit“).
- Diff `f6bb3c3..origin/develop` gelesen (nur DBG1-Dateien; die Testdateien von F2 in `engine/sim`, `engine/level`, `engine/room` gehören nicht zu DBG1). **Keine schweren Befunde.**
- Sicherheit: `Room.Dev` prüft `Manager.Dev` vor jeder Feldprüfung und vor der Sperre, Ablehnung als Warnung im Log; danach `own()` (Gerät verbunden, gleiche Verbindung, sonst `bad_request`), `slot` nur aus `d.slots`, `Slot` als Zeiger (fehlt → `bad_request`), `amount` 1…1000, `resource` über `addStockCapped`, `factor` nur 1, 2, 4, 8.
- Determinismus: `engine/sim/dev.go` nutzt kein `rng`, Münz-IDs über `newID`; der Zeitraffer ruft `StepIsland` `scale()`-mal mit `1/TickHz`, `r.tick++` einmal; `testdata/golden/` unverändert; Produktionscode in `engine/sim` nur `dev.go`. Pause (`connected() == 0`) setzt den Faktor auf 1 zurück, ein Raum ohne Gerät tickt nicht. `engine/room` liest keine `events`, der Verlust der Zwischenereignisse im Zeitraffer betrifft nur die Anzeige (mit der Freigabe bestätigt).
- Kriterien: AC-01, AC-05 in DBG1.1; AC-02, AC-03, AC-06 (Log, `TestDevLog`) in DBG1.2; AC-04 in DBG1.3. Die Vorschläge der Sessions deckt die Freigabe der Spec ab („umfasst B-178 und die Vorschläge der Sessions“).

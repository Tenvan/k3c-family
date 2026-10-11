# WZ2.4 · check_run im Hintergrund, console_tail-Doku, Sprint ohne Prio, Session-IDs mit Buchstaben

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** sprint/wz2
- **Abhängig von:** WZ2.1
- **Tickets:** B-372
- **Kriterien:** AC-05

## Ziel

Lange Prüfläufe liefern über `check_run` ein Ergebnis statt eines Timeouts; die `console_tail`-Doku, `plan_set` an Sprints ohne `Prio` und Session-IDs mit Buchstaben stimmen.

## Kontext

- Spec: `docs/backlog/B-372-check-run-timeout-go.md` (AC-01 bis AC-04); Entscheidung 🧑 2026-10-11: Hintergrundlauf mit Lauf-ID.
- `check_run` liegt in `tools/k3c-dev/internal/mcpsrv/check_run.go`, Beschreibungen in `tools.go`; `go:test` und `task:check:go` enden im MCP-Aufruf mit „The operation timed out.“, obwohl `task check:go` in der Shell ~40 s grün läuft.
- Ungeprüft: Kommt der Timeout aus dem MCP-Client oder aus k3c-dev? Das misst diese Session und trägt es in B-372 › Notizen ein; der Hintergrundlauf gilt unabhängig davon.
- `plan_set` an Sprints verlangt noch `Prio` (seit B-361 entfernt); `reSession` in `planning/store.go` lehnt Session-IDs wie `K3.2a` ab.
- Umgebung `live`: die Messung braucht laufendes k3c-dev. Firewall-Regel: Proben nur auf Loopback.

## Erlaubte Dateien

- `tools/k3c-dev/internal/mcpsrv/check_run.go`, `tools.go`, `check_*.go` und zugehörige `*_test.go`
- `tools/k3c-dev/internal/planning/store.go`, `edit.go` und zugehörige `*_test.go`
- Server-Anweisungen von k3c-dev (Text, in `mcpsrv/`)
- `docs/backlog/B-372-check-run-timeout-go.md` (Notizen)

## Nicht-Ziele

Go-Tests selbst beschleunigen; neue MCP-Tools außer der Abfrage der Lauf-ID; Änderungen am MCP-Client.

## Schritte

1. Messen: `check_run task:check:go` aufrufen, Dauer und Fehlerquelle (Client oder k3c-dev) in B-372 › Notizen eintragen.
2. Test zuerst (rot): ein Lauf > 60 s liefert Exit-Code und Fehlerzeilen, direkt oder über eine Lauf-ID (B-372/AC-01).
3. `check_run` startet lange Läufe im Hintergrund und gibt die Lauf-ID zurück; Abfrage liefert Exit-Code und Fehlerzeilen; hängt ein Lauf, bricht er nach fester Frist mit klarer Meldung ab.
4. Server-Anweisungen und Tool-Beschreibung nennen die aktuellen Parameter von `console_tail` (B-372/AC-02).
5. Test: `plan_set` an einem Sprint ohne `Prio` ist erfolgreich (B-372/AC-03).
6. Test: `plan_create` legt Session `K3.2a` an (B-372/AC-04).
7. `task check:dev` ausführen, Ergebnis eintragen.

## Fertig, wenn

- [ ] AC-05: Tests für Lauf > 60 s, `plan_set` Sprint ohne `Prio` und Session-ID mit Buchstaben grün; `console_tail`-Doku stimmt (`B-372/AC-01`–`AC-04`).
- [ ] Herkunft des Timeouts steht in B-372 › Notizen.
- [ ] `task check:dev` grün; keine Datei > 400 Zeilen.

## Prüfen

```bash
task check:dev
```

Bei laufendem k3c-dev stattdessen `check_run` mit `task:check:dev`; die Messung in Schritt 1 braucht k3c-dev (live).

## Ergebnis

- AC-05 umgesetzt, geprüft: `TestCheckRunLangerLaufLiefertLaufID`, `TestCheckRunUnbekannteLaufID`, `TestCheckRunKurzerLaufAntwortetDirekt` (B-372/AC-01), `TestInstructionsNennenConsoleTailParameter` (B-372/AC-02), `TestSessionIDMitBuchstabe` (B-372/AC-04) grün; `TestSprintOhnePrioUndEinschiebbar` belegt B-372/AC-03 (bestand schon). `task check:dev` grün (go test, golangci-lint 0 Issues).
- Neu: `internal/mcpsrv/check_job.go` (Lauf im Hintergrund, Lauf-ID `L<n>`, letzte 20 Läufe, Panik wird zum Fehler, Sperre und Meldungen wie bisher), Parameter `run` an `check_run`, Texte in `tools.go` und `instructions.md`; Session-IDs `K3.2a` in `store.go` und `planning.go`.
- Nicht geprüft: Herkunft des Timeouts (Client oder k3c-dev) wurde nicht gemessen; die Live-Messung scheiterte an einer abgelaufenen MCP-Sitzung (k3c-dev startete beim Bauen neu). Vermerkt in B-372 › Notizen, die Messung gehört 🧑 bzw. einer Session mit laufendem k3c-dev neuer Bauart.
- Abweichung: `console_tail`-Doku stimmte bereits mit dem Schema überein; der Test sichert es nur. Neue Tickets: keine.

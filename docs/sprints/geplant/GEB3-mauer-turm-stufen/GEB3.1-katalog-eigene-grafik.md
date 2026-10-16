# GEB3.1 · Asset-Katalog kennt eigene Gebäude-Grafik

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb3/1-katalog-eigene-grafik
- **Abhängig von:** –
- **Tickets:** B-437
- **Kriterien:** AC-04

## Ziel

Der Asset-Katalog von k3c-dev führt jede PNG unter `public/grafik/gebaeude/` mit eigener Lizenz statt „unbekannt“.

## Kontext

- **Spec:** GEB3 AC-04 = B-437/AC-01.
- `tools/k3c-dev/internal/assets/catalog.go`, Funktion `asset`: Pack = `parts[1]` (bei `grafik/gebaeude/x.png` also `gebaeude`); ohne Credit-Zeile setzt der Code für Sounds `selfMade`, sonst `"unbekannt"`. Eine zweite Konstante für eigene Gebäude-Grafik (z. B. `ownBuilding = "eigene Grafik (tools/gebaeude, kein Credit nötig)"`) und ein Fall `a.Pack == "gebaeude"` genügen. `LicenseFlag` muss für den neuen Text leer bleiben (Test in `catalog_test.go` Zeile ~66 erweitern).
- Vitest-Seite hat dieselbe Ausnahme schon (`src/tools/credits.test.ts` `OWN`, `zuordnung.test.ts`).

## Erlaubte Dateien

- `tools/k3c-dev/internal/assets/catalog.go`, `tools/k3c-dev/internal/assets/catalog_test.go`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Kein Umbau des Katalogs, keine CREDITS-Zeile für `gebaeude`, keine Änderung an der Oberfläche von k3c-dev.

## Schritte

1. Sprint-Branch `sprint/geb3` (erste Session: von `origin/develop`, `Start-Commit` setzen), `Status: in Arbeit`, committen, pushen.
2. Test zuerst in `catalog_test.go`: PNG unter `grafik/gebaeude/` → eigene Lizenz, Flag leer; Pack-PNG ohne Credit → weiter „unbekannt“.
3. `asset` anpassen.
4. Code-Review, Ergebnis ausfüllen, `Status: fertig`, B-437 `erledigt`, ein Commit, push.

## Fertig, wenn

- [ ] AC-04: `go test` in `tools/k3c-dev/internal/assets` belegt eigene Lizenz und unveränderten Fallback.
- [ ] `check_run dev:test` für `internal/assets` grün (B-440 betrifft nur `internal/planning`), `task check` grün.

## Prüfen

```bash
task check
task check:dev
```

`check:dev` scheitert vorbestehend an `internal/planning` (B-440); maßgeblich ist `internal/assets`. Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

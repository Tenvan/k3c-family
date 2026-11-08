# CU1.4 · Workbench und assetkit ohne Register

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** mcp
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/cu1
- **Abhängig von:** PS1
- **Tickets:** B-700
- **Kriterien:** AC-01

## Ziel

`tools/assetkit` und die Workbench kennen kein Grafik-Register und keinen Atlas mehr: Register-, Katalog-, Zuordnungs- und Atlas-Code ist entfernt, der Bereich Assets › Sound liest `data/sounds.json`, die Recherche-Tools laufen ohne Slot-Prüfung gegen das Register. `task check:dev` grün.

## Kontext

- Treffer (Suche 2026-11-07): `tools/assetkit/register.go` (+Test), `catalog.go` (+Test, nennt `tools/buildings`), `zuordnung.go` (`docs/assets/zuordnung*.md`), `slotdetails.go` (+Test, `gen:atlas`, `atlas-0`), `atlasread.go` (`public/atlas/atlas.json`), `usage.go` (`usageSkip` mit `assets.json`, `sprites.json`), `vorrat_adopt_parallel_test.go`, `repo_test.go`, `tools/assetkit/codemap.md`.
- Workbench: `internal/studioview/studioview.go` (Sound-Slots „aus data/assets.json“, Zeilen 24–34), `internal/mcpsrv/tools_assets.go`, `tools_assets_slots_test.go`, `instructions.md` (Abschnitt Recherche: `id` ist Slot aus dem eingefrorenen Register). Nach MC1.4 gibt es kein `assets_catalog` mehr.
- `data/sounds.json` entsteht in CU1.2 (gleiches Format wie das Register, nur Sound-Slots). Läuft CU1.4 vor CU1.2, legt CU1.4 die Datei an (Inhalt = Sound-Slots aus `data/assets.json`) und CU1.2 übernimmt sie.
- Bleiben: Recherche-Tools (`tools_assets_research.go`) mit `data/assets-recherche.json`, `data/assets-vorschlaege.json`, `data/asset-prompts.json`; Papier-Paket `internal/paperassets`.
- Die Workbench läuft als EXE der Repo-Wurzel: Prüfen nur über `check_run` mit `checkout` (`dev:test`, `dev:lint`) bzw. `task check:dev`.

## Erlaubte Dateien

- `tools/assetkit/**`
- `tools/k3c-workbench/internal/studioview/**`, `internal/mcpsrv/tools_assets*.go`, `internal/mcpsrv/instructions.md`, `internal/mcpsrv/codemap.md`
- `tools/k3c-workbench/frontend/src/**` (nur Seiten, die Register- oder Atlas-Daten anzeigen)
- `data/sounds.json` (nur falls vor CU1.2)
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-Zeilen)

## Nicht-Ziele

`data/assets.json` löschen (CU1.5); Recherche-Tools fachlich ändern; Papier-Tools (`internal/paperassets`).

## Schritte

1. Test (rot): `assetkit` exportiert keine Register-/Atlas-Funktionen mehr (Kompilier-Test oder `go vet` mit Suche); `studioview` liest Sound aus `data/sounds.json` (Test mit Test-Repo ohne `data/assets.json`).
2. Register-, Katalog-, Zuordnungs-, Slotdetail- und Atlas-Code samt Tests entfernen; Aufrufer in der Workbench umstellen oder entfernen.
3. Recherche-Tools: Slot-Prüfung gegen Register streichen, `instructions.md` anpassen.
4. Frontend: Ansichten, die Register oder Atlas zeigen, entfernen.
5. Codemaps nachziehen; `task check:dev`.

## Fertig, wenn

- [ ] AC-01: `git grep -n -E "assets.json|atlas.json|sprites.json|zuordnung" -- tools/assetkit tools/k3c-workbench` liefert nur noch die Recherche-Dateien (`assets-recherche.json`, `assets-vorschlaege.json`).
- [ ] Workbench-Tests im Test-Repo ohne `data/assets.json` grün.
- [ ] `task check:dev` grün.

## Prüfen

```bash
task check:diff
task check:dev
```

Kein Review-Agent, außer der Diff wächst über 150 geänderte Code-Zeilen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

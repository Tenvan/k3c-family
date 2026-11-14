# CU1.4 · Workbench und assetkit ohne Register

- **Status:** fertig
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

- **AC-01 umgesetzt, geprüft:** `git grep -n -E "assets.json|atlas.json|sprites.json|zuordnung" -- tools/assetkit tools/k3c-workbench` (ohne `node_modules`) ist leer. Dazu: Register liest `data/sounds.json` (`RegisterPath`), `atlasread.go`, `slotdetails.go`(+Test) und `zuordnung.go` sind entfernt, `usageSkip` kennt `sounds.json`, Sound-Seite (`studioview.Sounds`), MCP-Anleitung und Frontend-Texte nennen `data/sounds.json`; der Größenvergleich (`assetsui/groesse.go`) liest die Rollenmaße aus `data/paper.json › figuren` statt `data/sprites.json`.
- **Tests:** `studioview`, `assetsui`, `datadoc`, Workbench-Root (`.`) und `mcpsrv` (Einzelaufruf) grün, Frontend 464 Tests grün, `go vet` beider Module sauber. Rot lokal: `tools/assetkit` (TLS-Tests: `x509: certificate signed by unknown authority`, Norton; mit zurückgestellten Änderungen ebenfalls rot, in der CI grün) und zwei `mcpsrv`-Tests nur unter Last (`TestSimTestClientsFakeClient`, `TestSimTestOnlineHeadless` laufen einzeln grün). `task check:dev` und `task check:diff` nicht gelaufen (Nutzerwunsch); kein Review-Agent.
- **Abweichung vom Plan (verschoben → B-760):** Der Register-/Katalog-Code in `assetkit` ist nicht ganz entfernt. Er hängt in Recherche, Import, Vorrat und Credits an `reg.Find(slot)` und an Grafik-Slots; das Streichen brächte einen Umbau der ganzen Recherche samt Tests (Nicht-Ziel „Recherche-Tools fachlich ändern“). `catalog.go` läuft deshalb weiter über `public/grafik` und `public/sprites`; fehlen sie (CU1.5), ist das kein Fehler. Die Frontend-Katalog-Seite zeigt die Papier-Assets und blieb.
- Hinweis: Das Test-Repo der Workbench-Tests enthält jetzt `data/sounds.json` statt `data/assets.json` (Tests laufen ohne `data/assets.json`).

# MC1.4 · Alte Register-Tools entfernen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** mcp
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mc1/4-register-tools-entfernen
- **Abhängig von:** MC1.3
- **Tickets:** B-699
- **Kriterien:** AC-06

## Ziel

Auf `develop` gibt es in der Workbench keine Register-Variante der Asset-Tools mehr: `assets_catalog` und die Slot-Helfer sind entfernt, `instructions.md` und `codemap.md` beschreiben nur noch die Spec-Tools; die Recherche-Tools bleiben. `task check:dev` ist grün.

## Kontext

- Vertrag `docs/standards/paper-mcp.md` › 2: ersetzt werden `assets_list`, `assets_get`, `assets_set`, `assets_catalog` (Register `data/assets.json` ab MC1 eingefroren, Entfernung der Datei B-700 / CU1). **Bleiben:** `assets_research`, `assets_hits`, `assets_hit_add`, `assets_hit_drop`, `assets_import` (`tools_assets_research.go`, nicht anfassen).
- Nach MC1.2 sind `assets_list/get/set` schon auf Spec-Basis (gleiche Namen). Übrig sind: das Tool `assets_catalog`, die Typen `assetsListIn`/`assetsIDIn`/`assetsSetIn`/`assetsCatalogIn` und die Funktionen `slotText`, `choiceText`, `assetsCatalog`, soweit MC1.2 sie nicht schon entfernt hat, sowie der Import `k3c/tools/assetkit` in `tools_assets.go`, falls nur noch dafür gebraucht. `tools/assetkit/` selbst bleibt (Studio, Recherche).
- `instructions.md` › „Grafik- und Sound-Slots“ beschreibt noch `assets_list {art?, kategorie?, status?}`, `assets_catalog`, `assets_set {id, wahl}`; Zeile 25 (Worktrees) zählt die Tools mit `checkout` auf. `internal/mcpsrv/codemap.md` nennt `tools_assets.go (assets_list, assets_get, assets_set, assets_catalog)`.
- `assetRepo(t)` (seit MC1.2 in `tools_assets_slots_test.go`) wird von den Research-Tests gebraucht und bleibt.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/mcpsrv/tools_assets.go`, `tools_assets_set.go`, `tools_assets_render.go`, `tools_assets_preview.go`, `tools_assets_review.go`, `tools_assets_prompts.go` (nur Reste entfernen)
- `tools/k3c-workbench/internal/mcpsrv/tools_assets_test.go`, `tools_assets_slots_test.go`
- `tools/k3c-workbench/internal/mcpsrv/tools.go` (nur Registrierung)
- `tools/k3c-workbench/internal/mcpsrv/instructions.md` (Abschnitt Assets, Zeile Worktrees)
- `tools/k3c-workbench/internal/mcpsrv/codemap.md` (nur die Zeile zu `tools_assets*.go`)

## Nicht-Ziele

- `data/assets.json`, `tools/assetkit/`, `tools/k3c-studio/` nicht ändern (CU1 bzw. WB1).
- Recherche-Tools nicht ändern.

## Schritte

1. `TestAlteRegisterToolsEntfernt` (rot): `ListTools` enthält kein `assets_catalog`; die Beschreibungen von `assets_list`, `assets_get`, `assets_set` nennen weder `data/assets.json` noch „Slot“; die Eingabe-Schemas haben keine Felder `art`, `kategorie`, `status`, `wahl`. Die Research-Tools sind weiter gelistet.
2. `assets_catalog` und die Register-Reste entfernen; `go vet`/`golangci-lint` melden keine ungenutzten Funktionen (`check_run {target: dev:test, pattern: TestAssets}`).
3. `instructions.md`: Register-Zeilen zu `assets_list`/`assets_get`/`assets_set`/`assets_catalog` streichen, Unterabschnitt zu Recherche-Tools behalten und klarstellen, dass `id` dort weiter ein Slot aus dem eingefrorenen Register ist; Zeile Worktrees: „alle `assets_*`“ statt Einzelaufzählung. `TestInstructionsNennenAssetTools` (MC1.3) bleibt grün und findet kein `assets_catalog` mehr.
4. `codemap.md`: Zeile zu `tools_assets*.go` auf die neuen Dateien und Tools.
5. `check_run {target: task:check:diff}`, danach `task check:dev` (Konsole oder `check_run` falls im Katalog; sonst `dev:test` + `go:lint` mit `checkout`).

## Fertig, wenn

- [ ] AC-06: `TestAlteRegisterToolsEntfernt` grün; `grep -n "assets_catalog" tools/k3c-workbench/internal/mcpsrv` liefert nichts.
- [ ] AC-06: `task check:dev` grün (Frontend, go test, golangci-lint der Workbench).
- [ ] Research-Tests (`check_run {target: dev:test, pattern: TestAssetsResearch}`) grün.

## Prüfen

```bash
task check:diff
task check:dev
```

Kein Review-Agent nötig (Entfernen, unter 150 Zeilen), außer der Diff wächst darüber.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

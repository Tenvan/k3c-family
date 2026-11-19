# WE5.5 · Welt-Feld skins und Name-Override für Gegner-Varianten (B-984)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we5
- **Abhängig von:** WE5.4
- **Tickets:** B-984
- **Kriterien:** AC-05, AC-08

## Ziel

Die Welt-Datei kennt `skins` (Basis-Asset → Ersatz-Asset), `WorldDef` reicht sie weiter; `name` einer Gegnerart ist übersteuerbar (B-984).

## Kontext

Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Welt-Datei streng gelesen in `engine/sim/island_worlds.go` (unbekanntes Feld → Fehler; `extends`-Kette), Typ `WorldDef`. Asset-Pfade haben die Form `<gruppe>/<asset>` wie in `data/paper.json` (z. B. `gegner/wolf`, `objekte/tanne`); die Engine liest keine Grafik. Gegner-Namen stehen in `data/enemies.json` (`name`), Overrides nach `docs/rules/welten.md` § 1.1. Regeln: `welten.md` § 2 (Format), § 3 (Prüfung).

## Erlaubte Dateien

- `engine/sim/**`
- `docs/rules/welten.md`

## Nicht-Ziele

Zeichnen im Spiel (WE6.2), Workbench (WE6.3), Prüfung, ob ein Asset existiert, neue Gegner-IDs.

## Schritte

1. Tests zuerst: Welt mit `skins` lädt; `extends` mischt je Schlüssel; Fehlerfälle (Form, Gruppenwechsel, Selbstverweis); `overrides › enemies › wolf › name`.
2. Feld `skins` lesen, prüfen, in `WorldDef` ablegen.
3. `welten.md` § 2 und § 3 nachziehen; Review `k3c-reviewer` vor dem Commit.

## Fertig, wenn

- [ ] AC-05: B-984/AC-01 bis AC-03 als Tests grün.
- [ ] AC-05, AC-08: Regel nachgezogen, `testdata/golden/` unverändert (B-984/AC-04).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-05 geprüft: `TestSkinsLaedtUndMischtMitExtends`, `TestSkinsResolveWorldOhneEltern`, `TestSkinsFehlerfaelle`, `TestWorldValuesGegnerNameUebersteuerbar`; `welten.md` § 2.3 nachgezogen. AC-08 geprüft: Golden unverändert. `go vet`, Lint 0, `go test ./engine/...` (bis auf bekanntes `TestHTMLEigenstaendig`), `task check:diff` grün. Commit 789a039af auf `sprint/we5`. API: `WorldDef.Skins` (nach `ResolveWorld` über die extends-Kette gemischt), `EnemyData.Name`. `island_worlds.go` hat genau 400 Zeilen.

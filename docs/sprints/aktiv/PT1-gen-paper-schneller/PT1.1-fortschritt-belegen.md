# PT1.1 · gen:paper-Fortschritt und Kopieren belegen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** pt1/1-fortschritt-belegen
- **Abhängig von:** –
- **Tickets:** B-740, B-748, B-749, B-750
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

B-740, B-748, B-749 und B-750 sind gegen ihre Kriterien belegt, Lücken im Bereich nachgezogen, die Tickets stehen auf `erledigt`.

## Kontext

Umsetzung laut Commits: B-740 mit #518 (`tools/paper/cli/fortschritt.ts`, `gen.ts`), B-748 mit #520 (`tools/Taskfile.yml` › `paper-sync`/`paper-render`, `bin/gen.ts --fehlt`, `--nur-kopieren`), B-749 mit #521 (⏳-Zeile `vorAsset`), B-750 mit #522 (↳-Zeilen `beiAusgabe`). Tests: `tools/paper/cli/gen.test.ts`, `work.test.ts`. Ein voller Lauf dauert gut 3 Minuten; für B-750/AC-01 genügt ein Lauf, der `figuren/monarch` rendert.

## Erlaubte Dateien

- `tools/paper/cli/gen*.ts`
- `tools/paper/cli/work*.ts`
- `tools/paper/cli/fortschritt*.ts`
- `tools/paper/bin/gen.ts`
- `tools/Taskfile.yml`

## Nicht-Ziele

Cache und Parallelität (PT1.2), Specs und Stil.

## Schritte

1. Je Kriterium Test oder Lauf: `bunx vitest run tools/paper/cli` (bzw. `check_run`), `task gen:paper-sync --status` vor und nach dem Löschen eines Manifests, `.work/grafik/gen.log` prüfen.
2. Lücken (z. B. fehlender Formattest für `gen.log`) ergänzen.
3. Ergebnis eintragen, Tickets auf `erledigt`.

## Fertig, wenn

- [ ] AC-01: `B-740/AC-01` bis `AC-03` belegt.
- [ ] AC-02: `B-748/AC-01` bis `AC-03` belegt.
- [ ] AC-03: `B-749/AC-01` und `B-750/AC-01` belegt.
- [ ] `task check:diff` grün.

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

AC-01 bis AC-03 geprüft: neuer `tools/paper/cli/fortschritt.test.ts` (stderr und `gen.log`, ✅/❌), Test „⏳ vor 🔨“. Läufe: `gen:paper-sync --status` Exit 1, nach dem Kopieren 0 (3 s, ohne Rendern); `--fehlt` 1/0; Monarch zeigt 210 ↳-Zeilen (heute 210 statt 195 Ausgaben). `check:diff` grün. Commit 1411cca32.

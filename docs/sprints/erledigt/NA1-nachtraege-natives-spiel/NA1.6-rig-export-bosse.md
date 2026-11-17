# NA1.6 · Nativer Rig-Export der Bosse auf Stand der Specs

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/na1
- **Abhängig von:** NA1.5
- **Tickets:** B-885
- **Kriterien:** AC-07

## Ziel

`native/k3c-rig/assets/bosse` und `assets/soll/bosse` entsprechen den Boss-Specs `version` 3; `task native:rig:export:check` ist grün (B-885).

## Kontext

Rig-Prototyp `native/k3c-rig` (eigenes Go-Modul). `task native:rig:export` (`native/Taskfile.yml` Zeile 141, `bun native/k3c-rig/export/export.ts`) schreibt Teil-PNGs, Rig-Dateien, Elemente und Sollwerte nach `native/k3c-rig/assets/` (eingecheckt) und liest die Specs unter `public/grafik/` nur. `task native:rig:export:check` wiederholt den Export und vergleicht byte-genau, `task native:rig:test` rechnet das Rig gegen die Sollwerte. Seit MG10 (B-720) stehen fünf Boss-Specs auf `version` 3, der Export auf `version` 1 (`native/k3c-rig/assets/bosse/*/rig.json`). `kristallherz-waechter` hat keinen Export. Dieses Ticket ist das Feature, der Gen-Lauf gehört hierher (B-742).

## Erlaubte Dateien

- `native/k3c-rig/assets/**`

## Nicht-Ziele

Specs unter `public/grafik/` ändern; Code in `native/k3c-rig/rig` oder `export/`; `kristallherz-waechter` neu bauen.

## Schritte

1. `task native:rig:export:check` → Abweichung festhalten (welche Gruppen).
2. `task native:rig:export`.
3. `git status --short native/k3c-rig/assets` prüfen: erwartet nur Bosse; andere Gruppen im Ergebnis nennen.
4. `task native:rig:export:check` und `task native:rig:test`.
5. Agent `k3c-grafik` prüfen lassen; `task check:diff`.

## Fertig, wenn

- [ ] AC-07: `task native:rig:export:check` und `task native:rig:test` grün; `git diff --stat` nur unter `native/k3c-rig/assets/` (B-885/AC-01, AC-02).
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

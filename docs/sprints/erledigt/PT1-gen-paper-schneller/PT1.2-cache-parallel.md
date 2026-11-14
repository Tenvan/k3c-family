# PT1.2 · Cache je Asset und paralleles Rastern

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** pt1/2-cache-parallel
- **Abhängig von:** PT1.1
- **Tickets:** B-751
- **Kriterien:** AC-04

## Ziel

`gen:paper` überspringt unveränderte Assets anhand des `renderKey` und rastert die Ausgaben auf mehreren Kernen, bitgleich zum heutigen Ergebnis.

## Kontext

B-751: 98 % der Zeit in `resvg` (`@resvg/resvg-js`, ein Kern je Aufruf), `figuren/monarch` allein über 2 Minuten mit 195 Ausgaben. Manifeste in `.work/grafik/<gruppe>/<asset>/manifest.json` tragen schon `renderKey` (`tools/paper/cli/gen.ts`). Default: Worker = verfügbare Kerne minus 1, höchstens 8, `K3C_PAPER_JOBS` übersteuert. Worker mit Bun-/Node-Bordmitteln (`worker_threads` oder Kindprozesse), keine neue Abhängigkeit. Golden: `tools/paper/golden` nur lesen.

## Erlaubte Dateien

- `tools/paper/cli/gen*.ts`
- `tools/paper/cli/work*.ts`
- `tools/paper/cli/pool*.ts`
- `tools/paper/cli/fortschritt*.ts`
- `tools/paper/bin/gen.ts`

## Nicht-Ziele

Kosten je Frame senken (optional in B-751, nur wenn die Messung es verlangt), Renderer tauschen.

## Schritte

1. Messung vorher (voller Lauf, Zeit je Gruppe) in die Notizen von B-751.
2. Cache: Asset überspringen, wenn `renderKey` aus Spec, Stil und Renderer-Version zum Manifest passt; Test.
3. Parallel: Ausgaben eines Assets und Assets untereinander auf Worker verteilen; Fortschrittszeilen bleiben vollständig.
4. Messung nachher, Grafik-Golden grün, zweiter Lauf 0 Assets.
5. Ergebnis eintragen, B-751 auf `erledigt`.

## Fertig, wenn

- [ ] AC-04: zweiter `task gen:paper` rendert 0 Assets (`B-751/AC-01`); voller Lauf mit 8 Kernen ≤ ein Drittel der Zeit, `tools/paper/golden` unverändert grün (`B-751/AC-02`).
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

AC-04 geprüft: zweiter Lauf 0 Assets (3 s); kalt 5 min 19 s → 54 s mit 8 Workern (22 Kerne); golden.json unverändert, Bitgleichheit per Test gegen `rendereAsset` und `gen`. Cache = `renderKey` plus Hash der Renderer-Quellen; `K3C_PAPER_JOBS` übersteuert (Kerne minus 1, höchstens 8). Review-Befunde (k3c-reviewer) behoben; offen niedrig: kein Timeout für hängendes resvg. `check:diff` grün. Commit b33d00515.

# PT4.1 · frame --ohne mit Beinpfad von Vierbeinern, mit Review

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** pt4/1-frame-ohne-vierbeiner
- **Abhängig von:** –
- **Tickets:** B-851
- **Kriterien:** AC-01

## Ziel

`paper frame --ohne oberbein-<b>`/`unterbein-<b>` entfernt bei Vierbeinern den Beinpfad dieses Beins.

## Kontext

Spec B-851 (freigegeben, Klasse A). Vorarbeit PT3.1 (`tools/paper/cli/frame.ts`, `tools/paper/rig/{beinpfad,reitBoxen,reiten}.ts`); `einschub` in `vierbeinerBild` fragt nur `nur(id)`.

## Erlaubte Dateien

- `tools/paper/cli/frame*.ts`
- `tools/paper/rig/**`
- `tools/paper/bin/paper.test.ts`
- `docs/standards/paper-spec.md`
- Planung PT4, B-851 (über MCP)

## Nicht-Ziele

Workbench, Specs, Golden.

## Schritte

1. Test (rot), Umsetzung, Vertrag.
2. `bun test tools/paper --timeout 60000`, `task:check:diff` grün.

## Fertig, wenn

- [ ] AC-01 mit Nachweis.
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

- AC-01 geprüft: `rig/vierbeiner.test.ts` › „B-851/AC-01“ grün (`--ohne oberbein-vn`/`unterbein-vn` entfernt den Beinpfad vn aus dem SVG, hf bleibt); `bin/paper.test.ts` › „B-851/AC-01“ grün (Exit 0, Boxen gleich, PNG anders, Ober- und Unterbein gleichwertig).
- Umsetzung: `VierbeinerKontext.beinOhne` (`vierbeinerBild.ts`), `frame.ts` reicht `w.ohne`; `vierbeinerBild` und `reittierEbeneBild` (über `reitSegmente`) nutzen `beinAus` aus `reitBoxen.ts`. Vertrag `paper-spec.md` › 8 nachgezogen. Render ohne `--ohne` unverändert.
- `bun test tools/paper` 351 grün, 1 rot: `golden/public.test.ts` (fehlendes `galopp.png` im Worktree, B-737, bekannt). `task:check:diff` grün (59 s). Kein Gen, kein Golden.
- Neue Tickets: keine.

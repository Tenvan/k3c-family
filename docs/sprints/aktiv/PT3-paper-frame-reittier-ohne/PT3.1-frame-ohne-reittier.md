# PT3.1 · frame --ohne mit Reittier-Teilen, Bein-Box mit Wulst

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** pt3/1-frame-ohne-reittier
- **Abhängig von:** –
- **Tickets:** B-845
- **Kriterien:** AC-01

## Ziel

`paper frame --ohne reittier:<teil>` funktioniert bei Reit-Animationen, die Bein-Box umfasst den Wulst der Vorderbeine.

## Kontext

Spec B-845 (Planungsangaben vervollständigen, dann Klasse A). Vorarbeit PT2.1 (`tools/paper/rig/reitBoxen.ts`, `pruefeOhne` in `tools/paper/cli/frame.ts`).

## Erlaubte Dateien

- `tools/paper/cli/frame*.ts`
- `tools/paper/rig/**`
- `tools/paper/bin/paper.test.ts`
- `docs/standards/paper-spec.md`
- Planung PT3, B-845 (über MCP)

## Nicht-Ziele

Workbench-Code, Specs, Golden.

## Schritte

1. Spec vervollständigen und freigeben.
2. Test (rot) für `--ohne reittier:hals` und die Bein-Box.
3. Umsetzung, Vertrag nachziehen.
4. `bun test tools/paper --timeout 60000` gezielt, `task:check:diff` grün.

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

- Spec B-845 vervollständigt (Anforderungen, Fehlerfälle), freigegeben „Klasse A, Claude, 2026-11-13“.
- AC-01 geprüft: `bin/paper.test.ts` › „B-845/AC-01“ grün: `--ohne reittier:hals` bei `galopp` → Exit 0, Box fehlt, alle anderen Boxen gleich, PNG anders; `reittier:oberbein-vn` nimmt den Beinpfad samt beiden Bein-Boxen; unbekanntes `reittier:`-Teil und `reittier:` ohne Reittier → Exit 1 `teil-unbekannt` (Liste mit Präfix).
- AC-02 geprüft: `rig/reitBoxen.test.ts` › „B-845/AC-02“ grün: Oberbein-Box umfasst beide Kanten des Beinpfads (`beinKanten`, neu in `beinpfad.ts`, Wulst als `WULST`) in vier Galopp-Phasen.
- Umsetzung: `frame.ts` prüft und reicht `reittierOhne` über `ReitKontext` (`rig/reiten.ts`, Erweiterung von `Kontext` ohne Änderung an `render/`); `reitSegmente` und `reittierGeometrie` nehmen `ohne`. Vertrag `paper-spec.md` › 8 nachgezogen.
- `bun test tools/paper` 349 grün, 1 rot: `golden/public.test.ts` › „monarch hat idle …“ (fehlendes `galopp.png`, im Worktree nicht gerendert, B-737; unabhängig). Golden-Render gleich (Render ohne `--ohne` unverändert).
- `task check:diff` grün (check_run, Worktree). Kein Gen, kein Golden.
- Neue Tickets: keine.

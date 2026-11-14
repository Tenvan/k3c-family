# PT2.3 · Review PT2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** pt2/3-review
- **Abhängig von:** PT2.2
- **Tickets:** B-708, B-735
- **Kriterien:** alle

## Ziel

Sprint PT2 ist abgenommen, der eine PR des Sprints ist offen mit Auto-Merge, der Sprint liegt in `erledigt/`.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Diff in `tools/paper`, `public/grafik`, Golden). Prüfpunkte: Vertrag und Code stimmen, Golden-Änderung begründet, Standard-Stil unverändert. Gen-Lauf und Golden liefen am Feature (PT2.2).

## Erlaubte Dateien

- alle Erlaubten Dateien von PT2.1 und PT2.2
- Planung PT2 und ihre Tickets (über MCP)

## Nicht-Ziele

Neue Funktionen, Stil-Befunde.

## Schritte

1. `task check`, `task check:go` nacheinander.
2. Diff `origin/develop...sprint/pt2` lesen, `k3c-reviewer` parallel; schwere Befunde beheben.
3. Abnahme in der README, Sprint nach `erledigt/`, PR mit Auto-Merge.

## Fertig, wenn

- [ ] Alle Kriterien der README mit Nachweis in den Session-Ergebnissen.
- [ ] Review-Befunde behoben oder als Ticket.
- [ ] PR offen mit Auto-Merge.

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

- `task check` grün (4 min), danach `task check:go` grün (2 min), beide über `check_run` im Worktree.
- Kriterien: AC-01 bis AC-03 mit Nachweis in PT2.1, AC-04 und AC-05 in PT2.2.
- Review `k3c-reviewer` (Diff `c8ff7f341..HEAD`): kein schwerer, kein mittlerer Befund; Vertrag und Code stimmen, Golden-Änderung begründet (PNG-Hashes nur bei umgestellten Assets und `*-wolf*` des Monarchen, sonst nur `renderKey`), Stil bis auf die vier Materialien unverändert. Behoben: fehlender Nachweis für drei Assets in `ersatz.test.ts`, falsche Angabe zu `index.paper.json` im PT2.2-Ergebnis. Als Ticket: `--ohne` mit `reittier:`-Teilen und Wulst der Bein-Box (B-845).
- Abnahme-Ticket: B-844 (Gen; Sim und Balance verschoben, Sperre 2026-11-11).
- Abweichung: PR mit Auto-Merge nicht geöffnet (Auftrag ohne Push; Branch des Worktrees, Merge über die Welle). B-844/B-845 umnummeriert von B-838/B-839 (Kollision mit WZ44).

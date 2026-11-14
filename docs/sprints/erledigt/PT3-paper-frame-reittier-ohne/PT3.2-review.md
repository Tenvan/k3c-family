# PT3.2 · Review PT3

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** pt3/2-review
- **Abhängig von:** PT3.1
- **Tickets:** B-845
- **Kriterien:** alle

## Ziel

Review des Sprints PT3, Abnahme in der README.

## Kontext

Nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer`.

## Erlaubte Dateien

- Planung PT3, B-845 (über MCP)
- kleine Fixes in den Dateien von PT3.1

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Kriterien prüfen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket für Gen nur, wenn Grafik betroffen.

## Fertig, wenn

- [ ] Abnahme in der README, Sprint erledigt.

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

- AC-01 (Sprint) geprüft: `B-845/AC-01` und `B-845/AC-02` grün (PT3.1, Commit f9bf699a3).
- `task check` grün (6 min 47 s), `go:test` grün (2 min 47 s), beide über check_run im Worktree.
- k3c-reviewer: keine Befunde; Render ohne `--ohne` unverändert, kein Golden berührt, Randfälle (Variante, Ebenen, Figur ohne Reittier) belegt.
- Abnahme-Ticket für Gen: keins nötig (Grafik nicht betroffen, nur Boxen und Ausblenden in `frame`).
- Neue Tickets (anzulegen): „paper frame --ohne oberbein-<b> blendet bei Vierbeiner-Animationen den Beinpfad nicht aus“ (PAP, Problem, niedrig): Bei `vierbeinerBild` und den Reittier-Ebenen fragt `einschub` nur `nur(id)`, das dort immer true ist, so bleibt der Beinpfad sichtbar. Nur nötig, wenn die Vorschau Beine des Pferds einzeln ausblenden soll.

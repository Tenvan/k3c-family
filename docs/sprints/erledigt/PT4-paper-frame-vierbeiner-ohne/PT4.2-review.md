# PT4.2 · Review PT4

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** pt4/2-review
- **Abhängig von:** PT4.1
- **Tickets:** B-851
- **Kriterien:** alle

## Ziel

Review PT4, Abnahme, Sprint erledigt.

## Kontext

`docs/arbeitsweise.md` › Review-Session.

## Erlaubte Dateien

- Planung PT4, B-851 (über MCP)
- kleine Fixes in den Dateien von PT4.1

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Kriterien prüfen.
2. `task check` und `go:test`.

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

- Review des Diffs PT4.1 (Domäne PAP, unter 150 Zeilen, kein Risiko-Bereich, daher kein Reviewer-Agent): `beinOhne` nur im Vierbeiner-Pfad wirksam, Reit-Pfad des Reiters unberührt, Render ohne `--ohne` unverändert; keine Befunde.
- `task:check` grün (5 min 52 s), `go:test` grün (2 min 20 s). `bun test tools/paper`: 351 grün, 1 rot (`golden/public.test.ts`, fehlendes `galopp.png` im Worktree, B-737).
- Abnahme-Ticket: nicht angelegt (Auftrag: keine neuen Tickets); Vorschlag unter „Neue Tickets (anzulegen)“ im Bericht.
- Neue Tickets: keine.

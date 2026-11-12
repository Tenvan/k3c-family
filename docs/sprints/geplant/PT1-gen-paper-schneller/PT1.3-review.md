# PT1.3 · Review PT1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** pt1/3-review
- **Abhängig von:** PT1.2
- **Tickets:** B-740, B-748, B-749, B-750, B-751
- **Kriterien:** alle

## Ziel

Sprint PT1 ist abgenommen, der eine PR des Sprints ist offen mit Auto-Merge, der Sprint liegt in `erledigt/`.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Diff in `tools/paper`). Prüfpunkte: bitgleiche Ausgabe, stdout bleibt ein JSON, Cache invalidiert bei Stil- und Renderer-Änderung, keine neue Abhängigkeit. Gen-Lauf und Grafik-Golden liefen am Feature (PT1.2); ein Abnahme-Ticket nur, falls dort etwas offen blieb.

## Erlaubte Dateien

- alle Erlaubten Dateien von PT1.1 und PT1.2
- Planung PT1 und ihre Tickets (über MCP)

## Nicht-Ziele

Neue Funktionen, Stil-Befunde.

## Schritte

1. `task check`, `task check:go` nacheinander.
2. Diff `origin/develop...sprint/pt1` lesen, `k3c-reviewer` parallel; schwere Befunde beheben.
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

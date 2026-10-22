# BD2.4 · Review BD2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** bd2/4-review
- **Abhängig von:** BD2.3
- **Tickets:** B-378, B-377, B-382
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Sprint BD2 ist gegen seine Kriterien AC-01 bis AC-03 geprüft, Befunde sind behoben oder als Tickets erfasst, der Sprint ist nach `erledigt/` verschoben.

## Kontext

- Sprint-README: `docs/sprints/geplant/BD2-rueckmeldung-lobby-nachtraege/README.md` (nach Aktivierung unter `aktiv/`). Ergebnisse der Sessions BD2.1 (SIM, `engine/sim/skills.go`), BD2.2 (SRV, `engine/net`), BD2.3 (CLI, `src/scenes/lobbyLogic.ts`) lesen.
- Review nach `docs/arbeitsweise.md` › Tempo im Lauf: Reviewer-Agent getrennt vom Schreiben (Sonnet); Schwerpunkte: Determinismus der Sim, Netz-Budget Q08 (B-386, kein Wachstum der Ereignisse), Spielstand unverändert (`SkillHeld` nicht gespeichert), Lobby-Herkunft statt Name, Golden-Änderungen nachvollziehbar.
- Ticket-Status `erledigt` setzen (B-378, B-377, B-382) über `plan_set`.

## Erlaubte Dateien

- Dateien der Sessions BD2.1–BD2.3 nur für Befund-Fixes (≤ 30 Zeilen je Fix, sonst Ticket)
- `docs/sprints/` (README, Sessions, `sprints/README.md`), `docs/backlog/`, `docs/projekte/BED-bedienung-hud.md` (Tabelle)

## Nicht-Ziele

Neue Funktionen, Umbauten über die Befunde hinaus.

## Schritte

1. Branch `bd2/4-review`, `Status: in Arbeit`. Diff `origin/develop...sprint/bd2` lesen, Ergebnisse der Sessions gegen die Kriterien legen.
2. Reviewer-Agent über den Gesamt-Diff; schwere Befunde beheben, übrige als Ticket.
3. `task check`, `task check:go`, `task check:race` nacheinander.
4. Abnahme in der Sprint-README (≤ 5 Zeilen), Tickets auf `erledigt`, Sprint `Status: erledigt` (verschiebt den Ordner), Tabelle im Projekt BED und Fahrplan in `sprints/README.md` anpassen. Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-03: je Kriterium Nachweis aus den Session-Ergebnissen im Ergebnis genannt.
- [ ] `task check`, `task check:go` und `task check:race` grün.
- [ ] B-378, B-377, B-382 `erledigt`; Sprint BD2 nach `erledigt/` verschoben.

## Prüfen

```bash
task check
```

Danach `task check:go`, dann `task check:race` (nacheinander). Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

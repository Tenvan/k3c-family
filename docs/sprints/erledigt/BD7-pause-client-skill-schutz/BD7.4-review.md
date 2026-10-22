# BD7.4 · Review BD7

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/bd7
- **Abhängig von:** BD7.1, BD7.2, BD7.3
- **Tickets:** –
- **Kriterien:** alle

## Ziel

Sprint BD7 ist geprüft, abgenommen und als ein PR gegen `develop` offen.

## Kontext

Review-Session nach `docs/arbeitsweise.md` › Review-Session. Diff `origin/develop...sprint/bd7` enthält bis zum Merge von #431, #432, #436 und #440 deren Commits; gelesen wird nur der BD7-Anteil (ab dem Merge-Commit der Basis).

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`, `docs/projekte/BED-bedienung-hud.md`
- Fixes schwerer Befunde in den Dateien von BD7.1 bis BD7.3

## Nicht-Ziele

Neue Funktionen; Abnahme am Gerät (🧑).

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander.
2. Diff von einem getrennten Reviewer lesen lassen (nur schwere Befunde).
3. Falls Dienste über `svc_start` erreichbar: im Browser-Pane Menü öffnen/schließen, Server-Log `🚦 Gerät pausiert` belegen.
4. Abnahme, Sprint und Tickets erledigt, PR.

## Fertig, wenn

- [ ] alle Kriterien mit Nachweis in den Session-Ergebnissen.
- [ ] `task check`, `task check:go` grün; `task check:race` grün oder ehrlich übersprungen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- Alle Kriterien: AC-01 (BD7.1), AC-02/AC-03 (BD7.2), AC-04/AC-05 (BD7.3) mit Nachweis. `task check`, `task check:go` grün; `task check:race` übersprungen (kein C-Compiler, prüft die CI).
- Sprint-Review (Sonnet, Diff ab Basis-Merge): abgenommen, kein schwerer Befund.
- Browser-Pane (Agent, Dienste des Worktrees über `svc_start`, Couch-Raum mit einem Tastatur-Spieler): Esc öffnet das Menü → `{"t":"pause"}`, `cycle.secondsLeft` steht 3 s lang still; Esc schließt → `{"t":"resume"}`, die Zeit läuft weiter (579,1 → 576,1 in 3 s). Abnahme am Gerät (Xbox, online mit zwei Geräten) offen bei 🧑.

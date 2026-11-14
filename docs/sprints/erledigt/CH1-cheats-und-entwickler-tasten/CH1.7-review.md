# CH1.7 · Sprint-Review CH1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ch1
- **Abhängig von:** CH1.1, CH1.2, CH1.3, CH1.4, CH1.5, CH1.6
- **Tickets:** B-888, B-889, B-890
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07, AC-08

## Ziel

Der Sprint CH1 ist geprüft: Reviewer-Agent über den Diff, schwere Befunde behoben, Abnahme-Ticket für die Sichtprüfung angelegt.

## Kontext

- Diff `main..sprint/ch1`.
- AC-08 (Sichtprüfung) bleibt bei 🧑.

## Erlaubte Dateien

- `native/k3c-game/**`
- `data/*.json`

## Nicht-Ziele

- Neue Funktionen.

## Schritte

1. Reviewer-Agent (k3c-reviewer) über den Diff.
2. Schwere Befunde beheben.
3. Abnahme-Ticket anlegen.

## Fertig, wenn

- [ ] Alle AC geprüft oder als offen vermerkt (AC-08).
- [ ] `check_run task:check:diff` grün.

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

- Review (k3c-reviewer, Sonnet) über `3c0588d56..HEAD`: kein Sperrer. Behoben: F5–F8 nur im laufenden Spiel ohne Menü (F6 hob sonst die Pause des Menüs auf); Reload prüft vorher, ob der Spielstand ladbar ist (sonst bleibt das Spiel); kein Einzelschritt auf Vorrat in der Menü-Pause; Cheat-Schalter ohne Spiel zurückgesetzt; Berichtsname mit Taktzahl gegen Überschreiben. Bewusst offen: Reload verwirft den alten Raum ohne `CloseRoom` (eine laufende Hintergrund-Speicherung kann parallel schreiben, vermutet), Schalter-Anzeige nach F6 kann vom Raum abweichen, ein Gerät ohne Platz kann beim Bedienen beitreten.
- AC-01 bis AC-07 geprüft (Session-Ergebnisse CH1.1–CH1.6); `check_run task:check:go` grün, `go test -short ./...` in `native/k3c-game` grün, `task check:diff:go` grün.
- AC-08 offen: Sichtprüfung durch 🧑 im Abnahme-Ticket B-899 (dazu `gen:paper` mit Golden und `check:race`).

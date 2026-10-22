# GEB11.5 · Review: alle Kriterien prüfen

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb11/5-review
- **Abhängig von:** GEB11.4
- **Tickets:** B-458
- **Kriterien:** alle

## Ziel

Alle Kriterien des Sprints sind mit getrenntem Reviewer geprüft, schwere Befunde behoben, PR auf `develop` offen.

## Kontext

`docs/arbeitsweise.md` › Review-Session. Code-Sprint mit Grafik (DEV) und Client (CLI); keine Änderung an `engine/`.

## Erlaubte Dateien

- Dateien der Sessions GEB11.1–GEB11.4 für Befunde, Planungsdateien

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Reviewer-Agent (getrennt vom Schreiben) über den Diff zu `origin/develop`.
2. `task check`, `task check:go`, `task check:race` nacheinander.
3. Abnahme-Abschnitt der README, PR.

## Fertig, wenn

- [ ] alle: Nachweis je Kriterium in den Session-Ergebnissen, Checks grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Review (Reviewer-Agent Sonnet, getrennt vom Schreiben, statisch auf `origin/sprint/geb9..sprint/geb11`): keine schweren Befunde. Bestätigt: kein Eingriff in `engine/` oder `data/hub.json`, jede Textur hat eine Datei, Rückfall auf Burg bzw. gezeichnete Form, Cache-Schlüssel verhindert Neuzeichnen je Frame, alter Snapshot ohne `hubLevel`/`hubUpgrade` fällt auf Burg, Randwerte ohne Division. Mittel (Verhalten): Das Gerüst erscheint auch bei `waitingWorker` durch Gefahr (nachts), also ohne arbeitenden Bauer — für die Abnahme von Ralf notiert. Offen: Tests für `paid > gold` und `gold = 0` (ungeprüft). AC-01, AC-03, AC-04 mit Nachweis in GEB11.1/.3/.4; AC-02 entfällt begründet (GEB11.2 verworfen). AC-04 Abnahme durch Ralf in GEB11.6 offen.

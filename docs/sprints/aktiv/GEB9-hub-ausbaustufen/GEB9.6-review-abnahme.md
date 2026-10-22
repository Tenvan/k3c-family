# GEB9.6 · Review und Abnahme

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb9/6-review-abnahme
- **Abhängig von:** GEB9.5
- **Tickets:** B-435
- **Kriterien:** AC-06

## Ziel

Code-Review des Sprints und Vorbereitung der Abnahme durch 🧑; Sprint wird abgeschlossen, der PR gegen `develop` geöffnet.

## Kontext

Ablauf: `docs/arbeitsweise.md` › Review-Session (Sprint-Abnahme). Die manuelle Abnahme (Stufenreihe in der Vorschau, Spiel in zwei Hub-Stufen) macht 🧑, nicht der Agent (AC-06).

## Erlaubte Dateien

- Befund-Fixes in den Dateien der Sprint-Sessions (je Fix unter ca. 30 Zeilen)
- `docs/sprints/` (Abnahme-Abschnitt, Status, Verschieben nach `erledigt/`), `docs/backlog/` (B-435 Status, neue Tickets), `docs/projekte/GEB-gebaeude-grafik.md`

## Nicht-Ziele

Neue Funktionen, manuelle Browser-Abnahme durch den Agenten.

## Schritte

1. Branch `geb9/6-review-abnahme`, `Status: in Arbeit`.
2. Alle Sessions-Ergebnisse und den Diff `origin/develop..sprint/geb9` lesen; schwere Fehler suchen (Reviewer-Agent, getrennt vom Schreiben).
3. `task check`, `task check:go`, `task check:race` nacheinander.
4. Abnahme-Abschnitt der Sprint-README füllen; AC-06 als „Abnahme durch 🧑 offen: Stufenreihe in der Asset-Vorschau und Spiel in mindestens zwei Hub-Stufen“ führen. Mit offener manueller Abnahme bleibt der Sprint aktiv, bis 🧑 abgenommen hat.
5. PR gegen `develop` öffnen (PR erst nach vollständigem Push).

## Fertig, wenn

- [ ] Alle Kriterien AC-01 bis AC-05 mit Nachweis in den Session-Ergebnissen; AC-06 als Abnahme durch 🧑 vorbereitet.
- [ ] `task check`, `task check:go`, `task check:race` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

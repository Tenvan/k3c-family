# GEB9.6 · Review und Abnahme

- **Status:** fertig
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

- [x] Alle Kriterien AC-01 bis AC-05 mit Nachweis in den Session-Ergebnissen; AC-06 abgenommen durch 🧑.
- [x] `task check`, `task check:go`, `task check:race` grün (Nachprüfung 2026-10-22).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Review und Prüfläufe wie zuvor (keine schweren Befunde, `check:go` und `check:race` grün am 2026-10-22).

AC-06 geprüft (Ralf, 2026-10-24, PC mit Tastatur):
1. Stufenreihe der Hub-Gebäude in der Asset-Vorschau ok.
2. Spiel in mindestens zwei Hub-Stufen ok.
3. Heilplatz-Stufen 1–5 deutlich unterscheidbar ok (nach GEB13).
Keine neuen Tickets. TV/Xbox nicht geprüft.

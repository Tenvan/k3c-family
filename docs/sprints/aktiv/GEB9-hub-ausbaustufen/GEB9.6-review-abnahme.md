# GEB9.6 · Review und Abnahme

- **Status:** blockiert
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

Review (Reviewer-Agent Sonnet, getrennt vom Schreiben, statisch auf `origin/develop..sprint/geb9`): keine schweren Befunde. Rückwärtskompatibilität des Schlüssels `art/farbe` bestätigt (Mauer/Turm/Burg byte-gleich, Randfälle `a/b/c`, `/rope` laufen in Fehler), kein Pfad verlangt eine Textur ohne Datei (40 Stufen-PNGs vorhanden), keine weiteren Verwender der gelöschten Grundform-Dateien. AC-01 bis AC-05 mit Nachweis in GEB9.1–9.5. Prüfläufe: `task test` (Client 2348 Tests), `task typecheck`, `task lint`, `go test` und `golangci-lint` für `tools/buildings`, Studio-Vitest (`scenesLogic`, vollständig 463 Tests in GEB9.2) grün. `task check:go` scheiterte an `engine/devrun` (`TestStopBrichtBeiZweitemSignalAb`, `undefined: kaputt` in einer Test-Fixture): `engine/` ist in diesem Sprint unberührt, vermutlich Wanduhr-Test unter Last bzw. Altlast, ungeprüft ob auf `develop` reproduzierbar. `check:race` nicht gefahren (kein Go-Code in `engine/` geändert). AC-06 (Abnahme durch Ralf in Asset-Vorschau und Spiel mit zwei Hub-Stufen) steht aus; Session auf `blockiert` bis dahin. Auffälligste Schwäche für die Abnahme: Heilplatz-Stufen unterscheiden sich nur dezent (Pfosten, Laterne).

Nachprüfung 2026-10-22 (Claude, autonome Teile, Stand `develop` df21c2c2 mit GEB9 gemergt): `check_run task:check:go` grün (4 min, inklusive `engine/devrun`), `task:check:race` grün. Der `engine/devrun`-Fehler ist damit nicht reproduzierbar (vermutet: Wanduhr-Test unter Last, B-284); nur gemeldet, kein Ticket. Autonom ist nichts mehr offen; AC-06 bleibt bei Ralf (Heilplatz-Stufen zusätzlich als Frage B-544 in KR2). Session bleibt `blockiert`.

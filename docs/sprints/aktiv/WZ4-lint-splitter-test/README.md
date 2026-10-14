# WZ4 · DEV · Splitter-Test in DOM-Umgebung

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-406
- **Start-Commit:** 3880dc29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-14, 🧑 im Chat (Revision 1)

## Ausgangslage

Der Splitter der Planungsseite ist nur über `splitShare`/`clampSplit` getestet, Doppelklick und Ziehen nicht (B-406, im Root-Vitest fehlt jsdom/happy-dom). B-415 (Lint rot durch `tools/cockpit`) ist auf `develop` bereits erledigt und gehört nicht mehr zum Sprint.

## Ziel

Das Splitter-Verhalten der Planungsseite ist in einer DOM-Umgebung getestet. Am Ende sichtbar: Splitter-Test grün in `task check`.

## Beteiligte und Zielgruppen

Alle, die `task check` brauchen (Entwickler, CI); Agent in INF und DEV.

## Anforderungen

B-406 › Anforderungen.

## Nicht-Ziele

Neue Cockpit-Funktionen; Umbau der Planungsseite; Lockerung der Lint-Regeln.

## Regeln und Einschränkungen

Datei ≤ 400 Zeilen, Funktion ≤ 60 (Komplexitäts-Budget). Neue Test-Abhängigkeiten (DOM-Umgebung) gehören in die DEV-Session und nur in `tools/k3c-dev/frontend`, nicht in den Root-Vitest, falls dort vermeidbar.

## Beispiele

Doppelklick auf den Splitter → Teilung springt auf den Standardwert. Ziehen jenseits der Grenzen → Teilung bleibt im erlaubten Bereich.

## Ausnahme- und Fehlerfälle

Lint-Fehler nur im Worktree (fehlende Pakete) → kein Code-Fix, Hinweis in der Arbeitsweise. localStorage nicht verfügbar → Standardwert.

## Akzeptanzkriterien

- **AC-01** Ein Test mit DOM-Umgebung prüft Doppelklick und Ziehen des Splitters (B-406).

## Offene Fragen

Neue devDependency `happy-dom` im Root (🧑 mit der Spec-Freigabe, siehe B-406). Hinweis: Mit nur einem Ticket kann der Sprint auch in einen anderen WZG-Sprint aufgehen (🧑).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ4.2 | `WZ4.2-splitter-dom-test.md` | Umsetzung | autonom | fertig |
| WZ4.3 | `WZ4.3-review.md` | Review | autonom | in Arbeit |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

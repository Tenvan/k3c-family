# WZ4 · INF, DEV · Lint grün mit Cockpit-Plugin, Splitter-Test in DOM-Umgebung

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** INF, DEV
- **Reife:** Entwurf
- **Tickets:** B-415, B-406
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach dem Merge des Cockpit-Plugins (PR #278) ist `task lint` rot: `tools/cockpit/hooks/core.ts` hat 570 Zeilen (Grenze 400), `src/tools/soundtest.ts:117` verletzt `no-unused-expressions` (B-415). Der Splitter der Planungsseite ist nur über `splitShare`/`clampSplit` getestet, Doppelklick und Ziehen nicht (B-406, im Root-Vitest fehlt jsdom/happy-dom).

## Ziel

`task check` ist wieder grün, und das Splitter-Verhalten der Planungsseite ist in einer DOM-Umgebung getestet. Am Ende sichtbar: `task lint` ohne Fehler, Splitter-Test grün.

## Beteiligte und Zielgruppen

Alle, die `task check` brauchen (Entwickler, CI); Agent in INF und DEV.

## Anforderungen

B-415, B-406 › Anforderungen. B-415 ist zuerst zu klären: Bestehen die Fehler schon auf `develop` oder nur im Worktree (fehlende Pakete in `tools/k3c-dev/frontend`)?

## Nicht-Ziele

Neue Cockpit-Funktionen; Umbau der Planungsseite; Lockerung der Lint-Regeln.

## Regeln und Einschränkungen

Datei ≤ 400 Zeilen, Funktion ≤ 60 (Komplexitäts-Budget). Neue Test-Abhängigkeiten (DOM-Umgebung) gehören in die DEV-Session und nur in `tools/k3c-dev/frontend`, nicht in den Root-Vitest, falls dort vermeidbar.

## Beispiele

Doppelklick auf den Splitter → Teilung springt auf den Standardwert. Ziehen jenseits der Grenzen → Teilung bleibt im erlaubten Bereich.

## Ausnahme- und Fehlerfälle

Lint-Fehler nur im Worktree (fehlende Pakete) → kein Code-Fix, Hinweis in der Arbeitsweise. localStorage nicht verfügbar → Standardwert.

## Akzeptanzkriterien

- **AC-01** `task lint` und `task check` laufen ohne Fehler; keine Datei über 400 Zeilen (B-415).
- **AC-02** Ein Test mit DOM-Umgebung prüft Doppelklick und Ziehen des Splitters (B-406).

## Offene Fragen

B-415 und B-406 hatten Projekt REL bzw. WZG; B-415 wurde auf WZG gesetzt (Cockpit-Werkzeug). 🧑 bestätigt?

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ4.1 | `WZ4.1-lint-cockpit.md` | Umsetzung | autonom | offen |
| WZ4.2 | `WZ4.2-splitter-dom-test.md` | Umsetzung | autonom | offen |
| WZ4.3 | `WZ4.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

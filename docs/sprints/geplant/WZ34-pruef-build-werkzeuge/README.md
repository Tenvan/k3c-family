# WZ34 · DEV · Prüf- und Build-Werkzeuge: Studio-Test, check:diff, buildall

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-617, B-621, B-629
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

## Ausgangslage

`task check:studio` ist auf `develop` rot (B-617), die Workbench-Test `TestSourcesUndConsoleTail` flackert (B-621), und `task buildall` baut die Web-EXEs von Studio und Workbench nicht mit (B-629). Details stehen in den Tickets.

## Ziel

Die Prüf- und Build-Wege der Werkzeuge sind verlässlich: Studio-Test grün, kein flackernder Workbench-Test, `task buildall` liefert alle EXEs auf dem Stand des Checkouts. Am Ende sichtbar: `task check:studio` grün, zwei `task buildall`-Läufe mit identischem, aktuellem `bin/`.

## Beteiligte und Zielgruppen

Wer an Werkzeugen entwickelt; Agent DEV und INF; 🧑 prüft, ob die Web-EXEs nach `task buildall` laufen.

## Anforderungen

`B-617 › Anforderungen`, `B-621 › Anforderungen`, `B-629 › Anforderungen`.

## Nicht-Ziele

Tempo von `check:diff` (B-619, umgesetzt, wartet auf Abnahme in WZ38).

## Regeln und Einschränkungen

Prüfen nur über `task`; Go-Test-Änderungen ohne neues Flackern (mehrfach laufen lassen); keine Rücksicht auf alte Spielstände.

## Beispiele

`task buildall` → `bin/k3c-workbench-web.exe` und `bin/k3c-studio-web.exe` tragen den Stand des Checkouts.

## Ausnahme- und Fehlerfälle

Eine laufende EXE sperrt das Ziel → `task buildall` nennt den Grund statt still zu überspringen.

## Akzeptanzkriterien

- **AC-01** `task check:studio` ist grün, der Test `levelView` passt zu `level-forest.json` (`B-617/AC-01`).
- **AC-02** `TestSourcesUndConsoleTail` ist unabhängig vom Zeitpunkt der Ressourcen-Zeile, 20 Läufe grün (`B-621/AC-01`).
- **AC-03** `task buildall` baut die Web-EXEs mit, alle EXEs in `bin/` stehen auf dem Stand des Checkouts (`B-629/AC-01` bis `AC-03`).

## Offene Fragen

keine.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ34.1 (DEV) Studio-Test und flackernder Workbench-Test beheben (AC-01, AC-02).
- WZ34.2 (INF) `task buildall` um die Web-EXEs ergänzen (AC-03).
- WZ34.3 Review, alle Kriterien, schließt ab.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

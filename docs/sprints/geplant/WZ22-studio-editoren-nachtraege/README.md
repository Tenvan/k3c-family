# WZ22 · DEV · Studio-Editoren: Fehler sichtbar, Im Spiel öffnen wiederholbar

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-487, B-491, B-499
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Im Chunk-Editor bleiben abgelehnte Aufrufe von `planLoad` und `planCheck` stumm (`tools/k3c-studio/frontend/src/world/usePlan.ts`, B-487). Aus den Reviews zu WZ17 liegen sieben Kleinigkeiten am Welten Editor offen (Schieber, Klick-Sprung, Planungsliste; B-491). „Im Spiel öffnen“ scheitert in einer eigenen Welt beim zweiten Mal mit demselben Seed (`save_exists`), die Standard-Welt lädt still den Stand einer anderen Welt (`tools/k3c-studio/backend/app_game.go`, B-499).

## Ziel

Die Editoren des Studios melden jeden Fehler sichtbar, und „Im Spiel öffnen“ ist in jeder Welt wiederholbar. Am Ende sichtbar: zweimal „Im Spiel öffnen“ in `nordmeer` ohne Fehlermeldung.

## Beteiligte und Zielgruppen

🧑 im Asset-Studio; Umsetzung durch Agent in DEV; 🧑 gibt die Spec frei.

## Anforderungen

B-487, B-491, B-499 › Ziel und Akzeptanzkriterien (Specs noch Entwurf).

## Nicht-Ziele

Neue Editor-Funktionen; Umbau der Lobby über das hinaus, was B-499 braucht; TLS-Fehler von `TestDownloadHit` (B-474, Duplikat des verworfenen B-480).

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-studio/`); braucht B-499 eine Änderung an Spielstand-Namen im Server (`engine/room`), wird das eine eigene SRV-Session vor WZ22.2. Oberfläche nach Atomic Design. Tests nicht abschwächen.

## Beispiele

`planLoad` abgelehnt → „Laden fehlgeschlagen: <Grund>“ am Editor. Standard-Welt und `nordmeer` mit Seed `abc` → zwei verschiedene Spielstände.

## Ausnahme- und Fehlerfälle

`planCheck` abgelehnt → Speichern gesperrt, letztes gültiges Bild bleibt. Spielstand-Name einer Welt existiert schon → laden statt `save_exists`.

## Akzeptanzkriterien

- **AC-01** Abgelehnte `planLoad` und `planCheck` zeigen ihren Grund am Editor, der Entwurf bleibt erhalten (B-487/AC-01, B-487/AC-02).
- **AC-02** Die sieben Punkte aus B-491 sind behoben oder bewusst verworfen, je Punkt mit Test (B-491).
- **AC-03** „Im Spiel öffnen“ startet in jeder Welt wiederholt ohne Fehler und lädt nie den Stand einer anderen Welt (B-499/AC-01, B-499/AC-02).

## Offene Fragen

🧑: Darf der Name des Spielstands die Welt enthalten (B-499; betrifft Spielstand-Namen im Server)?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ22.1 (DEV) Chunk-Editor meldet Ablehnungen, Nachträge Welten Editor (AC-01, AC-02).
- WZ22.2 (DEV) „Im Spiel öffnen“ je Welt wiederholbar, ggf. vorher eine SRV-Session für Spielstand-Namen (AC-03).
- WZ22.3 (DEV) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

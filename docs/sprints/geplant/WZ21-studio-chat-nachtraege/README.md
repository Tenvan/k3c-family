# WZ21 · DEV · Studio-Chat: eigene Umgebung, Grundgerüst als Instruktion, Entwurf geschützt

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-516, B-514
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus der Abnahme WZ19.5 (2026-10-20): Das Prompt-Grundgerüst steht heute als Text im Eingabefeld statt als feste Instruktion (B-516). Aus dem Review WZ19.4: Abbruch beendet nicht den Prozessbaum der CLI, ein Test-Riegel fehlt, der Mock setzt `from` anders als Go, ein Chat-Entwurf bleibt beim Weltwechsel stehen (B-514; Punkt 1 mit PR #400 erledigt). Die geerbte Umgebung und der doppelte Anmeldefehler (B-515) sind mit PR #406 behoben, B-515 ist erledigt.

## Ziel

Der Studio-Chat schickt das Grundgerüst selbst mit, und Abbruch, Mock und Weltwechsel verhalten sich sauber. Am Ende sichtbar: Ralf schreibt nur „Erstelle eine Welt Nordsee“ und bekommt einen Entwurf; ein Abbruch lässt keinen CLI-Prozess zurück.

## Beteiligte und Zielgruppen

🧑 im Asset-Studio; Umsetzung durch Agent in DEV (`tools/k3c-studio/`); 🧑 gibt die Spec frei und nimmt den Chat ab.

## Anforderungen

B-516, B-514 › Ausgangslage und Ziel (Specs noch Entwurf, Anforderungen beim Bereitmachen ausformulieren). B-515 ist mit PR #406 erledigt und gehört nicht mehr dazu.

## Nicht-Ziele

Neue Chat-Funktionen (Werkzeuge der CLI, weitere Aufträge außer Welten); Chat in k3c-dev.

## Regeln und Einschränkungen

Domäne DEV; Oberfläche nach Atomic Design (`tools/webkit`, `docs/arbeitsweise.md` › Oberflächen der Wails-Programme). Keine neue Abhängigkeit. CLI-Flags (`--append-system-prompt`) gegen die installierte CLI-Version prüfen. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Wunsch „Erstelle eine Welt Nordsee“ ohne Gerüst im Feld → Antwort im Welt-Format. Abbruch während der Antwort → kein CLI-Prozess bleibt übrig. Weltwechsel mit offenem Chat-Entwurf → Entwurf gehört nicht zur neuen Welt.

## Ausnahme- und Fehlerfälle

Abbruch während der Antwort → der ganze Prozessbaum endet. CLI-Version kennt `--append-system-prompt` nicht → klare Meldung statt stiller Antwort ohne Gerüst.

## Akzeptanzkriterien

- **AC-01** entfällt: B-515 ist mit PR #406 erledigt (geerbte Umgebung und doppelter Anmeldefehler behoben). Vorher: Der Chat startet die CLI ohne die Sitzungs-Variablen der startenden Claude-Sitzung; ein Anmeldefehler erscheint genau einmal (B-515).
- **AC-02** Das Grundgerüst ist eine feste Instruktion im Code und geht bei Welt-Aufträgen automatisch mit; das Eingabefeld enthält nur den Wunsch (B-516).
- **AC-03** Die offenen Punkte 2 bis 4 aus B-514 sowie die Nachträge aus B-514 › Notizen (Mock `mockWorldDraft.ts` setzt `from`, Chat-Entwurf beim Weltwechsel) sind behoben oder begründet verworfen, je Punkt mit Test oder Begründung (B-514; Punkt 1 ist mit PR #400 erledigt).

## Offene Fragen

🧑: Bleibt der Knopf „Prompt-Grundgerüst“ zum Ansehen erhalten?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ21.1 entfällt (eigene Umgebung und Anmeldefehler, B-515 mit PR #406 erledigt).
- WZ21.2 (DEV) Grundgerüst als feste Instruktion (AC-02).
- WZ21.3 (DEV) Restpunkte B-514: Prozessbaum, Test-Riegel, Mock, Weltwechsel (AC-03).
- WZ21.4 (DEV) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

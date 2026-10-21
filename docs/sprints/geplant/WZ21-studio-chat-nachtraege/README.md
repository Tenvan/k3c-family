# WZ21 · DEV · Studio-Chat: eigene Umgebung, Grundgerüst als Instruktion, Entwurf geschützt

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-515, B-516, B-514
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus der Abnahme WZ19.5 (2026-10-20): Der Studio-Chat reicht `os.Environ()` vollständig an `claude -p` weiter (`tools/k3c-studio/backend/chat_cli.go`); aus einer Claude-Sitzung gestartet, scheitert die Anmeldung, der Fehler erscheint doppelt (B-515). Das Prompt-Grundgerüst steht heute als Text im Eingabefeld statt als feste Instruktion (B-516). Aus dem Review WZ19.4: Chat-Entwürfe können die Planung einer Insel mit gleicher id verlieren, Abbruch beendet nicht den Prozessbaum, Test-Riegel fehlt (B-514).

## Ziel

Der Studio-Chat meldet sich unabhängig von der startenden Sitzung an, schickt das Grundgerüst selbst mit und verliert beim Übernehmen eines Entwurfs keine Planung. Am Ende sichtbar: Ralf schreibt nur „Erstelle eine Welt Nordsee“ und bekommt einen Entwurf, auch wenn das Studio aus k3c-dev in einer Claude-Sitzung gestartet wurde.

## Beteiligte und Zielgruppen

🧑 im Asset-Studio; Umsetzung durch Agent in DEV (`tools/k3c-studio/`); 🧑 gibt die Spec frei und nimmt den Chat ab.

## Anforderungen

B-515, B-516, B-514 › Ausgangslage und Ziel (Specs noch Entwurf, Anforderungen beim Bereitmachen ausformulieren).

## Nicht-Ziele

Neue Chat-Funktionen (Werkzeuge der CLI, weitere Aufträge außer Welten); Chat in k3c-dev.

## Regeln und Einschränkungen

Domäne DEV; Oberfläche nach Atomic Design (`tools/webkit`, `docs/arbeitsweise.md` › Oberflächen der Wails-Programme). Keine neue Abhängigkeit. CLI-Flags (`--append-system-prompt`) gegen die installierte CLI-Version prüfen. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Studio aus k3c-dev in einer Claude-Desktop-Sitzung gestartet → Chat antwortet mit eigener Anmeldung. Wunsch „Erstelle eine Welt Nordsee“ ohne Gerüst im Feld → Antwort im Welt-Format. Abbruch während der Antwort → kein CLI-Prozess bleibt übrig.

## Ausnahme- und Fehlerfälle

CLI nicht angemeldet → eine klare Meldung mit Hinweis auf `claude auth login`, nicht doppelt. Abbruch während der Antwort → der ganze Prozessbaum endet.

## Akzeptanzkriterien

- **AC-01** Der Chat startet die CLI ohne die Sitzungs-Variablen der startenden Claude-Sitzung (Test der Umgebung); ein Anmeldefehler erscheint genau einmal (B-515).
- **AC-02** Das Grundgerüst ist eine feste Instruktion im Code und geht bei Welt-Aufträgen automatisch mit; das Eingabefeld enthält nur den Wunsch (B-516).
- **AC-03** Die offenen Punkte 2 bis 4 aus B-514 sowie die Nachträge aus B-514 › Notizen (Mock `mockWorldDraft.ts` setzt `from`, Chat-Entwurf beim Weltwechsel) sind behoben oder begründet verworfen, je Punkt mit Test oder Begründung (B-514; Punkt 1 ist mit PR #400 erledigt).

## Offene Fragen

🧑: Welche Variablen werden entfernt (Liste fest oder alle `CLAUDE*`/`ANTHROPIC*`)? Bleibt der Knopf „Prompt-Grundgerüst“ zum Ansehen erhalten?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ21.1 (DEV) Eigene Umgebung für die CLI, Anmeldefehler einmal (AC-01).
- WZ21.2 (DEV) Grundgerüst als feste Instruktion (AC-02).
- WZ21.3 (DEV) Restpunkte B-514: Prozessbaum, Test-Riegel, Mock, Weltwechsel (AC-03).
- WZ21.4 (DEV) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

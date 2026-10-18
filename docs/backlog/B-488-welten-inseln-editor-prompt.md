# B-488 · Welten und Inseln im Asset-Studio anlegen und bearbeiten, Welt per Prompt füllen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wunsch von 🧑 (2026-10-18, Chat). Heute gibt es eine einzige Datei `data/islands.json` mit einer Liste von Inseln (heute nur Insel 1); der Editor des Asset-Studios (WZ8, WZ17) bearbeitet nur die geplanten Chunks einer Stufe. Es gibt keinen Begriff „Welt“ als eigene Datei, keinen Editor zum Anlegen und Ändern von Inseln und keine Hilfe, eine Welt mit einem Prompt zu füllen. Offen im Plan: Anzahl und Inhalt weiterer Inseln (`docs/rules/stufen.md` § 7); B-103 hat Inselwechsel gebaut, aber ausdrücklich keine weiteren Inseln. B-435 (Gebäude per Prompt im Asset-Manager) ist die nächste Verwandtschaft für den Prompt-Gedanken.

## Ziel

Im Asset-Studio lassen sich Welten (eine Welt hat n Inseln, jede Welt eine eigene JSON-Datei) und Inseln anlegen und bearbeiten, und eine Welt kann per Prompt gefüllt werden: Ein Button liefert ein Grundgerüst-Prompt, 🧑 erweitert es um eigene Wünsche und erhält daraus einen Entwurf der Welt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Wünsche von 🧑 (noch nicht entschieden, Spec-Runde offen):
- **Welten anlegen und bearbeiten:** Eine Welt hat n Inseln und liegt in einer eigenen JSON-Datei; mehrere Welten nebeneinander.
- **Inseln anlegen und bearbeiten:** Name, Stufen (Tiefen), Skalierung, geplante Chunks und, mit WZ17, die Chunk-Anzahl je Stufe.
- **Welt per Prompt füllen:** Ein Button erzeugt ein Grundgerüst-Prompt (Format der Welt-Datei, Regeln, Grenzen, gültige Arten); 🧑 ergänzt es um eigene Wünsche; das Ergebnis (JSON) wird geprüft und als ungespeicherter Entwurf geladen, nicht blind geschrieben.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidet 🧑 in einer Spec-Runde:
1. Wie verhält sich „Welt“ zu Spielstand und Raum (Entscheidung 003: ein Raum hat einen Spielstand mit n Inseln)? Wählt der Raum beim Anlegen eine Welt-Datei, und was passiert mit `data/islands.json` (wird zur Standard-Welt)? Berührt REG, SIM und SRV, nicht nur das Werkzeug.
2. Mehrere Inseln und Inselwechsel im Spiel (B-103 Nicht-Ziel, `stufen.md` § 7): Anzahl und Reihenfolge vorab klären.
3. Prompt: Läuft die KI außerhalb (Prompt wird kopiert, Antwort eingefügt) oder ruft das Studio selbst ein Modell auf? Annahme: kopieren und einfügen, ohne neue Abhängigkeit und ohne Schlüssel im Studio.
4. Wie viel darf die KI festlegen (Inselnamen, Stufen, Chunk-Pläne, Größen aus WZ17) und was bleibt fest (Werte, Bosse, Balancing)?
5. Reihenfolge zu B-485/WZ17 und B-468 (Ruinen, Gegnerlager): erst Chunk-Editor fertig, dann Welten.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

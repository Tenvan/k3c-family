# U3 · PLAT · Level-Betrachter

- **Status:** geplant
- **Domäne:** PLAT
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-092
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Es gibt Testseiten für Szenarien, Figuren und Grafiken, aber keine, die ein generiertes Level zeigt. Der Endpunkt dafür entsteht in U2 (B-092, B-091).

## Ziel

Die Seite `leveltest.html` zeigt Seed und Biom als Level-Streifen mit Warnungen und startet das Level im Spiel. Am Ende sichtbar: auf dem TV mit Controller Seed ändern, scrollen und „Im Spiel starten“.

## Beteiligte und Zielgruppen

Entwickler, 🧑 beim Balancing; Bedienung mit Tastatur und Controller.

## Anforderungen

B-092 › Anforderungen.

## Nicht-Ziele

Spielen, Level bearbeiten oder speichern (B-092 › Nicht-Ziele).

## Regeln und Einschränkungen

Domäne PLAT. Regel „Seiten & Navigation“ aus `CLAUDE.md` (`installPageChrome()`, Eintrag in `src/landing/pages.ts`, `toggleFullscreen()`, `openPage()`/`goHome()`). Voraussetzung: U2 abgeschlossen. Die Abnahme am TV macht nur 🧑.

## Beispiele

Seed `test`, Biom `forest` → 22 Abschnitte, zwei Portale, ein Ausgang.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar → Hinweistext statt leerer Seite (B-092 › Ausnahme- und Fehlerfälle).

## Akzeptanzkriterien

- **AC-01** Seite eingetragen, `installPageChrome()` vorhanden, Projektregeln grün (B-092/AC-01).
- **AC-02** Abbildungs-Funktion getestet (B-092/AC-02).
- **AC-03** Beispiel-Level stimmt mit `level_generate` überein (B-092/AC-03).
- **AC-04** Hinweis ohne Server (B-092/AC-04).
- **AC-05** 🧑 hat die Seite mit Controller am TV bedient (B-092/AC-05).

## Offene Fragen

keine blockierenden (Export der Positionen ist ausdrücklich kein Teil dieser Spec).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- U3.1 Abbildungs-Funktion (Abschnitte/Objekte → Zeichenmodell) mit Tests (AC-02, AC-03).
- U3.2 Seite `leveltest.html`: Eingabe, Zeichnen, Fehlerhinweis, Kachel (AC-01, AC-04).
- U3.3 Review (alle); AC-05 ist die Abnahme durch 🧑.

## Abnahme

–

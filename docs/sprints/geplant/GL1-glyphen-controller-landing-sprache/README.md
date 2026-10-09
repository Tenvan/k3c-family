# GL1 · PLAT, CLI · Tastensymbole je Controller, Landingpage in der gewählten Sprache

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** PLAT, CLI
- **Reife:** Entwurf
- **Tickets:** B-339, B-369
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Glyphen im Spiel und Hinweise auf den Seiten zeigen für jedes Pad Xbox-Beschriftung und -Farben; `Gamepad.id` wertet niemand aus (B-339). Die Landingpage bleibt nach dem Sprachwechsel deutsch, ihre Texte stehen als Literale in `src/landing/` und `index.html` (B-369). Beide Tickets ändern die Hinweise der Landingpage (`index.html`, Fußleiste) und werden deshalb gemeinsam umgesetzt.

## Ziel

Spiel und Seiten zeigen die Tastensymbole des Controllers, den der Spieler gerade benutzt, und die Landingpage folgt der gewählten Sprache. Am Ende sichtbar: Mit einem DualSense am PC zeigen Spiel, Landingpage und Shell ✕/□/△ und „Create + Options“, im Split-Screen mit gemischten Pads je Zelle die passenden Symbole; nach Sprachwechsel auf English zeigt die Landingpage nach dem Neuladen englische Texte.

## Beteiligte und Zielgruppen

Spieler am PC, TV oder Handy mit verschiedenen Controllern; Xbox bleibt Zielplattform und Standard. Agent in PLAT (`src/input/`, `src/landing/`, `index.html`, `src/core/shell.ts`, `src/tools/`) und CLI (`src/scenes/glyphs*.ts`). 🧑 entscheidet die unterstützten Familien und die Symbol-Quelle und nimmt am Gerät ab.

## Anforderungen

B-339 › Anforderungen; B-369 › Anforderungen. Reihenfolge: zuerst PLAT (Erkennung der Controller-Familie, Texte und Symbole der Seiten), dann CLI (Symbolsätze der Glyphen im Spiel je Spieler).

## Nicht-Ziele

Andere Tastenbelegung je Plattform; Controller ohne `mapping: 'standard'`; Bilddateien aus Hersteller-Packs mit unklarer Lizenz (B-339 › Nicht-Ziele); weitere Sprachen, Home-Button-Text und Touch-Overlay (B-215); Tasten von Spieler 2 an der Tastatur (B-371, AZ1).

## Regeln und Einschränkungen

- Domänen nacheinander: PLAT-Sessions zuerst, dann CLI. Was eine Session in der anderen Domäne bräuchte, wird ein Ticket.
- Belegung bleibt gleich (Standard-Mapping), nur die Darstellung ändert sich; B bzw. ○ bleibt unbelegt, View + Menu reserviert.
- `CLAUDE.md` › Seiten & Navigation (Landingpage bleibt offen, iframe). Texte über `t()` in de und en, Rückfall Deutsch.
- Kontrast ≥ 4,5:1 (bestehender Test), Mindest-Schriftgröße (B-136). Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit.
- Erst nach U6 (B-337, HUD-Elemente) sinnvoll; die Controller-Abnahme erst nach B-314 (B-339 › Regeln).

## Beispiele

B-339 › Beispiele; B-369 › Beispiele.

## Ausnahme- und Fehlerfälle

B-339 › Ausnahme- und Fehlerfälle (unbekannte ID → Xbox, Wechsel ohne Flackern); B-369 › Ausnahme- und Fehlerfälle (unbekannte Sprache → Deutsch, Übernahme spätestens beim Neuladen).

## Akzeptanzkriterien

- **AC-01** Die Zuordnung `Gamepad.id` → Familie ist getestet (Xbox-, DualShock/DualSense- und unbekannte IDs; Rückfall Xbox) (`B-339/AC-01`).
- **AC-02** `src/landing/*.ts` und `index.html` enthalten kein deutsches Text-Literal mehr, die Texte stehen in den Textdateien für de und en (`B-369/AC-01`).
- **AC-03** Landingpage, Shell und Werkzeug-Seiten zeigen die Symbole des zuletzt benutzten Controllers (`B-339/AC-04`, Browser-Pane).
- **AC-04** Für jede Familie liefert die Glyph-Funktion Beschriftung und Farben aller Glyph-Tasten; Kontrast-Test grün (`B-339/AC-02`).
- **AC-05** Im Spiel zeigt jede Zelle die Symbole des Controllers ihres Spielers (`B-339/AC-03`, Test der Zuordnung + Browser-Pane mit gemocktem Pad).
- **AC-06** Nach Sprachwechsel in den Optionen zeigt die Landingpage nach dem Neuladen die gewählte Sprache (`B-369/AC-02`, 🧑 am Gerät).
- **AC-07** 🧑 hat mit einem PlayStation-Controller am PC abgenommen (`B-339/AC-05`, nach B-314).

## Offene Fragen

- Welche Familien außer Xbox und PlayStation (Switch Pro, 8BitDo, Steam Controller, generisch)? 🧑, blockiert GL1.1. Vorschlag: `xbox`, `playstation`, Rest → `xbox`; weitere später als Ticket.
- Symbole selbst zeichnen (Kreis + Zeichen wie heute) oder ein CC0-Pack (z. B. Kenney Input Prompts)? 🧑, blockiert GL1.2 und GL1.3. Vorschlag: selbst zeichnen, keine neuen Assets.
- Grenze PLAT/CLI für die Beschriftungen: Vorschlag Familie und Tasten-Beschriftung je Familie in `src/input/` (PLAT, auch von den Seiten genutzt), Farben und Zeichnen in `src/scenes/glyphs*.ts` (CLI). 🧑, nicht blockierend.
- Übernimmt die Landingpage die Sprache schon beim Schließen der Spiel-Seite (ohne Neuladen)? 🧑, nicht blockierend (B-369 › Offene Fragen). Vorschlag: nur beim Neuladen, wie AC-06.
- Tastatur je Layout (DE/US) beschriften? 🧑, nicht blockierend; Vorschlag: nein, späteres Ticket.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- GL1.1 PLAT: Controller-Familie aus `Gamepad.id`, zuletzt benutzter Controller je Spieler und je Gerät (AC-01).
- GL1.2 PLAT: Landingpage-Texte über `t()`, Symbole der Familie auf Landingpage, Shell und Werkzeug-Seiten (AC-02, AC-03).
- GL1.3 CLI: Symbolsätze je Familie in den Glyphen, je Zelle die Familie ihres Spielers (AC-04, AC-05).
- GL1.4 Review (Code-Sprint, Domäne CLI): alle Kriterien prüfen.
- GL1.5 Workshop (🧑): Sprachwechsel am Gerät, Abnahme mit PlayStation-Controller am PC nach B-314 (AC-06, AC-07).

## Abnahme

–

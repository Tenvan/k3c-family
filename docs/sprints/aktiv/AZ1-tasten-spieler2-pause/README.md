# AZ1 · CLI · Tasten von Spieler 2 an der Tastatur, Pause-Anzeige

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-371, B-333
- **Start-Commit:** 87f25c20
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-10, 🧑 im Chat (Revision 1)

## Ausgangslage

Seit PL1.2 (B-316) spielt Spieler 2 an derselben Tastatur, seine Hinweise, Glyphen und Skill-Leiste zeigen aber die Tasten von Spieler 1 (B-371). Hält 🧑 einen Raum über `/dm` an, meldet der Zustand das schon (`devPaused`), im Spielbild fehlt aber eine Pause-Anzeige, und die Figuren laufen in ihrer Animation weiter (B-333).

## Ziel

Jede Zelle zeigt, was für ihren Spieler gerade gilt: die Tasten seines Tastatur-Layouts und einen angehaltenen Raum. Am Ende sichtbar: Am PC mit zwei Tastatur-Spielern nennt das Feld von Spieler 2 Enter, Pfeile und Ziffernblock; nach „⏸ Pause“ in `/dm` steht in jeder Zelle eine Pause-Anzeige und alle Figuren stehen still.

## Beteiligte und Zielgruppen

Zwei Spielende an einer Tastatur am PC; Spieler am TV oder PC, 🧑 als Spielleiter mit `/dm`. Agent in CLI (`src/scenes/`). 🧑 entscheidet das Aussehen der Pause-Anzeige und nimmt am PC ab.

## Anforderungen

B-371 › Anforderungen; B-333 › Anforderungen. Reihenfolge: zuerst B-371 (Tasten je Layout), dann B-333 (Pause-Anzeige, Animationen anhalten).

## Nicht-Ziele

Tastenbelegung ändern (B-316) oder frei einstellbare Tasten; Symbole je Controller-Familie (B-339, GL1); Pause im Couch-Raum durch den Server (B-214); Änderungen an Server oder Protokoll.

## Regeln und Einschränkungen

- Domäne CLI (`src/scenes/`), nur zeichnen (ADR 001). `src/input/keyboardLayouts.ts` (PLAT) wird nur gelesen und ist die einzige Quelle der Tasten (B-371).
- Pause nur aus dem vorhandenen Feld `devPaused` des Zustands; keine Protokoll-Änderung.
- Texte über `t()` in de und en; Mindest-Schriftgröße (B-136); 2 Spieler gleichzeitig.
- Datei ≤ 400, Funktion ≤ 60 Zeilen (`GameScene.ts` steht bei 398), keine neue Abhängigkeit.

## Beispiele

B-371 › Beispiele; B-333 › Beispiele.

## Ausnahme- und Fehlerfälle

B-371 › Ausnahme- und Fehlerfälle (nur ein Tastatur-Spieler → Layout 1); B-333 › Ausnahme- und Fehlerfälle (Verbindungsanzeige hat Vorrang, Zustand ohne Pause-Feld → keine Anzeige).

## Akzeptanzkriterien

- **AC-01** Test: Die Glyph für `confirm` und die Slot-Beschriftungen von Spieler 2 an der Tastatur nennen Enter bzw. Ziffernblock-Tasten (`B-371/AC-01`).
- **AC-02** Browser-Pane: Im Feld von Spieler 2 stehen keine Tasten von Spieler 1 (`B-371/AC-02`).
- **AC-03** Bei angehaltenem Raum zeigt jeder Spieler-Bildschirmbereich eine Pause-Anzeige, läuft der Raum, ist sie weg (`B-333/AC-01`, Test der Anzeige-Funktion + Beobachtung am PC).
- **AC-04** Während der Pause stehen die Figuren-Animationen still und laufen nach „Weiter“ wieder (`B-333/AC-02`, Beobachtung am PC).

## Offene Fragen

- ~~Aussehen der Pause-Anzeige~~ – entschieden 🧑 2026-10-10 (Chat): Text „Pausiert“ mittig je Zelle auf halbtransparentem Band, passend zu den HUD-Elementen aus U6.
- ~~Reihenfolge zu U6~~ – erledigt: U6 ist im Code fertig (nur Workshop U6.4 offen), AZ1 baut darauf auf.

## Sessions

- AZ1.1 Hinweise, Glyphen und Skill-Leiste von Spieler 2 aus `KEYBOARD_2` (AC-01, AC-02).
- AZ1.2 Pause-Anzeige je Zelle und angehaltene Figuren-Animationen bei `devPaused` (AC-03, AC-04).
- AZ1.3 Review (Code-Sprint): alle Kriterien prüfen.
- AZ1.4 Workshop (🧑): Abnahme am PC, zwei Tastatur-Spieler und Pause über `/dm` (AC-03, AC-04).

## Abnahme

–

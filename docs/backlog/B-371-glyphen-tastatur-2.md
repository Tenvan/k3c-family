# B-371 · Hinweise und Glyphen zeigen für Spieler 2 an der Tastatur dessen Tasten

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** AZ1
- **Projekt:** BED
- **Erstellt:** 2026-10-09
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-10, 🧑 im Chat (mit AZ1)

## Ausgangslage

Seit PL1.2 (B-316) spielt Spieler 2 an derselben Tastatur mit Pfeilen, Strg rechts, Enter und Ziffernblock (`src/input/keyboardLayouts.ts` › `KEYBOARD_2`). Hinweise, Aktionen-Overlay und Skill-Leiste lesen die Tasten aber aus `KEY_ACTIONS`/`slotBindings('keyboard')` (`src/scenes/glyphs.ts`, `actionHints.ts`, HUD) und zeigen im Feld von Spieler 2 „Leertaste“, Q/R/T/Z (Browser-Pane, PL1.2).

## Ziel

Jeder Tastatur-Spieler sieht in seinem Feld die Tasten seines Layouts.

## Beteiligte und Zielgruppen

Zwei Spielende an einer Tastatur am PC; Agent in CLI (`src/scenes/`).

## Anforderungen

- Glyphen, Aktionshinweise, Beitritts- und Steuerhinweis im Feld von Spieler 2 nennen Enter, Pfeile, Strg rechts und Ziffernblock 0–5.
- Spieler 1 bleibt unverändert; Quelle der Tasten ist `keyboardLayouts.ts` (keine zweite Liste in `src/scenes/`).

## Nicht-Ziele

Belegung ändern (B-316); frei einstellbare Tasten.

## Regeln und Einschränkungen

CLI zeichnet nur; Spiel-Code fragt Aktionen ab, nie Tasten; 2 Spieler gleichzeitig; Datei ≤ 400, Funktion ≤ 60 Zeilen (`GameScene.ts` steht bei 398).

## Beispiele

Spieler 2 steht am Bauplatz → Hinweis „Enter halten: …“ statt „Leertaste halten: …“; Skill-Leiste „0 · 1 · 2 · 3 · 4“ (Ziffernblock).

## Ausnahme- und Fehlerfälle

Nur ein Tastatur-Spieler → Layout 1 wie heute.

## Akzeptanzkriterien

- **AC-01** Test: die Glyph für `confirm` und die Slot-Beschriftungen von Spieler 2 an der Tastatur nennen Enter bzw. Ziffernblock-Tasten.
- **AC-02** Browser-Pane: im Feld von Spieler 2 stehen keine Tasten von Spieler 1.

## Offene Fragen

keine

## Notizen

Aus PL1.2 (Kontext: „Zeigen sie Layout 2 nicht ohne Änderung in `src/scenes/`, wird das ein Ticket“). Nummer von Hand B-371, weil B-370 auf `sprint/s8` vergeben ist.

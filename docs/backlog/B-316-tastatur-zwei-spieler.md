# B-316 · Zwei Spieler spielen an einer Tastatur im Split-Screen

- **Domäne:** PLAT
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** PL1
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`KeyboardInput` (`src/input/playerInput.ts`) ist ein einziger Spieler: A/D und die Pfeiltasten bewegen dieselbe Figur, Aktionen liegen auf Leertaste, E, Q, R, T, Z, K (`src/input/slotBindings.ts`). 🧑 prüft derzeit nur am PC mit Tastatur (B-314); Split-Screen mit zwei Spielern lässt sich so nicht abnehmen.

## Ziel

An einer Tastatur spielen zwei Spieler mit getrennten Layouts. Nutzen: Jede Mechanik und jede Abnahme mit 2 Spielern ist am PC ohne Controller prüfbar.

## Beteiligte und Zielgruppen

🧑 bei Abnahmen am PC; Agents beim Testen im Browser-Pane.

## Anforderungen

- Layout Spieler 1 und Layout Spieler 2 ohne gemeinsame Tasten (Vorschlag: Spieler 1 links A/D, Shift, Leertaste, E, Q, R, T, Z, K; Spieler 2 rechts Pfeiltasten, Strg rechts, Enter, Ziffernblock).
- Beitritt von Spieler 2 per eigener Bestätigen-Taste.
- Die Hilfe zeigt beide Layouts (Glyphen aus S6, sonst Text).
- Reservierte Tasten bleiben (Pos1 = zurück, F = Vollbild, Esc = Pause).

## Nicht-Ziele

Tastenbelegung frei einstellbar; Controller-Änderungen.

## Regeln und Einschränkungen

Spiel-Code fragt Aktionen ab, nie Tasten (`CLAUDE.md` › Struktur); 2 Spieler gleichzeitig; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Spieler 1 drückt Leertaste, Spieler 2 drückt Enter → beide im Split-Screen, A/D bewegt nur Spieler 1, Pfeile nur Spieler 2.

## Ausnahme- und Fehlerfälle

Tastatur-Ghosting bei vielen gleichzeitigen Tasten → Layout so wählen, dass übliche Kombinationen gehen; kein Fehler im Spiel.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Die Layouts von Spieler 1 und 2 haben keine gemeinsame Taste, und jede Aktion ist in beiden belegt.
- **AC-02** 🧑 spielt am PC mit zwei Spielern an einer Tastatur im Split-Screen, beide bewegen und handeln unabhängig (Beobachtung).

## Offene Fragen

- Genaues Layout Spieler 2: 🧑 (Vorschlag oben).

## Notizen

Anlass: Chat 2026-10-06 („Dort immer nur Tastatur anbieten mit Steuerungs-Layout für ein oder zwei Spieler“).

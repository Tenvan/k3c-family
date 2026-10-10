# B-389 · Der Beitritts-Hinweis für Spieler 2 wird nicht angezeigt

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-10, 🧑 im Chat (Befunde aus U5.4 freigegeben)

## Ausgangslage

Beobachtung 🧑 am 2026-10-10 bei U5.4 (PC, Chrome, Touch-Gerätemodus, 1 Spieler im Raum): Der Beitritts-Hinweis und der Reise-Text erscheinen nur für Spieler 1. Für die anderen Spieler wird nichts angezeigt, auch nicht in der normalen Browser-Version. Die Texte stehen in `src/core/texts.de.ts` (`hud.join.touch`, `hud.join.keyboard`, `hud.join.pad`, `hud.joinCenter`). Die Tastatur-Belegung für Spieler 2 ist in `src/input/keyboardLayouts.ts` (`KEYBOARD_2`, Beitreten = Enter). Der Hinweis zum Beitreten kennt also für Spieler 2 eine andere Taste als der Text für Spieler 1 (Leertaste).

## Ziel

Ein Spieler, der noch nicht beigetreten ist, sieht einen Hinweis mit der Taste, mit der er beitreten kann.

## Beteiligte und Zielgruppen

🧑 testet mit 2 Spielern am PC; Agent baut in `src/scenes/`.

## Anforderungen

- Spieler 2 sieht einen Beitritts-Hinweis mit seiner Taste (Tastatur: Enter, Controller: A).
- Der Hinweis verschwindet, sobald der Spieler beigetreten ist.

## Nicht-Ziele

Neue Beitritts-Wege; Änderung der Tastenbelegung.

## Regeln und Einschränkungen

Jede Mechanik muss mit 2 Spielern gleichzeitig funktionieren (`CLAUDE.md`); `src/scenes` rechnet nichts (`noSim.test.ts`); Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Raum mit 1 Spieler, Spieler 2 noch nicht da → Hinweis für Spieler 2 mit „Enter“ (Tastatur) sichtbar. Spieler 2 tritt bei → Hinweis weg.

## Ausnahme- und Fehlerfälle

Bereits 2 Spieler im Raum → kein Hinweis mehr nötig.

## Akzeptanzkriterien

- **AC-01** Ein Test belegt: Ohne Beitritt von Spieler 2 liefert die Hinweis-Logik einen Hinweis für Spieler 2 mit seiner Taste.
- **AC-02** 🧑 sieht am PC mit 2 Spielern den Hinweis und dessen Verschwinden (Beobachtung).

## Offene Fragen

- Soll der Hinweis für Spieler 2 dauerhaft erscheinen, solange nur 1 Spieler im Raum ist? Entscheidet 🧑.
- Welche Taste zeigt der Hinweis für Spieler 2: Enter (Tastatur-Belegung) oder Leertaste (Text für Spieler 1)? Vermutlich Enter; bestätigt 🧑.

## Notizen

Herkunft: U5.4, Frage 17 („nur der von Spieler 1“) und Frage 19 (Beitritt anderer Spieler). Verwandt: B-388 (Touch-HUD), U5.

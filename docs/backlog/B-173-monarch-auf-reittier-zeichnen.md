# B-173 · Der Client zeichnet den Monarchen auf dem Standard-Reittier

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** S7
- **Erstellt:** 2026-10-03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S7

## Ausgangslage

Der Monarch wird als einzelne Figur gezeichnet (`src/scenes/worldRenderer.ts`, Spieler-Sprites aus `data/sprites.json`). Die Reittiere (13 Tiere, Sattelpunkte, Reiter-Skalierung) sind in `sprites.json` › `mounts` beschrieben und in `src/tools/spriteReference.ts` referenzierbar, aber im Spiel nicht angeschlossen. Mit B-152 hat jeder Monarch ein Standard-Reittier.

## Ziel

Der Monarch erscheint beritten: Reittier-Sprite mit dem Reiter auf dem Sattelpunkt, Lauf-, Steh- und Sprint-Animation passend zur Bewegung. Nutzen: Spielbild wie im Vorbild.

## Beteiligte und Zielgruppen

Spieler am TV und am Handy (1–4 Monarchen, Split-Screen); Umsetzung durch Agent; 🧑 nimmt am Gerät ab.

## Anforderungen

- Reiter wird am Sattelpunkt des Reittiers gezeichnet (Oberkörper des Monarchen, wie `game-design.md` › Grafik beschreibt); Tiefe und Spiegelung folgen der Laufrichtung.
- Animationen: Stehen, Laufen, Sprinten; Bildrate und Tempo passen zur Geschwindigkeit aus dem Snapshot, der Client rechnet nichts.
- Das Reittier kommt aus `data/monarch.json` › `mount` (Sprite-Schlüssel), nicht aus dem Code.
- Funktioniert für 1–4 Spieler im Split-Screen und mit den Farben/Kennzeichen je Spieler.

## Nicht-Ziele

Neue Grafiken (Auswahl GR2), andere Reittiere als Auswahl, Reittiere für Truppen, Ton.

## Regeln und Einschränkungen

`src/scenes` zeichnet nur Snapshots; Schriftgrößen/Lesbarkeit nach B-136. Datei ≤ 400 Zeilen, Funktion ≤ 60. Lizenz der Reittier-Sprites beachten (LPC-Tiere CC-BY 3.0: Credits Pflicht, siehe `public/sprites/CREDITS.md`).

## Beispiele

Zwei Spieler im Split-Screen laufen nach rechts → beide Reittiere laufen animiert, der Reiter sitzt am Sattelpunkt; beim Sprint schneller animiert.

## Ausnahme- und Fehlerfälle

Sprite-Schlüssel unbekannt → Rückfall auf die bisherige Figur und ein Eintrag im Client-Log. Reittier-Frames fehlen → kein Absturz.

## Akzeptanzkriterien

- **AC-01** Test: Die Auswahl von Reittier-Sprite, Animation und Reiter-Position ist eine reine Funktion aus Snapshot und Daten (Sattelpunkt, Richtung, Tempo); Unbekanntes ergibt den Rückfall.
- **AC-02** Im Spiel (Dev-Server, Browser) erscheint jeder Monarch mit Reittier, in Stehen, Laufen und Sprint verschieden animiert.
- **AC-03** Zwei Spieler im Split-Screen sehen beide Reittiere korrekt, ohne dass der Client etwas rechnet (`noSim`-Test bleibt grün).
- **AC-04** 🧑 hat die Darstellung am TV und am Handy abgenommen.

## Offene Fragen

keine: Standard-Reittier ist das braune Pferd `horse` (Workshop F1.4, 2026-10-03, `docs/rules/monarch.md` § 7).

## Notizen

Aus Beschluss Q23 (2026-10-03).

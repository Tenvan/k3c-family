# Regelwerk: Klang

Beschlossen nach dem Live-Test von 🧑 am 2026-11-15 (B-935, Projekt SND): Der Klang war zu konsolenlastig, mit Pieptönen
und zu schnell für gemütliche Couch-Abende. Zuordnung der Klänge: `data/sounds.json` (erste Wahl je Slot klingt),
Katalog `docs/assets/sounds.md`, Urheber und Lizenz `public/audio/README.md`. Begriffe: [Glossar](../glossar.md)
(Klangbild, Mischpegel, Ducking).

## 1. Klangbild

**Regel:** Jeder Klang des Spiels passt zu einem ruhigen Abend auf der Couch, mehrere Leute vor einem Fernseher:

| Merkmal | Bedeutung |
|---|---|
| warm | Schwerpunkt in Bass und Mitten, keine spitzen Höhen |
| akustisch | echte Instrumente und Geräusche statt Synthesizer |
| Holz | Bauen, Schläge, Treffer klingen nach Holz, Stein und Werkzeug |
| Stoff | weiche, dumpfe Aufpralle (Gegner fallen, Schritte, Beutel) |
| Glocken | Warnungen und Ankündigungen als tiefe Glocke, nicht als Signalton |
| gedämpft | leise im Mix, keine Pegelspitzen; Musik tritt hinter das Spiel zurück |
| langsam | ruhige Tempi, weiche Ein- und Ausklänge, kein hektisches Wiederholen |

**Ausschlüsse:** Kein Slot nutzt als erste Wahl

- Synth-Piepen (Sinus- und Rechteck-Töne als Rückmeldung, „Powerup“-Fanfaren),
- 8-Bit und Chiptune,
- schrille Hochfrequenz (Laser, Phaser, Zap),
- Klänge der Kenney-Pakete „digital-audio“ und „interface-sounds“.

**Begründung:** Gleichmäßiger, leiser Klang ermüdet nicht und stört Gespräche im Raum nicht; Holz, Stoff und Glocken
passen zur Papierschnitt-Grafik (Entscheidung 005).

**Prüfung:** `native/k3c-game/audio/klang_test.go` prüft die erste Wahl jedes Slots gegen die Ausschlüsse (Namensteile
`kenney-digital-audio-`, `kenney-interface-sounds-`, `8-bit`, `chiptune`, `nes-`). Die Musik-Slots stehen dort als
ausdrückliche Ausnahme, bis B-936 ruhige Stücke liefert.

## 2. Mischpegel

**Regel:** Der Abschnitt `mischung` in `data/sounds.json` dämpft die Busse vor den Einstellungen des Spielers
(`effekte`, `musik`, je 0 … 1) und formt das Ducking bei Warnungen (`duckPegel` 0 … 1, `duckAnstieg`, `duckHalten`,
`duckAbklingen` in Sekunden). Das Ducking senkt die Musik weich und nur so weit, dass die Warnung hörbar bleibt; es
springt nicht.

**Begründung:** Die Einstellungen bleiben 0 … 1 für den Spieler; der Grundpegel des Spiels ist eine Mischentscheidung
und gehört in die Daten, nicht in den Code.

**Verweis:** `native/k3c-game/audio/register.go` (liest und prüft die Werte), `duck.go` (Ducking).

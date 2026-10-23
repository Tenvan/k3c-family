# B-554 · Entscheidung S1: Je Musikzustand spielt die erste Präferenz des Slots, die übrigen Kandidaten bleiben bis zur Bestätigung liegen

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** KR4
- **Projekt:** SND
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

SO4.1 (Workshop: Stücke wählen) ist Ralfs Sache. Damit SO4.3 bauen konnte, hat Claude autonom vorentschieden: je Zustand spielt die **erste Präferenz** des Slots `music:*` in `data/assets.json` (die Reihenfolge, die Ralf im Asset-Studio gesetzt hat). Lizenzen von Claude am 2026-10-22 auf den OpenGameArt-Seiten geprüft (alle CC0, Link `publicdomain/zero/1.0`); dabei die Urheber korrigiert (der Import hatte falsche Namen).

## Ziel

🧑 bestätigt die Auswahl (oder ändert die Reihenfolge im Asset-Studio) und gibt frei, die nicht gewählten Kandidaten zu entfernen.

## Beteiligte und Zielgruppen

🧑 hört und wählt (Geschmack, Q16); Claude räumt danach auf.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage). Lautheit am TV: SO4.5.

## Regeln und Einschränkungen

Nur CC0 oder CC-BY (Q15). Eine Änderung der Reihenfolge im Slot wirkt sofort, ohne Code (`musicTracks` liest das Register).

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A oder B gewählt und über das Entfernen der übrigen Kandidaten entschieden.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Passen die Stücke je Zustand?

| Zustand | Stück (Default) | Urheber | Lizenz |
|---|---|---|---|
| Tag | Happy Adventure (Loop) | TinyWorlds | CC0 |
| Abend | [Chiptune] Medieval: The Old Tower Inn | RandomMind | CC0 |
| Nacht | Haunting Chiptune Loop [Void Estate] | Zane Little Music | CC0 |
| Kampf | Pixel Sprinter | Zane Little Music | CC0 |
| Tiefe/Höhle | 8-Bit Cave Loop | Wolfgang_ | CC0 |
| Boss | NES Shooter Music: Boss | SketchyLogic | CC0 |
| Lobby | NES Shooter Music: Map | SketchyLogic | CC0 |
| Niederlage/Sieg | still (B-555) | – | – |

- **A (Default, gilt, umgesetzt):** wie Tabelle. Die übrigen Kandidaten (Tag +5, Nacht +1, Kampf +2, Boss +1, Lobby +1) bleiben unter `public/audio/kandidaten/` und im Slot, bis 🧑 bestätigt; SO4.3 hat sie bewusst **nicht** gelöscht (Auswahl noch nicht von 🧑).
- **B:** anderes Stück je Zustand: 🧑 zieht es im Asset-Studio an Platz 1.
- **Danach (A oder B):** Claude entfernt die nicht gewählten Musik-Kandidaten samt Credit-Zeilen (etwa 9 MB weniger), SO4.1 auf `fertig`. Zwei Stücke je Zustand (Q16 erlaubt 1–2) wären ein eigenes Ticket (Wechsel im Automaten).

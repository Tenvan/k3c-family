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

**Entscheidung 🧑 (KR4.1, 2026-10-23):** Auswahl bestätigt (Tabelle: Tag Happy Adventure, Abend The Old Tower Inn, Nacht Haunting Chiptune Loop, Kampf Pixel Sprinter, Tiefe/Höhle 8-Bit Cave Loop, Boss NES Shooter Boss, Lobby NES Shooter Map; Niederlage siehe B-555/B-573). Die übrigen Kandidaten bleiben vorerst liegen (Tag +5, Nacht +1, Kampf +2, Boss +1, Lobby +1, unter `public/audio/kandidaten/` und im Slot).

**Offen:** Aufräumen (Kandidaten samt Credit-Zeilen entfernen, ca. 9 MB, SO4.1 auf `fertig`) erst nach der Hörprobe SO4.5 und Freigabe durch 🧑. Dieses Ticket bleibt `eingeplant`, KR4.1 bleibt deshalb `offen`.

# B-637 · Ein Hafen-Chunk auf Inselstufe 1 baut das Boot zum Verlassen der Insel

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BS5
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑 mit BS2

## Ausgangslage

Heute öffnet der Sieg über den Endboss „Boot oder Portal“ zur nächsten Insel (`docs/rules/stufen.md` § 1, `docs/rules/bosse.md`); im Code meldet die Sim `SwitchReady`, `engine/room/island_switch.go` tauscht die Insel. Einen Hafen oder ein gebautes Boot gibt es nicht.

## Ziel

Auf Inselstufe 1 (Wald, Tiefe 0) liegt ein eigener **Hafen-Chunk**. Dort bauen die Spieler mit Münzen und Material das Boot; ist es fertig und der Endboss besiegt, verlassen alle Spieler gemeinsam am Hafen die Insel.

## Beteiligte und Zielgruppen

Spieler (Langzeitziel sichtbar ab Stufe 1), Claude, 🧑 entscheidet Lage und Bedingungen.

## Anforderungen

- Neue Chunk-Art `harbor` (geplanter Chunk, `docs/rules/geplante-chunks.md`) in `data/worlds/standard.json` › `plannedChunks` Tiefe 0, fester Platz am Rand einer Seite.
- Hafen-Gebäude/Steg mit Boot in Bauabschnitten; jeder Abschnitt kostet Münzen (Skala B-635) und Material (Holz, Stein, Eisen …, B-638).
- Inselwechsel: Boot fertig **und** Endboss besiegt → alle lebenden Spieler am Hafen → Wechsel (ersetzt „Boot oder Portal“).
- Hafen in Regeln, Glossar, Protokoll (Zustand Bauabschnitt), Spielstand.
- 2 Spieler, deterministisch.

## Nicht-Ziele

Seefahrt, Bootsfahrt-Animation über mehrere Szenen, Inhalte weiterer Inseln (stufen.md § 7).

## Regeln und Einschränkungen

Kette REG → SIM → CLI; Protokoll in eigener Session. Hub und `exit`/`edge` bleiben generator-eigen; der Hafen ist ein geplanter Chunk.

## Beispiele

Insel 1, Tag 3: Hafen-Chunk links außen sichtbar, Boot-Abschnitt 1 (Kiel) für 10 Münzen + 60 Holz baubar. Nach Endboss + letztem Abschnitt: beide Spieler am Steg → Inselwechsel.

## Ausnahme- und Fehlerfälle

Boot fertig, Endboss lebt → Hinweis „Endboss besiegen“. Ein Spieler fehlt am Steg → wartet. Gegner können Boot-Abschnitte nicht zerstören (Default) – offene Frage.

## Akzeptanzkriterien

- **AC-01** Regeln (`stufen.md`, `geplante-chunks.md`, `welten.md`) und Glossar beschreiben Hafen, Boot und Wechselbedingung.
- **AC-02** Generator-Test: Insel 1 Tiefe 0 hat genau einen Hafen-Chunk am geplanten Platz.
- **AC-03** Sim-Test: Wechsel nur mit fertigem Boot und besiegtem Endboss, alle Spieler am Hafen.
- **AC-04** Protokoll und Spielstand tragen den Boot-Fortschritt (Golden aktualisiert).
- **AC-05** Client zeichnet Hafen und Boot je Abschnitt.

## Offene Fragen

- Seite und Platz des Hafens (Vorschlag: äußerster planbarer Platz links, −7)? → 🧑, Default: −7.
- Anzahl Boot-Abschnitte (Vorschlag 3: Kiel, Rumpf, Segel)? → 🧑, Default: 3.
- Können Gegner das Boot beschädigen? Default: nein.
- Portal als Alternative entfällt? Default: ja, nur Boot.

## Notizen

Wunsch 🧑 2026-10-26: „Ein Hafen um die Insel zu verlassen muss noch her, dafür ein eigener Hafen-Chunk auf Insel-Ebene 1.“

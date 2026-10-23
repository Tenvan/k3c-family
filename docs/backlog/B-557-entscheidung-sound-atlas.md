# B-557 · Entscheidung S4: Die Effekte laden aus einem Sound-Atlas, Einzeldateien sind der Rückfall

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

B-494 fragt, ob sich ein Sound-Atlas lohnt oder Einzeldateien mit Vorladen reichen; messen sollte 🧑 am TV (SO2.5). SO6.2 hat den Atlas autonom gebaut (Task `gen:soundatlas`), der Client nutzt ihn mit Rückfall auf Einzeldateien.

## Ziel

🧑 bestätigt den Atlas nach der Hörprobe am TV oder verwirft ihn.

## Beteiligte und Zielgruppen

🧑 misst und entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

ffmpeg ist Voraussetzung des Tasks (schon für den alten Atlas, `public/audio/README.md`).

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A oder B gewählt (nach SO2.5/SO6.6).

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Bleibt der Sound-Atlas?

- **A (Default, gilt, umgesetzt):** ja; zwei Anfragen für alle Effekte statt einer je Datei. Nach jeder Änderung der Effekt-Slots `task gen:soundatlas` (ein Test meldet einen veralteten Atlas).
- **B:** Atlas verwerfen, Einzeldateien laden (Client fällt ohnehin darauf zurück, wenn `atlas.json` fehlt).
- **Wenn 🧑 B wählt:** Atlas-Dateien und Task entfernen, eine kleine Session.

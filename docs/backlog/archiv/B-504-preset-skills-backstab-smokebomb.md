# B-504 · Die Skills backstab und smokeBomb des Presets Dieb stehen in data/monarch.json

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SK2
- **Projekt:** SKL
- **Erstellt:** 2026-10-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Lücke 6 der Monarch-Seite (WZ10.5, 🧑 am 2026-10-19): Das Preset Dieb nennt die Skills `backstab` und `smokeBomb`, sie fehlen in `data/monarch.json › skills`. Die Monarch-Seite zeigt sie als „Skill fehlt in data/monarch.json“.

## Ziel

Jedes Preset nennt nur Skills, die es in `data/monarch.json` gibt; die Monarch-Seite zeigt keine Lücke 6 mehr.

## Beteiligte und Zielgruppen

Spielende (Startverteilung), Entwickler (Daten), 🧑 entscheidet über den Dieb-Inhalt.

## Anforderungen

- Default (autonom, Claude, 2026-10-23; Bestätigung 🧑 über KR5): `presets.thief.activeSkills` leer, bis der Dieb-Inhalt kommt.
- Das Laden der Daten prüft, dass jedes Preset nur vorhandene Skills nennt, und scheitert sonst mit Meldung (Start und `-check-data`).

## Nicht-Ziele

Dieb-Skills bauen (Backstab, Smoke Bomb; eigenes Ticket, falls 🧑 es will), Balancing.

## Regeln und Einschränkungen

SIM legt Daten-Felder an; deterministisch; `docs/rules/monarch.md`: Dieb folgt später als Inhalt.

## Beispiele

Preset Dieb gewählt → Basiswerte, keine Skills, keine Warnung.

## Ausnahme- und Fehlerfälle

Preset nennt unbekannten Skill → Laden scheitert: `data/monarch.json: preset thief nennt unbekannten Skill backstab`.

## Akzeptanzkriterien

- **AC-01** Go-Test: Jedes Preset aus `data/monarch.json` nennt nur Skills aus `skills`; eine Prüfung mit unbekanntem Skill liefert einen Fehler mit Preset und Skill.

## Offene Fragen

Dieb bauen oder Preset anpassen: Default Preset anpassen, 🧑 bestätigt über KR5.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

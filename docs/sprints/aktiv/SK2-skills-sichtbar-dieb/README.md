# SK2 · REG, SIM, CLI · Skills im Spiel sichtbar, Dieb-Skills in den Daten

- **Status:** aktiv
- **Projekt:** SKL
- **Domäne:** REG, SIM, CLI
- **Reife:** bereit
- **Tickets:** B-504, B-503
- **Start-Commit:** 32b70302
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Aus der Abnahme der Monarch-Seite WZ10.5 (🧑, 2026-10-19): Die 24 Skills aus `data/monarch.json` (Tank 9, Zauberer 8, Heiler 7) haben im Spiel keine Darstellung, nur `noTarget` bei `castFailed` (B-503). Das Preset Dieb nennt `backstab` und `smokeBomb`, die in `data/monarch.json › skills` fehlen (B-504).

## Ziel

Jeder Skill ist im Spiel sichtbar, wenn er wirkt (aktiv) oder gelernt wird (passiv), und jedes Preset nennt nur Skills, die es gibt. Am Ende sichtbar: Ring je Klasse am TV, Monarch-Seite ohne Lücke 6 (Lücke 5 der Studio-Seite: B-562).

## Beteiligte und Zielgruppen

Spielende am TV, zwei Spieler; 🧑 entscheidet Dieb und Darstellung; Agent in REG, SIM und CLI.

## Anforderungen

B-504 › Anforderungen, B-503 › Anforderungen. Sprint-eigen: Die Entscheidungen der Offenen Fragen gelten als Default (autonom, Claude, 2026-10-23), bis 🧑 sie in SK2.1 bestätigt oder ändert.

## Nicht-Ziele

Neue Klassen oder Skills, auch die Dieb-Skills (B-560); Monarch-Animationen (GR9, B-505); Werte-Runde der Skills (BAL); Studio-Seite (B-562).

## Regeln und Einschränkungen

Feature-Kette REG → SIM → CLI. Neue Skills brauchen Wirkung in `engine/sim` und Felder in `data/monarch.json` (SIM legt an). Die Darstellung zeichnet nur Ereignisse und Snapshot, rechnet nichts. Deterministisch, 2 Spieler.

## Beispiele

Zauberer wirkt Fireball → Geschoss und Treffer sichtbar. Preset Dieb gewählt → nur vorhandene Skills, keine Warnung „Skill fehlt“.

## Ausnahme- und Fehlerfälle

Skill ohne eigene Darstellung → gemeinsamer Rückfall-Effekt je Klasse. Preset nennt unbekannten Skill → Laden der Daten scheitert mit Meldung.

## Akzeptanzkriterien

- **AC-01** 🧑 hat entschieden, ob `backstab` und `smokeBomb` gebaut werden oder das Preset Dieb angepasst wird; Regeltext nachgezogen (B-504). Default umgesetzt: Preset angepasst.
- **AC-02** `data/monarch.json` und Simulation passen zu den Presets; ein Test prüft, dass jedes Preset nur vorhandene Skills nennt, das Laden scheitert sonst mit Meldung (B-504).
- **AC-03** Jeder der 24 Skills hat im Spiel eine Darstellung oder den festgelegten Rückfall (Ring je Klasse; ein Test prüft jede Linie aus `data/monarch.json`); Abnahme am TV durch 🧑 (B-503).

## Offene Fragen

Vorentschieden (autonom, Claude, 2026-10-23), Bestätigung durch 🧑 in SK2.1 (Frage-Ticket im Sprint KR5):

- **Dieb:** Preset anpassen statt bauen. `presets.thief.activeSkills` wird leer, bis der Dieb-Inhalt kommt (`docs/rules/monarch.md`: „der Dieb folgt später als Inhalt“). `backstab` und `smokeBomb` (Unsichtbarkeit, „von hinten“) wären neue Mechaniken mit Balancing-Wirkung. Kein Spieler startet heute als Dieb (`presetFor`).
- **Darstellung:** je Klasse gemeinsam (Ring in der Farbe der Linie an der Figur), kein Effekt je Skill. Aktive Skills beim Wirken, Passive beim Lernen.
- **Ereignisse:** keine neuen. Der Client erkennt das Wirken am Anstieg der Abklingzeit eines Slots zwischen zwei Zuständen und das Lernen an einem neuen Eintrag in `skills` (reiner Vergleich wie `hubRaised`, SO6.4). Keine Protokoll-Session.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SK2.1 | `SK2.1-workshop-dieb-darstellung.md` | Workshop | Mensch | offen |
| SK2.2 | `SK2.2-preset-dieb-pruefung.md` | Umsetzung | autonom | fertig |
| SK2.3 | `SK2.3-skill-darstellung.md` | Umsetzung | autonom | offen |
| SK2.4 | `SK2.4-review.md` | Review | autonom | offen |
| SK2.5 | `SK2.5-abnahme-tv.md` | Workshop | Mensch | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

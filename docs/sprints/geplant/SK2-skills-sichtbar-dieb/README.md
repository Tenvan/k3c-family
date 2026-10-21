# SK2 · REG · Skills im Spiel sichtbar, Dieb-Skills in den Daten

- **Status:** geplant
- **Projekt:** SKL
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-504, B-503
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus der Abnahme der Monarch-Seite WZ10.5 (🧑, 2026-10-19): Die 24 Skills aus `data/monarch.json` (Tank 9, Zauberer 8, Heiler 7) haben im Spiel keine Darstellung, nur `noTarget` bei `castFailed` (B-503). Das Preset Dieb nennt `backstab` und `smokeBomb`, die in `data/monarch.json › skills` fehlen (B-504).

## Ziel

Jeder Skill ist im Spiel sichtbar, wenn er wirkt, und jedes Preset nennt nur Skills, die es gibt. Am Ende sichtbar: Skills am TV mit eigener Darstellung, Monarch-Seite ohne Lücken 5 und 6.

## Beteiligte und Zielgruppen

Spielende am TV, zwei Spieler; 🧑 entscheidet Dieb und Darstellung; Agent in REG, SIM und CLI.

## Anforderungen

B-504, B-503 › Ausgangslage (Specs noch Entwurf, Anforderungen im Workshop festlegen).

## Nicht-Ziele

Neue Klassen oder Skills über den Dieb hinaus; Monarch-Animationen (GR9, B-505); Werte-Runde der Skills (BAL).

## Regeln und Einschränkungen

Feature-Kette REG → SIM → CLI. Neue Skills brauchen Wirkung in `engine/sim` und Felder in `data/monarch.json` (SIM legt an). Die Darstellung zeichnet nur Ereignisse und Snapshot, rechnet nichts. Deterministisch, 2 Spieler.

## Beispiele

Zauberer wirkt Fireball → Geschoss und Treffer sichtbar. Preset Dieb gewählt → nur vorhandene Skills, keine Warnung „Skill fehlt“.

## Ausnahme- und Fehlerfälle

Skill ohne eigene Darstellung → gemeinsamer Rückfall-Effekt je Klasse. Preset nennt unbekannten Skill → Laden der Daten scheitert mit Meldung.

## Akzeptanzkriterien

- **AC-01** 🧑 hat entschieden, ob `backstab` und `smokeBomb` gebaut werden oder das Preset Dieb angepasst wird; Regeltext nachgezogen (B-504).
- **AC-02** `data/monarch.json` und Simulation passen zu den Presets; ein Test prüft, dass jedes Preset nur vorhandene Skills nennt (B-504).
- **AC-03** Jeder der 24 Skills hat im Spiel eine Darstellung oder den festgelegten Rückfall; Abnahme am TV durch 🧑 (B-503).

## Offene Fragen

🧑: Dieb-Skills bauen oder Preset ändern? Darstellung je Skill einzeln oder je Klasse gemeinsam? Braucht die Darstellung neue Feedback-Ereignisse (dann eigene SRV-Session für das Protokoll)?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SK2.1 (REG, Mensch) Workshop: Dieb-Skills und Darstellung der Skills (AC-01).
- SK2.2 (SIM) Dieb-Skills in Daten und Simulation oder Preset angepasst, Test der Presets (AC-02).
- SK2.3 (CLI) Darstellung der Skills im Spiel (AC-03).
- SK2.4 (CLI) Review (Code-Sprint): alle Kriterien prüfen.
- SK2.5 (CLI, Mensch) Abnahme am TV (AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

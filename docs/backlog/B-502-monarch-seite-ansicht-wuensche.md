# B-502 · Monarch-Seite: einheitliches Sheet, Zoom, zweispaltige Ansicht

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ23
- **Projekt:** WZG
- **Erstellt:** 2026-10-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Aus der Abnahme WZ10.5 (Wünsche von 🧑, 2026-10-19). Die Monarch-Seite (`tools/k3c-studio/frontend/src/monarch/`) zeigt die vier Spielerfarben aus `data/sprites.json › players`: Spieler 1 und 3 nutzen das Sheet `medieval-king` (Spieler 3 mit tint `#9ecbff`), Spieler 2 und 4 das Sheet `medieval-king-2` (Spieler 4 mit tint `#ffb3c8`). Die Bühne hat keine Zoom-Funktion. Bühne und Skill-Tabelle stehen untereinander (`.wd-monarch` in `styles/world.css`).

## Ziel

Die Monarch-Seite lässt sich besser ansehen und vergleichen: alle Monarchen auf demselben Sheet, Zoom in der Bühne, Skills und Monarchen nebeneinander.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Alle vier Monarchen basieren auf `medieval-king`; jeder unterscheidet sich nur durch seine Farbe (tint), nicht durch ein anderes Sheet.
- Die Monarch-Ansicht hat Zoom: Alt + Mausrad und zusätzlich eine Zoom-Leiste; Zurücksetzen auf Spielmaßstab möglich.
- Zweispaltiges Layout: links die Skills, rechts die Monarchen (Bühne, Steuerung, Spielerfarben); bei schmalem Fenster untereinander.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

1. Wunsch 1 weicht vom Spiel ab: Dort nutzen Spieler 2 und 4 `medieval-king-2` (`data/sprites.json › players`). Soll nur die Studio-Vorschau abweichen (dann zeigt sie nicht mehr, was das Spiel zeigt) oder soll das Spiel selbst umgestellt werden (Daten in `data/sprites.json`, Tints für Spieler 2 und 4 neu festlegen, ggf. Golden-Dateien)? Entscheidet 🧑; Domäne dann DEV oder GRA/CLI.
2. Zoom nur Ansicht (Bodenlinie und Skala skalieren mit) oder auch die Bühnengröße ändern?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

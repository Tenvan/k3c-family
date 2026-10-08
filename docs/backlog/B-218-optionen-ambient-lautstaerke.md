# B-218 · Die Optionen-Szene regelt auch die Lautstärke des Ambient-Busses

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SO4
- **Projekt:** SND
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-08, Chat, durch 🧑, mit Sprint SO4

## Ausgangslage

SO1.1 legt den Mixer (`src/audio/mixer.ts`) mit drei Bussen an und ergänzt `Settings.ambientVolume` (Standard 100 %) in `src/core/settings.ts`. Die Optionen-Szene (S5, `src/scenes/optionsLogic.ts`) zeigt nur Musik und Effekte.

## Ziel

Der Spieler stellt die Ambient-Lautstärke in den Optionen ein, sobald es Ambient-Ton gibt.

## Beteiligte und Zielgruppen

Spieler am TV und Handy.

## Anforderungen

- Eintrag „Ambient“ mit ◀ ▶ in 10er-Schritten wie Musik und Effekte, Texte in `texts.de.ts` und `texts.en.ts`.

## Nicht-Ziele

Ambient-Klänge selbst (SO2, SO4).

## Regeln und Einschränkungen

Texte aus den zentralen Textdateien; B nicht belegen; mit 2 Spielern bedienbar.

## Beispiele

Ambient auf 0 % → Ambient-Bus stumm, Musik und Effekte unverändert.

## Ausnahme- und Fehlerfälle

Kaputter Speicher → Standard 100 % (wie bei den anderen Werten).

## Akzeptanzkriterien

- **AC-01** Die Optionen zeigen Ambient; ein Test auf `optionsLogic` belegt Schrittweite und Grenzen 0–100 %.

## Offene Fragen

Erst sinnvoll, wenn es Ambient-Ton gibt (SO2/SO4); Entscheidung 🧑.

## Notizen

Ticket-Nummern B-216 und B-217 sind auf den Branches `sprint/s1` und `sprint/gr5` schon vergeben.

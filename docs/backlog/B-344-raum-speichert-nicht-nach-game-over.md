# B-344 · Der Raum speichert nach „Komplett verloren“ nicht mehr

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit K2.2b endet eine Insel im Niederlage-Modus „Komplett verloren“ mit `Island.Over = true` und dem Ereignis `gameOver` (`engine/sim/defeat.go`); `StepIsland` ändert danach nichts mehr. `engine/room/` speichert aber weiter (Autosave bei `dawn`, beim Verlassen) und würde den Stand nach dem Game Over über den letzten Spielstand schreiben.

## Ziel

Nach einem Game Over bleibt der zuletzt gespeicherte Spielstand unverändert (`docs/rules/stufen.md` § 4, B-102/AC-03).

## Beteiligte und Zielgruppen

Spieler (Koop), SRV; 🧑 gibt die Spec frei.

## Anforderungen

- Ist `Island.Over` gesetzt, schreibt der Raum keinen Spielstand mehr (weder Autosave noch Speichern beim Verlassen).
- Der Raum endet für alle Geräte gleich; wie der Client das Ende zeigt, regeln K4/K5.

## Nicht-Ziele

Anzeige des Endes (K5), Neustart nach Game Over, Protokollfeld für `gameOver` (K4).

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`), `engine/sim/` bleibt unverändert; Logging mit Emoji (🛑/🚫).

## Beispiele

Ultra, Burg fällt → `gameOver` → Spieler verlassen den Raum → `saves/<name>.json` hat dieselben Bytes wie vor dem Game Over.

## Ausnahme- und Fehlerfälle

Game Over ohne vorherigen Spielstand → es entsteht keine Datei.

## Akzeptanzkriterien

- **AC-01** Test in `engine/room/`: nach `Island.Over` bleibt die Spielstand-Datei bei Autosave und Verlassen unverändert.

## Offene Fragen

Schließt der Raum nach dem Game Over sofort, oder bleibt er offen, bis alle gegangen sind (🧑)?

## Notizen

Angelegt in K2.2b (Sprint K2), soll vor dem Abschluss von K4 erledigt sein.

# B-102 · Siegvarianten und Niederlage-Modi der Raum-Optionen sind umgesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Eine Kampagne hat kein Ende; die Niederlage ist fest (`castleFallen`, `engine/sim/world.go`).

## Ziel

Das Ziel des Raums (Endboss, Gold, Tage, alles abbauen, alles ausbauen) beendet das Spiel mit Sieg; der Niederlage-Modus (Gold/Material-Verlust, Stufenverlust, Komplett verloren) bestimmt die Folgen eines gefallenen Hubs. Nutzen: Spielrunden haben einen Bogen und eine einstellbare Härte.

## Beteiligte und Zielgruppen

Spieler und Entwickler.

## Anforderungen

- Siegprüfung je Variante nach `docs/rules/stufen.md` § 3 (Startwerte Gold 1000, Tage 20, alles abbauen, alles ausbauen); Ereignis `victory`.
- Niederlage-Modi nach `stufen.md` § 4; `castleFallen` wird der Modus „Stufenverlust“; „Komplett verloren“ beendet den Raum (Game Over, Spielstand bleibt).
- Standard je Schwierigkeitsgrad (Dev/Leicht Gold/Material, Normal/Hart Stufenverlust, Ultra komplett verloren), je Raum überschreibbar.

## Nicht-Ziele

Bosse (B-103), Anzeige (CLI), Raum-Optionen als Daten (B-101).

## Regeln und Einschränkungen

`docs/rules/stufen.md`; deterministisch; mit 2+ Spielern. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Raum Variante „20 Tage überleben“, Tag 21 erreicht → `victory`. Ultra, Burg der Oberwelt fällt → Raum endet.

## Ausnahme- und Fehlerfälle

Zwei Siegbedingungen gleichzeitig erfüllt → ein `victory` je Tick. Game Over mit Spielern in anderen Stufen → alle sehen das Ende.

## Akzeptanzkriterien

- **AC-01** Test je Siegvariante: Bedingung nicht erfüllt, erfüllt, Ereignis genau einmal.
- **AC-02** Test je Niederlage-Modus: Folgen laut Regelwerk (Gold, Material, Bauten, Truppen, Landstreicher bleiben).
- **AC-03** Test: Komplett verloren beendet den Raum und lässt den letzten Spielstand unverändert.
- **AC-04** Standardzuordnung Grad → Niederlage-Modus und Überschreiben je Raum getestet.

## Offene Fragen

Zählt „Gold sammeln“ nur eingesammeltes Gold oder auch Drops anderer Spieler (R1.3: Summe aller eingesammelten Münzen)?

## Notizen

Aus R1.3. Abhängig von B-101 (Optionen im Raum).

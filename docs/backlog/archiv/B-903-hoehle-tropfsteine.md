# B-903 · Die Höhle zeigt Tropfsteine nach der Natur-Eigenschaft stalactites

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** BI2
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Höhle hat seit B-891 die Natur-Eigenschaft `stalactites` (Tropfsteine 0–3, nur Darstellung). Es gibt noch keine Tropfstein-Deko.

## Ziel

Das native Spiel zeigt Tropfsteine als Deko, ihre Menge folgt dem Wert der Stufe; fehlende Assets als PAP-Ticket.

## Beteiligte und Zielgruppen

Spieler der Höhle; Claude setzt um; Ralf 🧑 prüft im Abnahme-Ticket.

## Anforderungen

- Biome mit `stalactites` zeichnen Tropfsteine, die von der Decke (B-902) hängen: Dreiecke mit der Spitze nach unten.
- Raster alle 2 Units; an einem Rasterplatz hängt ein Tropfstein, wenn `mix(x) % 6 < Wert` (Wert 0 = keiner, 3 = etwa jeder zweite Platz). Länge 0,6–1,8 Units, Breite 0,4–0,8 Units aus der Position.
- Die Menge bei kleinerem Wert ist eine Teilmenge der Menge bei größerem Wert (gleiche Plätze bleiben).

## Nicht-Ziele

Stalagmiten am Boden; Herabfallen oder Schaden.

## Regeln und Einschränkungen

Wie B-902.

- **Bereich:** `native/k3c-game/props/decor.go`, `native/k3c-game/props/decor_test.go`
- **Abhängig von:** B-902
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Gleicher Ausschnitt, Wert 0 / 1 / 3 → 0 / etwa n / etwa 3n Tropfsteine.

## Ausnahme- und Fehlerfälle

Biom ohne `stalactites` → keine. Ohne `ceilingHeight` hängen sie an der Decke mit Wert 2.

## Akzeptanzkriterien

- **AC-01** Wert 0 plant keine Tropfsteine, Biome ohne die Eigenschaft ebenfalls (Unit-Test).
- **AC-02** Die Anzahl im selben Ausschnitt steigt streng mit dem Wert (1 < 2 < 3), die Orte sind bei gleichem Wert gleich (Unit-Test).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

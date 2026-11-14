# B-905 · Der Eisenstollen zeigt Hitze und Glut nach der Natur-Eigenschaft glow

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

Der Eisenstollen hat seit B-891 die Natur-Eigenschaft `glow` (Hitze und Glut 0–3, nur Darstellung).

## Ziel

Das native Spiel zeigt Glut-Deko und Hitzeflimmern nach dem Wert der Stufe.

## Beteiligte und Zielgruppen

Spieler des Eisenstollens; Claude setzt um; Ralf 🧑 prüft im Abnahme-Ticket.

## Anforderungen

- Biome mit `glow` zeichnen Glutpunkte am Boden: Raster 1,5 Units, Glut wenn `mix(x) % 8 < 2 · Wert`; Kreise 0,08–0,18 Units, 0,05–0,4 Units über dem Boden, orange-rot, Deckkraft pulsiert mit der Uhr (Phase aus der Position).
- Dazu ein Hitze-Schleier am Boden (Rechteck 1,2 Units hoch, rot, Deckkraft 0,06 · Wert).

## Nicht-Ziele

Schaden durch Hitze; Lichtquellen in `world/light.go`.

## Regeln und Einschränkungen

Wie B-902.

- **Bereich:** `native/k3c-game/props/decor_glow.go`, `native/k3c-game/props/decor_test.go`
- **Abhängig von:** B-891
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Eisenstollen mit Wert 0 → keine Glut, kein Schleier; mit 3 → dichte Glut und deutlicher Schleier.

## Ausnahme- und Fehlerfälle

Biom ohne `glow` → nichts.

## Akzeptanzkriterien

- **AC-01** Wert 0 und Biome ohne `glow` planen weder Glut noch Schleier (Unit-Test).
- **AC-02** Die Zahl der Glutpunkte im selben Ausschnitt steigt streng mit dem Wert (1 < 2 < 3), der Schleier wird mit dem Wert deckender (Unit-Test).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

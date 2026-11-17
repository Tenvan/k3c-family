# B-1044 · Der Fluss spiegelt die Welt über der Gehlinie

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** BB2
- **Projekt:** PAP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` § 4: In K2C spiegelt der Fluss die ganze Welt über der Gehlinie, dunkler und getönt, mit hellen Glanzstrichen; Lichtquellen bleiben hell. Bei uns zeigt der Fluss Wellen-Reihen oder einen Shader mit Glanzstrichen (`native/k3c-game/world/river.go`, `water/river.kage`), aber keine Spiegelung.

## Ziel

Der Fluss verdoppelt die Welt als Spiegel; Feuer, Mond und Portale leuchten im Wasser.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (CLI, Rust-Client), Fachprüfung `k3c-hud`. Abnahme: 🧑.

## Anforderungen

- Die gezeichnete Welt über der Gehlinie wird an der Ufer-Unterkante nach unten geklappt in den Fluss gezeichnet, Helligkeit 60–70 %, leicht in Wasserfarbe getönt.
- Waagerechte Verzerrung und wandernde helle Glanzstriche als einziges Muster.
- Lichtquellen und Mond hell in der Spiegelung.
- Frame-Budget: 60 Hz mit zwei Ansichten auf dem Mini-PC; schlechtester Frame messen.

## Nicht-Ziele

Spiegelung von HUD und Text; Wetter auf dem Wasser.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/**`
- **Abhängig von:** B-1043
- **Braucht 🧑:** Freigabe, Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Nacht am Hub: Lagerfeuer und Mond spiegeln sich hell, die Burg dunkel und wellig.

## Ausnahme- und Fehlerfälle

Lava-Stufen: Fluss bleibt, Spiegel glüht mit; ohne Shader-Unterstützung bleibt die alte Wellen-Darstellung.

## Akzeptanzkriterien

- **AC-01** Test: Ein Objekt über der Gehlinie erscheint gespiegelt im Fluss (Lage und Helligkeit im Bereich).
- **AC-02** Messung: schlechtester Frame mit zwei Ansichten unter 16,7 ms auf dem Mini-PC (Wert im Ergebnis).
- **AC-03** Am TV (🧑): Stimmung wie K2C, keine Ablenkung.

## Offene Fragen

keine

## Notizen

Gedächtnis `reference_k2c-bildauswertung` (alle Bilder zeigen den Spiegel).

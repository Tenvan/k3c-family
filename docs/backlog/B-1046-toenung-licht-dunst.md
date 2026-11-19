# B-1046 · Tageszeit und Ereignis tönen das ganze Bild, Lichtquellen haben einen Lichthof

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU55
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` §§ 3, 6: In K2C tönen Tageszeit und Ereignis das ganze Bild (Abend lila, Nacht dunkelblau mit Mondhof, Blutmond rot, Vulkan orange, Winter grau-blau mit Nebel); Lichtquellen haben einen warmen Hof, hintere Ebenen verblassen zur Himmelsfarbe. Bei uns gibt es nur eine Nacht-Darstellung, keine einheitliche Tönung, keine Lichthöfe, keinen Dunst.

## Ziel

Stimmung entsteht aus Licht: jede Tageszeit und jedes Ereignis hat ihre Farbe, Feuer und Fackeln sind nachts die Orientierung.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (CLI, Rust-Client), Fachprüfung `k3c-grafik`. Abnahme: 🧑.

## Anforderungen

- Eine Tönung je Tageszeit und Ereignis (Tag, Abend, Nacht, Morgengrauen, Vollmond, Blutmond, Wetter) aus `data/look.json` (Abschnitt `tint`), auf Himmel, Ebenen, Figuren und Spiegelung gleich. Umsetzung über die vorhandene Lichtkarte (Multiply, B-1056, `world/lightdraw.rs`): Die Karte bekommt eine Grundfarbe statt nur Helligkeit.
- Lichthof (weich, additiv) um Lagerfeuer, Fackeln, Laternen, Kohlebecken, Kristalle; Fackel des Monarchen inklusive; Mond mit weitem Hof, in der Spiegelung hell (B-1044).
- Dunst zwischen den Parallax-Ebenen in Himmelsfarbe (Luftperspektive): je weiter hinten, desto heller und blasser; Werte je Ebene in `data/look.json` (`haze`).
- Assets bleiben neutral; nichts davon wird ins Asset gebacken.
- Aufteilung in zwei Sessions: RU55.4 Tönung und Dunst, RU55.5 Lichthöfe.

## Nicht-Ziele

Neue Wetter-Mechanik (SI3), Partikel wie Schnee (eigenes Ticket bei Bedarf).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/world/light.rs`, `rust/crates/k3c-game/src/world/lightdraw.rs`, `rust/crates/k3c-game/src/world/backdrop.rs`, `rust/crates/k3c-game/src/world/layers.rs`, `rust/crates/k3c-game/src/fx/paint.rs`, `data/look.json`, `data/light.json`
- **Abhängig von:** B-1043
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –
- Frame-Budget prüfen (schlechtester Frame, zwei Ansichten) im Abnahme-Ticket; RU42-Leitplanken, Allokations-Obergrenze bleibt grün. `data/light.json` (Go liest sie ebenfalls) nur lesend erweitern: neue Felder mit Standard, Go-Golden unverändert.

## Beispiele

Blutmond: alles rot getönt, Mond groß mit rotem Hof, Fackeln bleiben warm.

## Ausnahme- und Fehlerfälle

Höhle und Mine: keine Himmels-Tönung, nur Lichthöfe.

## Akzeptanzkriterien

- **AC-01** Test: Tönung je Phase aus `data/` wird auf alle Ebenen gleich angewandt.
- **AC-02** Lichthöfe an allen Lichtquellen, Messung schlechtester Frame im Ergebnis.
- **AC-03** Am TV (🧑): Nacht und Blutmond wirken wie K2C.

## Offene Fragen

keine

## Notizen

Gedächtnis `reference_k2c-bildauswertung` (Bilder 1, 3, 4, 9, 10, 15).

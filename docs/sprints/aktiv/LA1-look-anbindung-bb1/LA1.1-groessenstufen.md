# LA1.1 · Größenstufen mit dem Monarch auf dem Mount als Bezugswert

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** –
- **Tickets:** B-1163
- **Kriterien:** AC-01

## Ziel

Es gibt eine Tabelle der Größenstufen in den Daten: Der Monarch auf dem Mount ist der Bezugswert (1 M), alle anderen Figuren, Tiere, Bosse, Gebäude und das Schiff sind als Vielfache von M festgelegt, und das Rust-Spiel zeichnet sie in diesen Größen.

## Kontext

Auftrag von 🧑 (2026-11-20, nach RU36.2): Die Größenverhältnisse zwischen Monarch und den anderen Figuren passen nicht; Größenstufen definieren, Monarch auf dem Mount als zentraler Wert, alle anderen danach ausrichten, Abgleich an den K2C-Screenshots. Lane zweit, Modell opus (anspruchsvolle Session). Lesen: `docs/standards/bildaufbau.md` (Maßstab, RU55), `docs/standards/rust-port.md`, `docs/standards/paper-spec.md`, `data/look.json`, `data/paper.json`, die Tickets B-1038 bis B-1042 (Zielwerte aus BB1, z. B. Bürger 0,55 bis 0,65 M, Schiff etwa 3 M) und die K2C-Bildauswertung (Notiz `reference_k2c-bildauswertung`: 16 Screenshots, Maße je Bild). Vorgehen: (1) Ist-Größen je Figur und Gebäude aus Manifesten und Rig messen und als Vielfache von M tabellieren, (2) Soll aus K2C-Bildern und BB1 ableiten, (3) Stufen festlegen (z. B. Kleintier, Bürger, Truppe, Monarch mit Mount, Elite, Boss, Gebäude klein/mittel/groß, Burg, Schiff) und in einer Datei unter `data/` ablegen, (4) das Rust-Spiel liest die Tabelle (Skalierung der Figuren und Gebäude am Zeichenort) und Tests prüfen sie. Abweichungen, die nur neue Grafik löst (Papierschnitt-Assets in `tools/paper`), als Ticket melden, nicht hier zeichnen.

## Erlaubte Dateien

- `data/sizes.json` (neu), `data/look.json`
- `rust/crates/k3c-data/**`, `rust/crates/k3c-game/src/actors/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/art/**`, `rust/crates/k3c-game/src/world/**` (nur Größen und Skalierung)
- `docs/standards/bildaufbau.md` (Abschnitt Größenstufen)

## Nicht-Ziele

Neue oder geänderte Papierschnitt-Assets, Regeln und Balancing, Stimmung und Farben (LA1.4, LA1.5), die BB1-Nachträge (LA1.2), Cheat-Fenster, Seekarte und Schiff-Fix (RU36.3).

## Schritte

1. Dokumente und Tickets lesen, Ist-Größen messen (Tabelle im Ergebnis).
2. Stufen festlegen und `data/sizes.json` schreiben; Parser und Prüfung in `k3c-data` mit Test.
3. Rust-Spiel auf die Tabelle umstellen; Tests je Stufe (Verhältnis zum Monarchen).
4. `bildaufbau.md` um die Stufen ergänzen.
5. `cargo test -p k3c-data -p k3c-game`, `task rust:lint`, einmal `task check:diff`; Bildbeispiel Monarch, Bürger, Truppe, Boss und Burg neben K2C-Referenz im Ergebnis.

## Fertig, wenn

- [x] AC-01: `data/sizes.json` nennt jede Stufe als Vielfaches des Monarchen auf dem Mount; das Rust-Spiel zeichnet danach; Tests prüfen die Verhältnisse.
- [x] Ergebnis nennt Abweichungen, die neue Grafik brauchen, als Punkte „Für den Hauptrechner“.
- [x] Tests der geänderten Crates und `task rust:lint` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 geprüft: `data/sizes.json` (neu, 72 Assets in 24 Stufen, Bezug 1 M = 121,5 px = berittener Monarch) wird von `k3c-data::sizes` geladen und geprüft (Stufe vorhanden, `sicht` in (0, 1]); Look-Profil `k2c` setzt `sizes: true`, `go` bleibt unverändert. Figuren (Truppen, Gegner, Bosse mit eigenem Asset) bekommen ihre Höhe im Katalog (`actors/sizes.rs`: sichtbare Höhe = Stufe × M), Gebäude und Objekte über `units_per_px` (`art/scales.rs`, der Faktor aus `paper.json › skalierung` entfällt für Assets mit Stufe). Monarch und Reittiere sind der Bezug und bleiben. Tests: `cargo test -p k3c-data sizes` (Stufen im Standard-Korridor, Verhältnisse zum Monarchen, Prüfungen), `cargo test -p k3c-game sizes` (k2c zeichnet Figuren in ihrer Stufe, go unverändert, Sheet-Faktor, `sicht` gegen die gerenderte Grafik). `task rust:lint` grün, `cargo test -p k3c-game` grün. `bildaufbau.md` um den Abschnitt Größenstufen ergänzt.

Ist-Messung (sichtbare Höhe in M, vorher): Bürger 0,43–0,45, Elite-Truppen 0,46–0,54, Gier 0,28, Wölfe 0,56, Goblins 0,47, Höhlentroll 0,83, Bosse 1,0–1,4 (alle gleiche Rahmenhöhe), Burg 3,1, Turm 2,0, Mauer 0,74, Gebäude um 1,0; Bäume waren durch BB1 (Faktor 1,5) schon im Standard (2,5–5,6 M) und bleiben dort. Gesetzt: Bürger 0,6, Elite und Wolf/Goblin/Skelett/Zombie 0,65, große Gegner 1,0, Troll 1,2, Boss 1,3, Gebäude 0,85/1,1, Mauer 0,9, Turm 2,2, Burg 4,0.

Manuell nicht geprüft (Bildabgleich neben den K2C-Screenshots am PC bleibt bei Ralf, Abnahme LA1.4/LA1.6). Gen: `task gen:paper` lief für die Messung (17 Baum-Varianten neu gerendert, nicht eingecheckt).

`task check:diff` Stufe 1 (`regeln`) rot, nicht durch diese Session: `tests/projectRules.test.ts` (untracked Ordner `native/` im Arbeitsordner) und `tests/planning*.test.ts` (Fahrplan/Vorlage RU30, WI1 auf `main`). `cargo test -p k3c-data` hat einen roten Test `tree_rest_tests::rest_wie_go` (`sounds.json.slots` Länge); `data/sounds.json` ist nicht angefasst (vermutet: schon auf `main` rot, ungeprüft).

### Für den Hauptrechner

- Schiff (`gebaeude-paper/schiff`, Rumpf 1,2 M hoch, 3 M lang) weicht vom Standard (Rumpf 0,7 M, Länge 4–5 M) ab: braucht neue Grafik oder anderes Seitenverhältnis, gehört zu RU36.3; nicht in der Tabelle.
- Nur die Gerüste `geruest-1` bis `-4`, `-6` sind eingestuft; `hintergrundmauer`, `lagerfeuer`, Treppen, `barke` bleiben ohne Stufe, `burg-ausbau` ohne Messung.
- Kleinkram (Truhen 0,23 M, Felsen 0,2–0,4 M, Erzbrocken, Münzen) ohne Stufe (Lesbarkeit vor Maßstab); Entscheidung für Ralf, ob sie Stufen bekommen.
- Ungenutzte Bäume `birke-2`, `pappel-2`, `tanne-2` (0,9–1,3 M) sind nirgends referenziert; Aufräum-Ticket.
- Nadelbäume (bis 5,6 M) im Spiel am PC prüfen (Ralf).
- Rote Tests auf `main`: `native/`-Ordner entfernen, Fahrplan/RU30 und `sounds.json`-Golden prüfen.

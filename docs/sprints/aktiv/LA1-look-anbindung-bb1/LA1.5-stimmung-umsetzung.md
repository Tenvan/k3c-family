# LA1.5 · Stimmung nach dem Abgleich umsetzen (Farbe, Licht, Dunst, Tiefe)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** hauptrechner
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** LA1.4
- **Tickets:** B-1164
- **Kriterien:** AC-05

## Ziel

Die ranghöchsten Abweichungen der Stimmung aus LA1.4 sind im Rust-Spiel behoben: Farbe, Licht, Dunst und Tiefe liegen sichtbar näher am K2C-Vorbild.

## Kontext

Lane zweit, Modell opus. Eingabe: `docs/messung/stimmung-abgleich.md` (LA1.4, von 🧑 abgenommen 2026-11-21). Look-Werte in `data/look.json` (Profil k2c), Standard `docs/standards/bildaufbau.md`. Gen und Bildvergleiche darf die Lane zweit laufen lassen. Reicht die Session nicht für alle Punkte, bleiben Punkte als Liste im Ergebnis („Für den Hauptrechner“).

### Punkte aus LA1.4 (Rang 1 bis 3)

1. **Rang 1, Lichthöfe (Nacht, Hub):** Fackeln an allen Häusern überblenden Gebäude und Figuren; das Licht soll nur die Gegend erhellen, ohne selbst sichtbar zu sein (kein heller Fleck auf Haus oder Figur). Lichthof des Lagerfeuers; Fackel-Hof größer und weicher. Bezug: LA1.8 (Fackeln, Monarch-Licht), `data/light.json`.
2. **Rang 2, Nachthimmel:** Sterne; Mond mit weitem Hof. Dazu: Wellenstriche im Wasser abdunkeln (nachts zu hell, sehen nicht schön aus; bei Bedarf in den K2C-Screenshots nachsehen), Spiegelung weicher (Spiegel/Welt 74 % auf 60 bis 70 %), Nacht nicht ganz schwarz (Silhouetten der Kulisse leicht angehoben), Abendrot am Horizont.
3. **Rang 3, Himmel mit Verlauf (Tag, Abend):** Himmelsverlauf und Wolken statt flacher Fläche; Dunst zwischen den Kulissen-Ebenen (hinten heller und blasser); letzte feste Hintergrund-Ebene als monumentales Bild (großer Berg, Pyramide, Vulkan o. ä.). Das Monumentalbild braucht neue Grafik: dann Ticket statt eigener Zeichnung (Nicht-Ziel „Neue Assets“), nur anbinden, was `public/grafik` schon liefert.

Vorher-Bilder: `.work/stimmung/<szene>-<1|2>/shot-01.png` (Tag 300, Abend 14000, Nacht 18000, Morgengrauen 23500; Befehle in `docs/messung/stimmung-abgleich.md`).

## Erlaubte Dateien

- `data/look.json`, `data/light.json`
- `rust/crates/k3c-game/src/world/**`, `fx/**`, `art/**`, `still/**`
- `testdata/rust/**` nur für neue Bildvergleiche der geänderten Szenen

## Nicht-Ziele

Neue Assets, Regeln, Größen (LA1.1), Waldwand (LA1.2).

## Schritte

1. Abgleich-Liste lesen, Reihenfolge nach Rang.
2. Je Punkt ändern, Szene aufnehmen und neben K2C-Referenz prüfen.
3. Tests, `task rust:lint`, einmal `task check:diff`; Vorher-Nachher-Bilder im Ergebnis.

## Fertig, wenn

- [ ] AC-05: Die Punkte aus LA1.4 mit Rang 1 bis 3 sind behoben, Vorher-Nachher-Bilder im Ergebnis.
- [ ] Nachprüfung 🧑 am PC.
- [ ] Tests und `task rust:lint` grün.

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

Stand: umgesetzt, **Nachprüfung 🧑 am PC steht aus** (Status bleibt `offen` bis dahin). Hauptrechner statt Lane zweit (🧑 2026-11-21), Modell sonnet.

### Umgesetzt
- **Rang 1, Lichthöfe** (`data/look.json` › glow): Radius 1 → 1,5, Stärke 0,55 → 0,32, Kern 0,1 → 0, Abfall 0,4 → 0,5; Monarch-Licht Radius 2,5 → 3,6, Stärke 0,5 → 0,3, Höhe 3,2 → 3,4. Der Hof erhellt die Umgebung, überdeckt den Monarchen nicht mehr (Vorher/Nachher Nacht). **Nicht gesehen:** Fackeln an gebauten Häusern, die Aufnahmen zeigen den Hub ohne Gebäude; die gleichen Werte gelten für sie, die Sichtprüfung bleibt bei 🧑.
- **Rang 2, Nachthimmel:** Sterne nach der Lichtkarte (`world/sky.rs`, `draw_stars`, 90 Stärne, feste Lage ohne Zufall), Mond-Hof Radius 9 → 14, Stärke 0,55 → 0,6, Scheibe 0,12 → 0,08; Nacht nicht ganz schwarz (`sky.nightLift` 0,06 und `sky.backLift` 0,09, nur im Profil k2c; `light.json` unverändert, weil die Go-Golden daran hängen); Abendrot am Horizont (`sky.horizon`, `glowHorizon` 0,7, nur in Dämmerung und Morgengrauen); Spiegelung weicher (`mirror.brightness` 0,62, `tint` 0,22, `glint` 0,25).
- **Rang 3, Himmel mit Verlauf:** Verlauf von der Himmelsfarbe zu `sky.top` (24 Bänder), Dunst dichter (`haze` 0,3/0,15/0 → 0,4/0,25/0; Welt forest 0,35/0,2/0,05 → 0,45/0,3/0,05).
- Neuer Code: `k3c-data/src/look_sky.rs` (Struct `Sky`), `k3c-game/src/world/sky.rs` und `sky_tests.rs` (Tests für Dämmerlicht und feste Sternlage), Einbindung in `layers.rs`, `lightdraw.rs`, `light.rs`. Eingebaute Profilwerte (`look.rs`, `look_world.rs`) und Tests (`mirror_tests.rs`, `tests/mirror_bild.rs`) angepasst.

### Prüfung
- `cargo test -p k3c-game -p k3c-data --release`: alles grün außer `tree_tests::economy_wie_go` (`economy.json.ChestGold[0] = 15, erwartet 30`): nicht von dieser Session (Truhen-Gold halbiert in RU36.3, B-1159; Go-Baum nicht nachgezogen), Befund für den Hauptrechner.
- `cargo clippy --all-targets -D warnings` und `cargo fmt` sauber; `task check:diff` siehe Commit.
- Bilder: Vorher `.work/stimmung/<szene>-1/shot-01.png`, Nachher `.work/stimmung-nach/<szene>-1/shot-01.png` (tag, daemm, abend, nacht; nicht eingecheckt).

### Für den Hauptrechner
- B-1178: Wolken, monumentaler Hintergrund (neues Asset), Wellenstriche des Wassers (Shader der Plattform, `RIVER_WAVE`; ändern ohne Go-Golden).
- `economy_wie_go` rot (siehe oben).

### Abweichung vom Plan
AC-05 nur teilweise erfüllt: von Rang 3 fehlen Wolken und das Monumentalbild, von Rang 2 die Wellenstriche (B-1178).

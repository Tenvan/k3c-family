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

Stand: umgesetzt, **zweite Nachprüfung 🧑 am PC steht aus** (Status bleibt `offen`). Hauptrechner statt Lane zweit (🧑 2026-11-21), Modell sonnet.

### Umgesetzt (Runde 1)
- **Rang 1, Lichthöfe** (`data/look.json` › glow): weicher, größer, ohne Kern. **Rang 2:** Sterne nach der Lichtkarte (`world/sky.rs`), Mond-Hof größer, Nacht-Aufhellung (`sky.nightLift`, `backLift`, nur Profil k2c; `light.json` bleibt wegen der Go-Golden), Abendrot (`sky.horizon`), Spiegelung weicher (`mirror.brightness` 0,62, `glint` 0,25). **Rang 3:** Himmelsverlauf (`sky.top`, `gradient`), Dunst dichter.
- Neuer Code: `k3c-data/src/look_sky.rs`, `look_glow.rs` (aus `look.rs` ausgelagert, Dateigrenze), `world/sky.rs` mit Tests.

### Nachprüfung 🧑 2026-11-21 (Runde 2)
- **„überstrahlt noch, das Licht selbst darf gar nicht sichtbar sein“:** neues Feld `glow.sources` (`look_glow.rs`), in den Daten `false`: Lagerfeuer, Fackeln und Monarch zeichnen keinen sichtbaren Hof mehr; sie erhellen nur über die Lichtkarte (`light.json`). Der Mond behält seinen Hof. Test `ohne_sichtbare_quellen_hat_nur_der_mond_einen_hof`; die übrigen Hof-Tests testen mit `sources: true`.
- **„Dunst nur bei Wetter-Events“:** `tint::weather_haze` (Taifun und Flut, Vorboten und Wirkung, über `sight_of`); sonst kein Dunst (`layers.rs`).
- **„blass beim Abdunkeln – Filter oder nur der Dunst?“:** Beides trägt bei. Der Dunst legt in Himmelsfarbe einen Schleier über Kulisse und Mitte, die Lichthöfe einen gelben Schleier über Gebäude. Die Tönung (Multiplikator der Tageszeit) und meine Nacht-Aufhellung bleiben als mögliche Ursache; zum Eingrenzen gibt es `world/fx.rs`: jeder Effekt (Dunst, Höfe, Sterne, Himmel, Aufhellung, Tönung, Spiegelung) lässt sich per `K3C_FX_OFF=haze,tint,…` ausschalten. Aufnahme Morgengrauen vom Spielstand `spiel-1` ohne Dunst und ohne Höfe zeigt klare Farben (`.work/stimmung-nach/spiel-an/`).
- **Cheat-Fenster-Panel:** B-1179 (Schalter `fx::set` stehen bereit). **FPS-Anzeige** (zeigt 0 FPS, zu groß, abgeschnitten): B-1180.

### Prüfung
- `cargo test -p k3c-game -p k3c-data --release`: grün außer `tree_tests::economy_wie_go` (`ChestGold[0] = 15, erwartet 30`, nicht von dieser Session: Halbierung in RU36.3, B-1159). `cargo clippy --all-targets -D warnings` und `cargo fmt` sauber. `task check:diff` scheitert schon vorher an Lint-Fehlern einer Workbench-Oberfläche in `tools/`, nicht an dieser Änderung.
- Bilder: Vorher `.work/stimmung/`, Nachher `.work/stimmung-nach/` (nicht eingecheckt).

### Für den Hauptrechner
- B-1178: Wolken, monumentaler Hintergrund, Wellenstriche. B-1179: Cheat-Panel. B-1180: FPS-Anzeige. `economy_wie_go` rot.

### Abweichung vom Plan
AC-05 nur teilweise erfüllt: Wolken, Monumentalbild und Wellenstriche fehlen (B-1178). Die Fackeln an gebauten Häusern habe ich nachts nicht selbst gesehen (kein Stand mit Gebäuden in der Nacht aufgenommen).

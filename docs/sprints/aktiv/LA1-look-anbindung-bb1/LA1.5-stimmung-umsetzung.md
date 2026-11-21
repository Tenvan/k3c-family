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

Stand: umgesetzt, **weitere Nachprüfung 🧑 am PC steht aus** (Status bleibt `offen`). Hauptrechner statt Lane zweit (🧑 2026-11-21), Modell sonnet.

### Runde 1 (Stimmung)
Lichthöfe weicher, Sterne, Mond-Hof, Nacht-Aufhellung (`sky.nightLift`/`backLift`), Abendrot, Himmelsverlauf, Spiegelung weicher (`data/look.json`, `world/sky.rs`, `look_sky.rs`, `look_glow.rs`).

### Runde 2
`glow.sources` aus (Licht erhellt nur über die Lichtkarte), Dunst nur bei Taifun und Flut (`tint::weather_haze`), Schalter je Umgebungseffekt (`world/fx.rs`, `K3C_FX_OFF`), FPS-Anzeige (B-1180).

### Runde 3 (Fackeln, 🧑 2026-11-21)
- **Fackeln vor den Figuren:** Halter jetzt im Drawer `Part::Torches` auf der Ebene der Figuren (`Layer::ACTORS`, über den Gebäuden, unter den Figuren); die Flammen kommen nach der Lichtkarte (`draw_flames`) und bleiben hell.
- **Fackeln auf den Türen:** `glow.torchSpread` 2 weitet die Versätze aus `light.json` nur in der Darstellung (die Sim-Daten bleiben wegen der Go-Golden); Licht (`for_each_torch`) und Flammen nutzen dieselben Orte.
- **Burg ohne Fackel:** zwei Fackeln neben dem Tor (`glow.castleTorches` 2,6 Units), mit Licht.
- **Fackel-Atmosphäre:** wärmerer Schein (`glow.warmth` 1,6, Exponent auf `glowColor`), Flamme größer (12/32 Units), dreischichtig und flackernd mit der Spielzeit. Profil go unverändert (Spread 1, keine Burg-Fackeln, Warmth 1).
- **Eigene Flammen-Assets je Welt, verschiedene Fackeln, Lampen und Feuerschalen, Ankerpunkte an Häusern:** B-1181 (Größe L, braucht Entscheidungen von 🧑).

### Prüfung
`cargo test -p k3c-game -p k3c-data --release` grün außer `economy_wie_go` (`ChestGold`, aus RU36.3, nicht von dieser Session); `clippy --all-targets -D warnings` und `fmt` sauber. Aufnahme Nacht mit Spielstand `spiel-8` (Hub 4): Fackeln neben den Türen, Burg mit zwei Fackeln und warmem Schein (`.work/torch-neu/shot-01.png`). `task check:diff` scheitert weiter am Lint einer Workbench-Oberfläche in `tools/`.

### Für den Hauptrechner
B-1178 (Wolken, Monumentalbild, Wellenstriche), B-1179 (Cheat-Panel), B-1181 (Licht-Assets), `economy_wie_go` rot.

### Abweichung vom Plan
AC-05 nur teilweise erfüllt (B-1178).

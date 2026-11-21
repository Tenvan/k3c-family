# rust/crates/k3c-game/src/actors/

## Responsibility

Zeichnet die Figuren aus dem Spielzustand: Monarchen auf dem Reittier, Truppen je Beruf, Gegner und Bosse, dazu
Lebensbalken, Markierungen und Kampfzeichen. Figuren mit Teile-Baum laufen als Rig, `requisite`-Figuren als Sheet.
Port von `native/k3c-game/actors/*.go`.

## Design

- `mod.rs`: `Actors` (Drawer, Listener, `AssetSource`) mit `figs: BTreeMap<Key, Figure>`, `Catalog`, `Painter`, `Ink`;
  `new(lib)` lädt den Katalog (Fehler → Log, keine Figuren), `check_rigs` prüft die Rigs mit der Zeichenfläche.
  `lib` ist `Rc<RefCell<Library>>` (Abweichung von Go).
- `catalog.rs`: Zuordnung Rolle → Asset aus den Daten (`Catalog`, `Look`, `BossEntry`, `load_catalog`; `player`,
  `mount_of`, `enemy`, `sheet_assets`). `sizes.rs`: Größenstufen aus `data/sizes.json`. `levels.rs`: Sheets je Tiefe.
- `anim.rs`: reine Funktionen Zustand → Animation und Gangart (`Want`, `Motion`, `figure_want`, `monarch_want`,
  `anim_time`, `sheet_frame`, `mount_gait`).
- `figures.rs` / `figures_pose.rs`: `Figure`, `Key`, Anlegen (`figure`, `ensure_actors`), `apply`, Warnungen;
  `pose.rs`: Pose im Takt fortschreiben (`advance_all`, `MAX_POSE_STEP`).
- `draw.rs`: `ActorsDrawer`, `draw_figure`, Tönung. `marks.rs`, `markdraw.rs`, `combatmarks.rs`: Balken, Beute- und
  Last-Anzeigen, Spielermarker, Krone, Banner, Schild. `festival.rs`: tanzende Bürger und jubelnde Truppen im Hub
  (`festival_want`).

## Flow

Je Frame: `listen` gleicht die Figuren über stabile IDs mit der Welt ab, wählt Pose und Animation und schreibt
Animation und Umhang aller Rig-Figuren fort; `draw` zeichnet nur (Rig über `art::Actor`/`Painter`, sonst Sheet), danach
die Marken. Die Rig-Bilder lädt der Aufrufer vorher (`Library::load_rigs`).

## Integration

- Abhängigkeiten: `art` (Library, Painter, Actor), `view` (`Frame`, `Layer`, `Drawer`, `Listener`, `roles::AssetSource`), `k3c-sim` (`state::World`), `k3c-data` (`Source`), `k3c-platform` (`Canvas`, Log).
- Konsumenten: `game` (Paketbildung), `props`, `replay`, `still`.

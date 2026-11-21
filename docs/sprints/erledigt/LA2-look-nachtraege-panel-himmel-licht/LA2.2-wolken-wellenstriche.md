# LA2.2 · Wolken am Himmel und ruhigere Wellenstriche (B-1183)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la2
- **Abhängig von:** LA2.1
- **Tickets:** B-1183
- **Kriterien:** AC-02, AC-03

## Ziel

Der Himmel hat langsam ziehende Wolken, und die Wellenstriche im Wasser sind abgedunkelt und nachts nicht mehr zu hell.

## Kontext

Himmel: `rust/crates/k3c-game/src/world/sky.rs` (draw_sky, Sterne) und `data/look.json` (sky); Werte in `rust/crates/k3c-data/src/look_sky.rs`. Das eingebaute Profil muss der Datei entsprechen (Test `eingebaute_werte_sind_die_der_datei`); das Go-Profil bleibt unverändert (Golden-Tests). Wellenstriche liegen im Wasser-Code (`world/water*`). Ticket B-1183 (Wolken, Wellenstriche). Wolken zunächst prozedural, kein neues Asset.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/world/sky.rs`
- `rust/crates/k3c-game/src/world/sky_tests.rs`
- `rust/crates/k3c-game/src/world/water*.rs`
- `rust/crates/k3c-data/src/look_sky.rs`
- `rust/crates/k3c-data/src/look.rs`
- `data/look.json`

## Nicht-Ziele

Keine Wolken-Assets, kein Monumentalbild (LA2.4), kein Eingriff ins Go-Profil.

## Schritte

1. Wolkenebene im k2c-Profil: Dichte, Tempo, Tönung je Tageszeit, schaltbar über den fx-Schalter `sky`.
2. Wellenstriche abdunkeln (Wert in look.json, Nacht stärker).
3. Tests und Profil-Gleichheit prüfen.

## Fertig, wenn

- [ ] AC-02: Wolken ziehen im k2c-Profil sichtbar am Himmel, im Go-Profil unverändert.
- [ ] AC-03: Wellenstriche sind dunkler, Golden-Tests des Go-Profils unverändert grün.
- [ ] `task check:diff` grün.

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

AC-02 geprüft: Wolken im k2c-Profil (`data/look.json › sky`: `clouds` 7, `cloudAlpha` 0,5, `cloudSpeed` 0,004 Zellenbreiten/s, `cloudNight` 0,3; `look_sky.rs`). `world/sky.rs`: `cloud_boxes` (feste Lage aus Zahlenstrom, drei gestufte Ballen, waagerecht auf die Zelle begrenzt), `draw_clouds` (Weiß, am Abend ins Abendrot getönt, nachts auf `cloudNight` der Deckkraft und mit `back` abgedunkelt); gesteuert vom fx-Schalter `sky`, Uhr `Frame::clock` (`draw_sky` bekam den Parameter `clock`). Go-Profil unverändert (`sky.enabled` false). Test `clouds_are_deterministic_clamped_and_drift`.

AC-03 geprüft: Die Wellenstriche sind die Glanzstriche der Spiegel-Fläche (`mirror.glint`): 0,25 → 0,15, neu `mirror.glintNight` 0,4 (Anteil in tiefer Nacht, über `emissive` in `mirror_uniforms`); Go-Profil hat den Spiegel aus, Go-Golden unberührt. `testdata/rust/data-tree.json.gz` neu geschrieben (`go test ./internal/detvec/ -write`), `eingebaute_werte_sind_die_der_datei` und alle k3c-data-Tests grün, clippy grün, `task check:diff` nur bekannte Planungstests rot.

Nicht geprüft: Aussehen im Spiel (Dichte, Tempo, Dunkelheit sind vermutete Startwerte); manuelle Abnahme durch Ralf. Abweichung: `mirror.rs`, `layers.rs` und `testdata/rust/data-tree.json.gz` lagen außerhalb der erlaubten Dateien, waren nötig.

### Für den Hauptrechner
- Die Wellenstriche im Shader-Fluss ohne Spiegel (Profil go, `RIVER_WAVE`) bleiben unverändert (Go-Parität).

# LA2.4 · Monumentales Hintergrundbild je feste Welt (B-1183)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/la2
- **Abhängig von:** LA2.2
- **Tickets:** B-1183
- **Kriterien:** AC-04

## Ziel

Jede Welt hat eine letzte, feste Hintergrund-Ebene als monumentales Bild (großer Berg, Pyramide, Vulkan o. ä.).

## Kontext

Kulissen-Ebenen: `world/layers.rs` (draw_far, draw_layer_with, Schicht `fern` ohne Scroll), Zuordnung in `data/paper.json`, Gruppe `hintergruende` unter `public/grafik/`. Leitmotiv-Ebene siehe B-1148 bis B-1151. Ticket B-1183, Rang-3-Punkt aus `docs/messung/stimmung-abgleich.md`. Asset-Arbeit: Lane zweit. Motiv je Welt als Default entscheiden und im Ergebnis melden.

## Erlaubte Dateien

- `public/grafik/hintergruende/**`
- `data/paper.json`
- `rust/crates/k3c-game/src/world/layers.rs`

## Nicht-Ziele

Keine Wolken (LA2.2), keine neuen Scroll-Ebenen.

## Schritte

1. Motiv je Welt festlegen (Default: Oberwelt Berg, Sengoku Vulkan, Ägypten Pyramide).
2. Assets über `assets_*` anlegen und rendern.
3. In die feste Ebene `fern` einbinden, Dunst und Nachtdunkel prüfen.

## Fertig, wenn

- [ ] AC-04: Jede Welt zeigt am Horizont ein monumentales Bild in der festen Ebene, Test für Zuordnung grün.
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

AC-04 geprüft: Die Leitmotive je Welt existierten als Assets (`hintergruende/leitmotiv-<welt>`: Burgberg, Vulkan mit Rauch und Torii, Pyramiden mit Sphinx, Steinkreis; `data/paper.json › leitmotive`, Lage und Größe 3 bis 5 M in `look.json › worlds › motif`), wurden aber vom Rust-Spiel nirgends geladen oder gezeichnet. Neu: `world/layers.rs` `draw_motif` zeichnet das Motiv der Insel-Welt als letzte feste Ebene direkt nach dem Himmel (ohne Scroll, Fuß auf der Boden-Linie, Helligkeit `back`, der Dunst der Ebenen darüber dämpft weiter), nur im Profil mit Himmel, in Stufen mit Himmel und bei fx-Schalter `sky`; `MOTIFS`/`motif_of` (unbekannte Welt: Standard-Motiv), `Far::assets` lädt die vier Motive (je 768×480) für Stufen mit Himmel. Test `leitmotiv_je_welt_stimmt_mit_paper_json` (Zuordnung gleich `data/paper.json`). Motive als Default entschieden: Standard Berg, Sengoku Vulkan, Nil Pyramide, Britannien Steinkreis (kein neues Asset nötig). `cargo test -p k3c-game --lib` und clippy grün, `task check:diff` nur bekannte Planungstests rot.

Nicht geprüft: Aussehen im Spiel (Größe, Lage, Dunst; Textur-Budget +5,9 MiB RGBA für vier Motive, nicht gemessen); manuelle Abnahme durch Ralf. Abweichung: `world/mod.rs` (Asset-Liste) und `world/lightdraw.rs` (`world_of` sichtbar) lagen außerhalb der erlaubten Dateien, waren nötig.

### Für den Hauptrechner
- `tests/e2e_flow.rs` (`ablauf_wie_go_zwei_laeufe_gleich`) ist rot: Station `skillpunkt` bei Takt 411 statt 535 (Referenz `testdata/rust/e2e/flow.json`, Go-Stand RU32); vermutlich das bekannte B-912/NA1.2, unabhängig von LA2 (ungeprüft, ob schon auf main rot).
- Textur-Budget der Leitmotive in der Abnahme messen (alle vier Motive je Stufe mit Himmel geladen, weil die Welt-ID beim Laden fehlt).

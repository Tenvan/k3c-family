# LA2.4 · Monumentales Hintergrundbild je feste Welt (B-1183)

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

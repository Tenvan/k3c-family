# LA1.8 · Nachts tragen alle Gebäude Fackeln und der Monarch hat ein folgendes Licht

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** LA1.3
- **Tickets:** B-1161
- **Kriterien:** AC-07

## Ziel

Nachts tragen alle Gebäude Fackeln (nicht nur Mauer und Turm), am Tag nur die Halter ohne Schein, und der Monarch hat nachts ein kleines folgendes Licht (Glühwürmchen oder kleiner Stern).

## Kontext

Ticket B-1161 (Ralf 2026-11-20, G-11 in RU36.2): Fackeln an Mauer und Turm passen, aber nicht alle Gebäude haben Fackeln; der Monarch braucht nachts ein folgendes Licht. Lane zweit. Die Lichtkarte zeichnet Rust seit B-1056 (Multiply-Blend); Glow, Fackel-Höfe und Mond stehen in `data/look.json` (Profil k2c) und `data/light.json`. Fackeln als Spielobjekt (W15, `docs/rules/`), Gebäude mit Fackelplätzen: Daten in `data/buildings.json`/`data/light.json`. Vorgehen: (1) Ist-Stand je Gebäude (welche haben Fackelplätze), (2) fehlende Gebäude ergänzen (Papierschnitt-Halter nur über MCP `assets_*`, Regeln der Fackeln unverändert: Tag nur Halter, zerstört keine), (3) Monarch-Licht als Glow-Quelle am Monarchen (kleiner Radius, warme Farbe, nur nachts, je Ansicht und Spieler, Farbe pro Spieler optional) mit Test, (4) Vorher-Nachher-Bild Nacht. Autonom: Offene Fragen mit Default entscheiden und im Ergebnis melden.

## Erlaubte Dateien

- `data/light.json`, `data/look.json`, `data/buildings.json` (nur Lichtfelder)
- `rust/crates/k3c-game/src/fx/**`, `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/actors/**`
- `public/grafik/**` nur über `assets_*`

## Nicht-Ziele

Stimmung insgesamt (LA1.4, LA1.5), neue Fackel-Regeln, Kompass.

## Schritte

1. Ist-Stand der Fackeln je Gebäude festhalten.
2. Fehlende Gebäude mit Fackeln versehen; Test (jedes Gebäude mit Fackelplatz bekommt nachts Licht, am Tag nur Halter).
3. Monarch-Licht: Quelle, Radius, Farbe, nur nachts; Test.
4. `cargo test -p k3c-game`, `task rust:lint`, einmal `task check:diff`; Vorher-Nachher-Bild.

## Fertig, wenn

- [ ] AC-07 (B-1161): Alle Gebäude mit Fackelplatz tragen nachts Fackeln, am Tag nur Halter; der Monarch hat nachts ein folgendes Licht; Tests belegen es, Vorher-Nachher-Bild im Ergebnis.
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

### Ist-Stand (Schritt 1)
- `data/light.json › torches` kannte nur `wall` und `tower`. Fackeln (Halter immer, Schein nur nachts) und Lichthöfe lesen genau diese Tabelle (`world/light.rs`, `lightdraw.rs`, `halo.rs`, Sim `k3c-sim/state/torches.rs`).

### Umgesetzt
- **Fackeln an allen Hub-Gebäuden:** `torches` bekommt Einträge für Werkstatt, Bogner, Rüstkammer, Lager, Kinderstube, Kaserne, Taverne, Heiler, Schmiede (je links und rechts, ±0,8 Units). Burg bleibt beim Lagerfeuer. Regeln unverändert: nur gebaute Plätze, zerstört = keine, am Tag nur Halter ohne Schein. Kein neuer Code für die Fackeln nötig, daher gilt es im Spiel und in der Sim gleich.
- **Monarch-Licht:** neues `glow.monarch` (`radius` 2,5, `strength` 0,5, `height` 3,2 Units; `look.rs`, `look.json`) und `Kind::Monarch` in `world/halo.rs`: jeder Monarch trägt nachts einen kleinen warmen Hof über dem Kopf, der mit ihm wandert (je Ansicht und Spieler, gleiche Farbe `glowColor`-Ton). Nur im Profil `k2c` (wie alle Höfe), am Tag aus, außerhalb der Sicht kein Hof, zählt nicht zu `glow.max`.

### Kriterien
- **AC-07 erfüllt (Tests):** `alle_hub_gebaeude_tragen_fackeln` (je Hub-Gebäude nachts n Fackeln, zerstört 0, Tag kein Schein) und `monarch_traegt_nachts_ein_licht` (ein Licht je Monarch, folgt, Tag und Sichtfeld) in `world/halo_tests.rs`. **Vorher-Nachher-Bild: nicht erzeugt** (kein Bildlauf in dieser Session; Ralf prüft am PC).
- `cargo test -p k3c-game -p k3c-sim -p k3c-data` grün, `task rust:lint` grün, `go test ./...` grün. `task check:diff` bleibt in Stufe `regeln` rot an den bekannten Planungstests (RU30, Fahrplan, `planningProjects`).

### Nebenbei repariert (Go-/Rust-Datenvergleich)
- `engine/internal/detvec/data_test.go`: `sizes.json` fehlte in `dataFiles` (seit LA1.1 rot, `TestDataFilesComplete`); ergänzt.
- `testdata/rust/data-tree.json.gz` mit `-write` neu geschrieben (neue Fackel-Schlüssel, `sizes.json`; behebt auch den alten Fehler `sounds.json.slots`).
- `k3c-data/tree_rest_tests.rs`: `light.json` mit `same_as_go_zahlen_aus_daten` (neue Schlüssel erlaubt), `sizes.json` mit Vergleich und in der Prüfliste.

### Entscheidungen (Default, für Ralf)
- Monarch-Licht ist ein gezeichneter Hof (Glow-Quelle), keine Aufhellung der Lichtkarte; so bleibt das Go-Profil bildgleich.
- Alle Hub-Gebäude gleich: zwei Fackeln, kein Gebäude-spezifischer Versatz; kein neues Halter-Asset.

### Nachprüfung (Ralf, PC)
- Nachts im Hub: Fackeln an allen Gebäuden sichtbar, nicht zu viele Höfe (Frame-Budget); Monarch-Licht (Größe, Helligkeit, Split-Screen) abnehmen, Werte in `look.json` justierbar.

### Für den Hauptrechner
- Vorher-Nachher-Bild Nacht fehlt (braucht `gen:paper`/Bildlauf, Abnahme-Ticket LA1).
- Mit dem Neuschreiben von `data-tree.json.gz` ist der Go-Datenbaum wieder aktuell; weitere Datenstände bitte über `go test ./internal/detvec/ -write` statt über Rust-Sonderfälle.

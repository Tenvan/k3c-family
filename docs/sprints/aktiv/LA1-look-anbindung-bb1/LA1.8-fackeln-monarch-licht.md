# LA1.8 · Nachts tragen alle Gebäude Fackeln und der Monarch hat ein folgendes Licht

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

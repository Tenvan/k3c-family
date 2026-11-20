# LA1.1 · Größenstufen mit dem Monarch auf dem Mount als Bezugswert

- **Status:** offen
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

- [ ] AC-01: `data/sizes.json` nennt jede Stufe als Vielfaches des Monarchen auf dem Mount; das Rust-Spiel zeichnet danach; Tests prüfen die Verhältnisse.
- [ ] Ergebnis nennt Abweichungen, die neue Grafik brauchen, als Punkte „Für den Hauptrechner“.
- [ ] Tests der geänderten Crates und `task rust:lint` grün.

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

# LA1.7 · Mauern: durchgängig, Burgbereich geschlossen, Stufe 1 als Zaun, Platzhalter weg

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** LA1.2
- **Tickets:** B-947, B-1160, B-1175, B-1176
- **Kriterien:** AC-06

## Ziel

Die Mauern im Hub sind durchgängig ohne Lücken, die Mauer schließt im Burgbereich die Stelle zwischen den inneren Mauerstücken, die Mauer der Stufe 1 sieht aus wie ein Zaun oder eine Bretterwand, und die einfarbigen Platzhalter-Rechtecke im Hub sind weg.

## Kontext

Lane zweit, nach LA1.2 (gleicher Bereich `art/`, `world/`, `props/`). Ralf (2026-11-20, Screenshots im Rust-Spiel): (1) Der Mauerzug ist nicht durchgängig (B-947); (2) zwischen den inneren Mauerstücken, wo die Burg steht, fehlt die Mauer (B-1175); (3) Stufe 1 soll wie ein Zaun oder eine Bretterwand aussehen statt wie eine glatte Platte (B-1176); (4) im Hub stehen ein weißes und ein braunes Platzhalter-Rechteck (B-1160). Vorgehen: Ursache je Punkt benennen (Kachelbreite und Rundung der `hintergrundmauer`, Mauerplätze in `data/hub.json` und `hub-geruest`, Asset `mauer` Stufe 1, Platz ohne Asset), mit Test beheben; die Stufe-1-Optik als neue Papierschnitt-Spec über MCP `assets_*` (`assets_guide` zuerst), Maße nach `data/sizes.json` (LA1.1). B-947 verlangt vor Beginn zu prüfen, ob RU55 die Lücken schon behoben hat; die Screenshots zeigen: nein. Gen darf die Lane zweit laufen lassen (`gen:paper` für die eigene Prüfung).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/art/**`
- `data/hub.json`, `data/paper.json`, `data/look.json`
- `public/grafik/gebaeude-paper/mauer*`, `public/grafik/gebaeude-paper/hintergrundmauer*`, `public/grafik/gebaeude-paper/hub-geruest*` (Specs, `index.paper.json`, `golden.json`; nur über `assets_*`)

## Nicht-Ziele

Größenstufen (LA1.1), Waldwand und Leitmotive (LA1.2), Stimmung (LA1.4, LA1.5), Kompass (LA1.3), neue Mauerregeln.

## Schritte

1. Die vier Punkte am laufenden Spiel nachstellen (Hub Stufe 1 bis 3, Cheat F9 › Alles ausbauen), Ursachen festhalten.
2. Lücken in der Kachelung und im Burgbereich beheben, Test für lückenlose Kachelung (B-947/AC-01).
3. Stufe-1-Mauer als Zaun oder Bretterwand: Spec über `assets_*`, rendern, neben der alten ansehen.
4. Platzhalter-Rechtecke: Platz benennen, Asset belegen oder entfernen (B-1160).
5. `cargo test -p k3c-game`, `task rust:lint`, einmal `task check:diff`; Vorher-Nachher-Bilder im Ergebnis.

## Fertig, wenn

- [ ] AC-06: Der Mauerzug ist lückenlos (Test für die Kachelung), im Burgbereich geschlossen, die Mauer der Stufe 1 ist ein Zaun oder eine Bretterwand, und im Hub steht kein einfarbiges Platzhalter-Rechteck mehr; Vorher-Nachher-Bilder liegen im Ergebnis.
- [ ] Nachprüfung 🧑 am PC (B-947/AC-02).
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

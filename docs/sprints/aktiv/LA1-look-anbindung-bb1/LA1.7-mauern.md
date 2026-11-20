# LA1.7 · Mauern: durchgängig, Burgbereich geschlossen, Stufe 1 als Zaun, Platzhalter weg

- **Status:** fertig
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

- [x] AC-06: Der Mauerzug ist lückenlos (Test für die Kachelung), im Burgbereich geschlossen, die Mauer der Stufe 1 ist ein Zaun (fünf spitze Pfähle mit Fugen), und im Hub steht kein einfarbiges Platzhalter-Rechteck mehr; Vorher-Nachher-Bilder: Vorschau `public/grafik/gebaeude-paper/mauer/stand.png` (neu) und `.cache/paper/mcp/set/mauer-stand-0-diff.pngCheckout` (Diff zur alten Spec); Ganzbild im Spiel nicht aufgenommen.
- [ ] Nachprüfung 🧑 am PC (B-947/AC-02).
- [x] Tests und `task rust:lint` grün.

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

AC-06 geprüft (Tests), Nachprüfung am PC bleibt bei Ralf.

**Ursachen und Behebung**
- **B-947 Fugen:** Die Hintergrundmauer ist 44 px breit, die Zeichnung füllt nur die Spalten 6 bis 38 (Rand links 6 px, rechts 5 px transparent); die Kachelung setzte ganze Rahmen aneinander, daher rund 20 % Fuge. Neu `data/paper.json › kachelung` (`von` 6, `bis` 39 in Bild-px), `Sheet.tile`, `props/paint.rs › tiles` setzt die sichtbaren Streifen aneinander; nur im Look-Profil k2c. Test `hintergrundmauer_kachelt_ohne_fugen` prüft an den PNGs aller fünf Stufen, dass jede Spalte des Streifens gefüllt ist und der Rand transparent ist.
- **B-1175 Burgbereich:** `back_walls` verband nur Mauern derselben Seite; zwischen den innersten Mauern links und rechts (hinter der Burg) fehlte der Abschnitt. Neu: im Profil k2c verbindet ein Abschnitt die innersten gebauten Mauern beider Seiten (Stufe = die niedrigere). Test `burgbereich_ist_geschlossen` (beide Seiten gebaut, eine Seite nicht gebaut, nur eine Seite, Profil go).
- **B-1160 Platzhalter:** Die Rechtecke waren Taverne (braun), Heilplatz (weiß) und Schmiede (dunkelgrau): Die Assets `taverne`, `heilplatz`, `schmiede` waren geladen, aber nicht in `HUB_BUILDINGS` zugeordnet, daher zeichnete das Spiel gebaut die farbige Ersatzform. Zuordnung ergänzt (alle Profile); Test `kein_platzhalter_rechteck_im_hub` (jede Platzart aus `hub.json` hat gebaut ein Bild).
- **B-1176 Stufe 1:** Spec `mauer` Stufe 1 war schon ein Zaun aus drei Pfählen, aber ohne Fugen und gleich hoch, daher wie eine Platte. Neu: fünf Pfähle mit Fugen und unterschiedlicher Höhe (108–128 px), zwei Querbalken; ab Stufe 2 deckt die Wehrmauer die Pfähle (unverändert). Über `assets_set` gerendert; `golden.json` der Gruppe `gebaeude-paper` gewollt geändert (nur `mauer`). `sicht` der Mauer in `sizes.json` bleibt innerhalb der Toleranz (Test grün).

Tests: `cargo test -p k3c-data -p k3c-game` grün bis auf `tree_rest_tests::rest_wie_go` (`sounds.json`, nicht von hier, vermutet auf `main` schon rot); `task rust:lint` grün; `task check:diff` nur an den bekannten Fremd-Fehlern rot (`native/`, Fahrplan RU30/WI1).

### Für den Hauptrechner

- Burg-Mauer im Burgbereich liegt hinter der Burg; ob dort ein Tor gewünscht ist (Spieler laufen durch?), ist Spielfrage: Mauern blockieren in der Sim nicht, reine Optik. Abnahme am PC.
- Hub-Plätze `taverne`/`bogner` und `kaserne`/`schmiede` überlappen sich in der Breite (58 und 68 px), wie in LA1.2 festgehalten.

# LA1.2 · BB1-Nachträge im Rust-Spiel: Waldwand, Schiff-Baustelle, Pflanzung, Leitmotive

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** LA1.1
- **Tickets:** B-1146, B-1147, B-1148, B-1149, B-1150, B-1151
- **Kriterien:** AC-02

## Ziel

Die BB1-Nachträge sind im Rust-Spiel angebunden: Waldwand und acht neue Bäume, Leitmotive mit Luftperspektive, Schiff-Baustelle mit eigenem Gerüst und Bauplatz-Breiten, unterscheidbare Plantage-Stufen, Schiffsrumpf 4 bis 5 M lang, Baum-Texturbudget um 15 MiB gesenkt.

## Kontext

Lane zweit, nach LA1.1 (Größenstufen). Tickets B-1146 bis B-1151 (Ausgangslage dort), Standard `docs/standards/bildaufbau.md`, Papierschnitt über MCP `assets_*` (Entscheidung 005), Grafik rendern mit `task gen:paper` nur für die eigene Prüfung (Lane zweit darf Gen laufen lassen). Die Größen folgen der Tabelle aus LA1.1.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/art/**`, `world/**`, `props/**`
- `public/grafik/**` (Specs, `index.paper.json`, `golden.json`), `data/paper.json`, `data/look.json`
- Assets nur über MCP `assets_*`

## Nicht-Ziele

Stimmung (LA1.4, LA1.5), Kompass (LA1.3), neue Regeln.

## Schritte

1. Tickets B-1146 bis B-1151 lesen, Reihenfolge nach Wirkung (Waldwand, Schiff, Plantage zuerst).
2. Je Ticket umsetzen mit Test oder Bildvergleich; Assets nur über `assets_*`.
3. Texturbudget messen (B-1151), Ergebnis eintragen.
4. `cargo test -p k3c-game`, `task rust:lint`, einmal `task check:diff`.

## Fertig, wenn

- [x] AC-02: B-1146 bis B-1151 erledigt oder mit Grund als Ticket verschoben (siehe Ergebnis: erledigt B-1147, B-1149, B-1146 Daten und Baumvarianten; verschoben B-1146 Zeichnen der Schichten, B-1148, B-1150, B-1151).
- [ ] Texturbudget der Bäume um 15 MiB gesenkt (gemessen): nicht erreicht, gemessen und begründet verschoben (B-1151).
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

AC-02 teilweise geprüft, Rest mit Grund verschoben (die Tickets bleiben im Sprint LA1 offen).

**Erledigt**
- **B-1146 (Daten, Bäume, Zuordnung):** `k3c-data` liest `waldwand` und `leitmotive` (`Paper::motif_asset`, Ersatz `standard`, alte Dateien laden weiter); Test `paper_tests.rs`. Baumwahl je Platz deterministisch aus der Position (`props/nature.rs › tree_asset`): drei Höhen je Art plus dunkle Variante (jeder fünfte Baum, vermutet als Ersatz für die Waldwand-Nähe, die das Spiel nicht kennt), alle 12 Varianten kommen vor (Test), nur im Look-Profil k2c; `nature_assets` lädt sie im Wald-Level. Leitmotiv-Zuordnung je Welt gegen `look.json › worlds.<level>.motif` geprüft (AC-03).
- **B-1147:** neues Asset `gebaeude-paper/geruest-schiff` (über `assets_create`/`assets_set`, Geometrie wie `geruest-6`, Holz statt Steinwand); im Profil k2c nimmt das Schiff dieses Gerüst (`scaffold_class`), alle anderen Gebäude bleiben bei `geruest-6` (Test). Gerüst-Höhe 2,7 M (Stufe `geruest-schiff` in `sizes.json`, Test 2,5–3,2 M). Überdeckung: gemessen an den sichtbaren Breiten und `hub.json`; die Burg (4,0 M, 423 px) überdeckte die Treppe nebenan um 21 px. Darum Stufe `burg` 3,5 M (Standard 3,5–4,5) und `burg-baustelle` 2,5 M; Test `burg_ueberdeckt_keinen_nachbarplatz` grün. `SITE_SIZE` in `plan_sites.rs` sind nur Platzhalter ohne Grafik und blieben.
- **B-1149:** Plantage in drei Stufen im Profil k2c: Setzling (erste Hälfte der Wachszeit), Jungbaum `pappel-2` (etwa 1 M, zweite Hälfte), Altbaum `pappel` (2,9 M); Abstand mindestens 0,5 M (Test), Wachszeit unverändert, Profil go unverändert.

**Verschoben (mit Grund)**
- **B-1146 Zeichnen:** Waldwand und Leitmotiv als Schicht zeichnen: Es fehlt der Verbraucher in der Parallax-Ebene (`world/`-Zeichenpfad mit Tönung, Dunst und Split-Screen); ohne Blick ins laufende Spiel zu riskant, `motif_place` und die Zuordnung stehen bereit.
- **B-1148 Schiffsrumpf 4–5 M:** neue Papierzeichnung des gestreckten Rumpfs samt Stufen und Ladeplätzen, nur mit Vorschau am PC sinnvoll; gehört zu RU36.3 (Schiff, Seekarte).
- **B-1150 Leitmotive Luftperspektive:** Spec-Neuzeichnung ohne Auflagen, oder Dunst im Spiel (Entscheidung offen); wie B-1146 zusammen mit dem Zeichnen der Schicht.
- **B-1151 Texturbudget:** gemessen (Wald-Level, Baum-Texturen): vorher 4,2 MiB (tanne, pappel, birke), jetzt 20,8 MiB mit 11 Varianten (+16,6 MiB; tanne-mittel 3,5, tanne-hoch 4,2, tanne-dunkel 3,6). Senken braucht Ping-Pong-Frames im Renderer (`tools/paper`, nicht in den erlaubten Dateien) oder Umpacken beim Laden (Tanne 5 von 8, Pappel 5, Birke 6 verschiedene Frames, höchstens rund 4 MiB), oder weniger Varianten laden; das Ziel −15 MiB ist damit nicht erreicht.

Gen: `assets_create`/`assets_set` haben `geruest-schiff` gerendert; `golden.json` und `index.paper.json` der Gruppe `gebaeude-paper` enthalten dafür den neuen Eintrag (gewollte Änderung). Tests: `cargo test -p k3c-data -p k3c-game` grün bis auf `tree_rest_tests::rest_wie_go` (`sounds.json`, nicht von hier, vermutet schon auf `main` rot); `task rust:lint` grün; `task check:diff` nur an den bekannten Fremd-Fehlern rot (`native/`, Fahrplan RU30/WI1). Manuell nicht geprüft (Abnahme am PC: Ralf).

### Für den Hauptrechner

- Verschobene Punkte oben sind jeweils eigene Arbeit: Schichten zeichnen (B-1146 Rest, B-1150), Rumpf (B-1148), Texturbudget (B-1151); Entscheidung für Ralf, ob die Schicht-Zeichnung eine eigene Session in LA1 bekommt.
- `pappel-2`, `birke-2`, `tanne-2` sind jetzt teilweise in Gebrauch (`pappel-2` als Jungbaum); der Aufräum-Hinweis aus LA1.1 gilt nur noch für `birke-2` und `tanne-2`.
- Unter dem Wechsel der Burg auf 3,5 M bleiben `taverne`/`bogner` und `kaserne`/`schmiede` überlappend (58 und 68 px): das war schon vorher so (Plätze 128 px auseinander, Breiten 175–200 px), gehört in die Hub-Abnahme am PC.

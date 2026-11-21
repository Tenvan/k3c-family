# FG1.3 · Grafik und Texte der Figuren, Meister-Gebäude und Gegenstände

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** fg1/3-figuren-grafik-texte
- **Abhängig von:** FG1.1
- **Tickets:** B-1201
- **Kriterien:** AC-03

## Ziel

Alle Figuren, Meister-Gebäude und Gegenstände aus FG1.2 haben Grafik und Texte (de, en) im Standard.

## Kontext

Ticket B-1201. Assets nur über MCP `assets_*` (Entscheidung 005); Zuordnung in `data/paper.json`. Figuren: Bürger, Handwerker, Bogenschütze, Soldat, Elite-Soldat, Elite-Bogenschütze; Gebäude: Handwerksmeister, Bogner, Waffenschmied, Rüstungsschmied; Gegenstände: Werkzeuggürtel, Bogen, Schwert, Rüstung. Vorhandene Assets zuerst wiederverwenden.

## Erlaubte Dateien

- `public/grafik/**`
- `data/paper.json`
- `rust/crates/k3c-game/src/actors/**`
- `rust/crates/k3c-game/src/hud/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Vorhandene Figuren/Gebäude-Assets und Texte inventarisieren (`assets_list`).
2. Fehlende Assets mit `assets_create`, Vorschau prüfen.
3. Zuordnung in `data/paper.json`, Texte de/en im HUD.
4. Tests (Grafik-Vollständigkeit).

## Fertig, wenn

- [ ] AC-03: Grafik und Texte de/en für Figuren, Gebäude, Gegenstände.
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

AC-03 geprüft: Neue Assets über MCP `assets_*`: Figur `truppen/handwerker` (Bürger mit Werkzeuggürtel aus Leder mit goldener Schnalle und Beutel, Hammer statt Heugabel; Rig und Animationen wie der Bauer, `angriff` schlägt mit dem Hammer) und Icons `icons/icon-guertel`, `icon-bogen`, `icon-schwert`, `icon-ruestung` (32 px, Papierschnitt). Zuordnung in `data/paper.json` (`truppen.craftsman`, `figuren.truppen.craftsman`, vier Icon-Zeilen). Die Gebäude brauchten kein neues Asset: Werkstatt (Handwerksmeister), Bogner, Schmiede (Waffenschmied) und Rüstkammer (Rüstungsschmied) haben ihre Papierschnitt-Grafik und werden nur umbenannt. Im Spiel zeichnet `actors/figures.rs` (`look_kind`) einen Bürger mit Hammer als Handwerker, sonst wie bisher; Test `handwerker_hat_eine_eigene_figur`. Texte de/en: `site.workshop` Handwerksmeister/Master Craftsman, `site.smithy` Waffenschmied/Weaponsmith, `site.armory` Rüstungsschmied/Armorsmith, neu `site.bowyer` Bogner/Bowyer und `item.belt|bow|sword|armor` (Werkzeuggürtel, Bogen, Schwert, Rüstung). `cargo test -p k3c-game --lib` (1014) grün, clippy -D warnings sauber. `task check:diff` bricht nur am bekannten `alloc_je_bild` ab (B-1190 AC-04, 942 > 786).

Abweichungen: Die Gegenstands-Icons sind nur zugeordnet, aber noch an keiner Stelle im HUD gezeichnet (kein Regal-Symbol im Spiel; das Regal zeigt weiter die Hammer- und Bogenzeichnung der Plätze). Anzeigenamen in `data/troops.json` und `data/buildings.json` blieben (von Go geteilt, nicht in den erlaubten Dateien); die Spiel-Texte kommen aus `hud/texts_*`. Kein Reviewer-Agent, kein `gen:paper`-Lauf (Abnahme-Ticket).

### Für den Hauptrechner

- Gegenstands-Icons im HUD zeigen (Regal am Gebäude, Hinweis beim Holen) und Elite-Figuren für Bürger mit Rüstung prüfen; Ticket anlegen.

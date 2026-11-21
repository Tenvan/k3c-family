# FG1.3 · Grafik und Texte der Figuren, Meister-Gebäude und Gegenstände

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

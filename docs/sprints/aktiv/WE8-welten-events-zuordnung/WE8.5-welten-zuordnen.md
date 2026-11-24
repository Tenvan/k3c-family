# WE8.5 · Sengoku, Nil, Britannien: Skins, Namen und Events über world_* zuordnen (B-996)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/we8
- **Abhängig von:** WE8.8
- **Tickets:** B-996
- **Kriterien:** AC-05

## Ziel

Sengoku, Nil und Britannien tragen ihre Skins, Variantennamen und ihr Event; `world_check` ist für alle drei fehlerfrei (B-996).

## Kontext

Weltdateien `data/worlds/sengoku.json`, `nil.json`, `britannien.json` (gitignored, nur im Hauptcheckout; Kopie im Spielpaket entsteht beim Build). Geändert wird nur über die MCP-Tools `world_get`, `world_set` (Feld `skins`, seit WE6.3), `world_override_set` (`enemies › <art> › name`, `events › events › <id>`), `world_check`; die Workbench muss laufen. Zuordnung nach B-996: Sengoku Torii = `objekte/ausgang`, Pagode = Turm, Samurai-Lager = `objekte/portal`, Ninja = `gegner/wolf`, Samurai = `gegner/hoehlentroll`, Event `raid`; Nil Palme = Bäume, Obelisk = `objekte/felsen`, Barke = Schiff, Skorpion = `gegner/wolf`, Mumie = `gegner/zombie`, Event `sandstorm`; Britannien Steinkreis = `objekte/portal`, Rundhaus = Bauernhaus, römischer Späher = `gegner/goblin-bogenschuetze`, Moorgeist = `gegner/grubengeist`, Event `fog`. Asset-Pfade aus `assets_list` (WE7). Kein Worktree.

## Erlaubte Dateien

- `data/worlds/sengoku.json`
- `data/worlds/nil.json`
- `data/worlds/britannien.json`

## Nicht-Ziele

Werte der Varianten ändern (Balancing gesperrt), Code, Assets, Standard-Welt, Simulation.

## Schritte

1. `assets_list`: Pfade der 14 Assets bestätigen; fehlt eines, Skin weglassen und Ticket anlegen.
2. Je Welt `world_set` mit `skins`, `world_override_set` für Namen und das Event (`everyNights` nach B-991/B-992, vorläufig).
3. `world_check` je Welt; `world_get` ins Ergebnis.
4. Agent `k3c-welten` prüfen lassen; Sichtprüfung 🧑 mit Prüfliste (je Welt Objekte, Varianten, Event-Nacht) vorbereiten.

## Fertig, wenn

- [ ] AC-05: `world_get` zeigt je Welt Skins, Namen und Event (B-996/AC-01).
- [ ] AC-05: `world_check` für alle drei Welten ohne Fehler (B-996/AC-02).
- [ ] B-996/AC-03 (Sichtprüfung 🧑 am PC) steht mit Prüfliste in der Ralf-Queue.

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

Teilweise erledigt, Status bleibt offen.

- AC-05 geprüft: Feindnamen über `world_override_set` gesetzt (sengoku, nil, britannien).
- AC-02 geprüft: `world_check` für sengoku, nil und britannien gültig.
- Skins: Default aus B-1259 Frage 14 angewandt. Ersatz-Assets fehlen (z. B. `gegner/ninja`), also keine Skins. Nur die Feindnamen gelten.
- Events (`raid`, `sandstorm`, `fog`): blockiert, weil `world_override_set` `welt-events` nicht kennt. Ticket B-1260 angelegt (Sprint WE8), Ziel und Anforderungen stehen dort.
- AC-03 Sichtprüfung 🧑: Prüfliste siehe Abschnitt oben in dieser Datei.

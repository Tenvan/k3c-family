# SI5.2 · Figur, Hütte, Bauplatz und Boni in der Engine (B-972)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/si5
- **Abhängig von:** SI5.1
- **Tickets:** B-972
- **Kriterien:** AC-02

## Ziel

Figuren-Katalog, Feld `hermit` der Welt-Datei, Hütte, Zahlung, Bauplatz `hermitHall` und die Boni von Dojo und Schrein sind in der Engine umgesetzt und getestet (B-972/AC-02 bis AC-09).

## Kontext

**Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`)**; keine Simulation, kein Balancing.

Regel: `docs/rules/buerger.md` § 1, `docs/rules/materialien-gebaeude.md` § 3 und § 4 (aus SI5.1), Spec `B-972`. Code: Hub-Plätze aus `data/hub.json` (`sites`, `islandSites` mit `kind`, `offsetUnits`, `hubLevel`), Aufbau in `engine/sim/hub_level.go`, Prüfung in `hub_validate.go`, `hub_validate_layout.go`; Gebäude `engine/sim/building_*.go`, Daten `data/buildings.json`; Bau, Zerstörung, Reparatur `building_repair.go`; Platz ohne Generator anlegen: Muster `addShipSite` in `ship.go`; Münzeinwurf `economy_pay.go`; Tagesanbruch `economy_dawn.go`; Schaden der Kämpfer `unit_warrior.go`, `unit_archer.go`, `combat.go`; Mauern als Gebäude. Welt-Datei: `island_worlds.go`, `docs/rules/welten.md` § 2 und § 3. Daten einbinden: `data/embed.go`. Ob bezahlt ist, ergibt sich aus dem Bauplatz (Bau begonnen oder Gebäude steht); kein neues Spielstand-Feld und kein Eingriff in `engine/level/**` – sonst anhalten und melden. Neue Dateien mit Präfix `hermit_`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Erlaubte Dateien

- `engine/sim/**`
- `data/hermits.json`
- `data/embed.go`
- `data/hub.json`
- `data/buildings.json`
- `data/worlds/sengoku.json`
- `data/worlds/nil.json`
- `docs/rules/welten.md`
- `docs/rules/buerger.md`
- `docs/rules/materialien-gebaeude.md`

## Nicht-Ziele

Walfänger, Tranhütte, Sichtregel, Ausbau des Bauwerks, Anzeige (SI5.4), Welten-Editor (SI7), Werte abstimmen.

## Schritte

1. `data/hermits.json` (`swordsmith` → `dojo`, `damageFactor` 1,15; `priestess` → `shrine`, `wallHealFraction` 0,25; `price` 20, `buildSeconds` 20, `hp`), einbinden, laden, prüfen.
2. Welt-Datei: optionales Feld `hermit`, Prüfung gegen den Katalog; `sengoku` und `nil` setzen; `welten.md` § 2 und § 3 ergänzen.
3. Hub-Platz `hermitHall` in `data/hub.json` › `islandSites` (ab Hub-Stufe 2, freier Versatz, Hub-Prüfung grün); nur in Welten mit Figur.
4. Hütte in Tiefe 1 am fernen Rand, deterministisch; keine Hütte bei nur einer Stufe.
5. Zahlung an der Hütte startet den Bau im Hub; zweite Zahlung abgelehnt.
6. Boni: Dojo (Schaden der Krieger und Schützen), Schrein (Mauern bei Tagesanbruch); ruhen bei zerstörtem Bauwerk.
7. Zustand für die Anzeige (Hütte belegt oder leer, Figur unterwegs als Zeitfenster nach der Zahlung, Bauwerk).
8. Tests je Kriterium, auch Speichern und Laden ohne neue Version; Reviewer-Agent `k3c-welten` vor dem Commit.

## Fertig, wenn

- [ ] AC-02: B-972/AC-02 bis AC-08 je durch einen Go-Test belegt (Testname im Ergebnis).
- [ ] B-972/AC-09: `check_run task:check:diff` grün, `testdata/golden/` unverändert oder begründet.
- [ ] Dev-Workbench des Worktrees gestoppt.

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

–

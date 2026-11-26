# WK2.1 · Zwei Bürger je Tag, Handwerker ab Sonnenaufgang (B-1246, B-1253)

- **Status:** blockiert
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/wk2
- **Abhängig von:** –
- **Tickets:** B-1246, B-1253
- **Kriterien:** AC-01, AC-02

## Ziel

Standard bringt zwei Bürger je Tag; Handwerker beginnen bei Sonnenaufgang.

## Kontext

B-1246: „pro Tag nur zwei Leute im Standard“. B-1253: Arbeit ab Sonnenaufgang, Schutz am Lagerfeuer nur in der Nachtperiode oder bei Feindkontakt. Logik in `rust/crates/k3c-sim/src` (`unit/`, `world/`, `economy/`, `hub/`); Werte in `data/`.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `data/**`
- `docs/rules/**`

## Nicht-Ziele

Andere Welten umstellen, Balancing, `engine/**`, `testdata/**`.

## Schritte

1. Fehlschlagende Tests je Regel.
2. Spawn-Regel und Arbeitsbeginn ändern, Regeltext nachziehen.

## Fertig, wenn

- [ ] AC-01: Test belegt zwei Bürger je Tag in Standard.
- [ ] AC-02: Test belegt Arbeit ab Sonnenaufgang.

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

- AC-02 umgesetzt, geprüft mit `unit/danger_tests.rs`: Handwerker arbeiten am Tag trotz ferner Gegner; Gefahr nur nachts oder bei Feindkontakt (8 Units). Zwei Tests an die neue Regel angepasst (`outpost_tests`, `hub/level_tests`), Regeltext in `buerger.md`.
- AC-01 nicht umgesetzt (B-1246): braucht ein neues Datenfeld bei `recruitCamp`.
- Blockiert: `task:check:diff` bleibt rot durch 4 `k3c-sim`-Tests, die ohne diese Änderung ebenso scheitern.

### Für den Hauptrechner
- `main` ist rot (CI zu ab5f30539): `k3c-game` `drei_stufen_mindestens_halbe_monarchenhoehe_auseinander`.
- Auch ohne Änderung rot in `k3c-sim`: `eingebettete_welten_gueltig`, `standard_welt_gleich_bisherigen_inseln`, `insel_nach_1000_schritten_wie_go`, `standard_welt_erzaehlt_eine_geschichte` (data/worlds enthält britannien, nil, sengoku).
- B-1246 entscheiden: „zwei je Tag“ je Lager, je Stufe oder je Insel? Ein neues Feld in `data/economy.json` berührt die eingefrorene Go-Engine und ihre Golden.
- AC-02: Handwerker können an einem nahen Gegner zwischen Arbeit und Flucht pendeln; im Spiel prüfen.

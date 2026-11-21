# HB1.5 · Hafen als Start, 1 bis 2 Biome vom Lager, Hafen vom Wald umgeben

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/hb1
- **Abhängig von:** HB1.4
- **Tickets:** B-1188
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Neue Spiele starten am Hafen; der Seed legt 1 bis 2 Biome zwischen Lager und Hafen; der Hafen ist von Bäumen umgeben (Dichte einstellbar) und muss freigeräumt werden.

## Kontext

Ticket B-1188. Inselaufbau, Start und Hafen in `rust/crates/k3c-sim`, `k3c-room`, `k3c-level`; Regeln `docs/rules/stufen.md`, `docs/rules/welten.md`. Zuerst grep nach Hafen/harbor/start. Dichte als Datenwert in `data/`. Keine Rücksicht auf alte Spielstände. Go-Engine eingefroren (Entscheidung 013).

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-room/**`
- `rust/crates/k3c-level/**`
- `data/**`
- `testdata/**`

## Nicht-Ziele

Neue Inseln, Seekarte.

## Schritte

1. Startpunkt neuer Spiele auf den Hafen setzen.
2. Zahl der Biome zwischen Lager und Hafen (1 bis 2) aus dem Seed.
3. Hafen mit Bäumen umgeben, Dichte als Datenwert, Test.
4. Tests über viele Seeds.

## Fertig, wenn

- [ ] AC-01: Neue Spiele starten am Hafen.
- [ ] AC-02: 1 bis 2 Biome zwischen Lager und Hafen (Test über viele Seeds).
- [ ] AC-03: Hafen von Bäumen umgeben, Dichte einstellbar (Test).
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

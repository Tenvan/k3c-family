# WK2.1 · Zwei Bürger je Tag, Handwerker ab Sonnenaufgang (B-1246, B-1253)

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

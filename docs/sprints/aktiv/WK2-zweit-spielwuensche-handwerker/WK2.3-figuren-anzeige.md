# WK2.3 · Keine Forke, Tätigkeits-Symbol über Handwerkern (B-1245, B-1254)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/wk2
- **Abhängig von:** WK2.2
- **Tickets:** B-1245, B-1254
- **Kriterien:** AC-04, AC-05

## Ziel

Bürger ohne Werkzeug haben leere Hände; Handwerker zeigen ihre Tätigkeit als Symbol.

## Kontext

B-1245: Forken nur für Bauern, die es nicht mehr gibt. B-1254: Symbol je Tätigkeit. Figuren in `rust/crates/k3c-game/src/actors`, Symbole mit vorhandenen Icons (`hud/icons.rs`); fehlt ein Asset, bleibt es bei einem gezeichneten Zeichen, Grafik-Assets nur über `assets_*`. Das Frame-Budget (60 Hz, zwei Ansichten) halten.

## Erlaubte Dateien

- `rust/crates/k3c-game/**`
- `rust/crates/k3c-sim/**`
- `data/paper.json`

## Nicht-Ziele

Neue Figuren zeichnen.

## Schritte

1. Tests für Haltung und Symbolwahl.
2. Umsetzen.

## Fertig, wenn

- [ ] AC-04: Test belegt leere Hände ohne Werkzeug.
- [ ] AC-05: Test belegt Symbol je Zustand.

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

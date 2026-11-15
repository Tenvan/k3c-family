# WE7.4 · Review WE7 und Abnahme-Ticket (Gen, Grafik-Golden)

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/we7
- **Abhängig von:** WE7.1, WE7.2, WE7.3
- **Tickets:** B-988, B-989, B-990
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Der Sprint WE7 ist geprüft, das Abnahme-Ticket für Gen und Grafik-Golden steht, die Sichtprüfung 🧑 ist vorbereitet.

## Kontext

Sprint-README `docs/sprints/*/WE7-welten-assets/README.md` mit AC-01 bis AC-04, Tickets B-988, B-989, B-990. Prüfer: Agent `k3c-grafik` (Stil, Texturbudget, Verträge aus `tools/paper`). `gen:paper` samt Grafik-Golden (`public/grafik/<gruppe>/golden.json`) läuft nur im Abnahme-Ticket (B-742, Sperre `golden` gilt dort).

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `public/grafik/gegner/**`
- `public/grafik/gebaeude-paper/**`

## Nicht-Ziele

Neue Assets, Zuordnung in den Welten (WE8.5).

## Schritte

1. Alle 14 Assets mit `assets_get` gegen ihre Basis prüfen; `k3c-grafik` über den Sprint-Diff.
2. Schwere Befunde über `assets_set` beheben, Rest als Ticket.
3. `task check` (ohne Gen).
4. Abnahme-Ticket anlegen (AC-03 Gen: `check_run task:test` samt `gen:paper` und Golden), Abschnitt Abnahme füllen, Review-Liste für 🧑 im Bereich Assets nennen.

## Fertig, wenn

- [ ] AC-01 bis AC-04 mit Nachweis in der Sprint-README › Abnahme.
- [ ] Genau ein Abnahme-Ticket angelegt; Sichtprüfung 🧑 (B-988/AC-04, B-989/AC-04, B-990/AC-04) in der Ralf-Queue.

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

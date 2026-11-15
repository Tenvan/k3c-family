# GS4.1 · Regeltext: Flaschenpost (B-971)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs4
- **Abhängig von:** –
- **Tickets:** B-971
- **Kriterien:** AC-01

## Ziel

`docs/rules/wirtschaft.md` hat den Abschnitt Flaschenpost mit allen Regeln, die die Simulation braucht.

## Kontext

Spec: `docs/backlog/B-971-flaschenpost.md` (Anforderungen R1–R10, Ausnahmefälle, Offene Fragen mit Defaults). Regelwerk: `docs/rules/wirtschaft.md` § 1 (Truhe: 10–25 Gold, jede 3. gibt einen Skill-Punkt) im Tabellenformat Regel, Begründung, Daten, Zielkorridor. Tag und Nacht: Phasen `day`, `dusk`, `night`, `dawn` (`engine/sim/world_cycle.go`). Stufe 1 ist Tiefe 0; nicht jede Insel hat sie. Werte kommen nach `data/bottle.json` (legt GS4.2 an): `firstDay` 3, `everyDays` 4, Abstand zum Rand, Reichweite am Ziel – alle vorläufig, Balancing später (gesperrt). Quelle der Entscheidungen: 🧑 2026-11-15 (Ideenrunde); Defaults als „vorläufig“ kennzeichnen. `story.bottles` steht schon in `docs/rules/welten.md` (GS1). Kein Code.

## Erlaubte Dateien

- `docs/rules/wirtschaft.md`

## Nicht-Ziele

Code, Daten-Datei, Texte der Flaschenpost-Zeilen (B-976), Regeln für Einsiedler und Gatter.

## Schritte

1. Neuer Abschnitt „Flaschenpost“: Rhythmus, Ort, Bedingung, Aufheben, Flut, Ziel, Markierung, Geschichtszeile, Spielstand, je Regel Begründung und Daten.
2. Beispiel mit Tagen (B-971 › Beispiele) und Ausnahmefälle übernehmen.
3. Verweis in § 1 (Truhe) auf die Flaschenpost als Wegweiser.
4. Glossar-Texte (Flaschenpost, Markierung) für den letzten Commit des Sprints im Ergebnis vorbereiten.

## Fertig, wenn

- [ ] AC-01: Abschnitt Flaschenpost mit Rhythmus, Ort, Aufheben, Flut, Ziel, Markierung und Verweis auf `data/bottle.json` (B-971/AC-01).
- [ ] `check_run task:check:diff` grün.

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

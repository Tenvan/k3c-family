# RG2.2 · Review und Abschluss RG2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/rg2
- **Abhängig von:** RG2.1
- **Tickets:** B-185, B-219, B-295, B-203, B-024
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

Der Sprint RG2 ist geprüft und abgeschlossen: alle Kriterien nachgewiesen, Glossar angeglichen, Tickets mit Vorschlag für den Status.

## Kontext

Doku-Sprint, eine Umsetzungs-Session (RG2.1). Kriterien in `docs/sprints/geplant/RG2-regelwerk-klaerungen/README.md` (nach dem Aktivieren unter `aktiv/`). B-024 (Tiefe 3 und 4) ist schon erfüllt, Belege in B-024 › Notizen; hier nur gegenprüfen. B-203 (Gold-Schwelle) entfällt, Vorschlag verworfen. `docs/glossar.md` Zeile 105 (Geselle) nennt noch Schmiede und Rüstkammer; als Sammeldatei wird sie im letzten Commit des Sprints geändert. Kein Abnahme-Ticket nötig: Der Sprint ändert weder Sim noch Grafik noch Werte.

## Erlaubte Dateien

- `docs/rules/wirtschaft.md`
- `docs/rules/zielkorridore.md`
- `docs/rules/buerger.md`
- `docs/rules/monarch.md`
- `docs/game-design.md`

## Nicht-Ziele

Neue Regeln oder Werte; Code; Simulation und Balancing.

## Schritte

1. Jedes Kriterium AC-01 bis AC-05 mit den `grep`-Befehlen der Sprint-README prüfen.
2. `docs/glossar.md` › Geselle: nur Werkstatt (letzter Commit, nach `git merge origin/main`).
3. Befunde unter etwa 30 Zeilen direkt beheben, sonst Ticket.
4. `task check`, danach Abnahme in der Sprint-README (höchstens fünf Zeilen); Tickets B-185, B-219, B-295 erledigt, B-024 erledigt, B-203 verworfen jeweils nur nach Bestätigung der Defaults durch 🧑.

## Fertig, wenn

- [ ] AC-01 bis AC-05 je mit Befehl und Ergebnis im Abschnitt Ergebnis.
- [ ] `grep -n "Geselle" docs/glossar.md` nennt nur die Werkstatt.
- [ ] `task check` grün; Abnahme in der Sprint-README ausgefüllt.

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

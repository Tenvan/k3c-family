# MG9.5 · Review Sprint MG9

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/mg9
- **Abhängig von:** MG9.2, MG9.3, MG9.4
- **Tickets:** B-716, B-717, B-718, B-719
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Sprint MG9 ist geprüft und abgenommen, der PR nach `develop` ist offen.

## Kontext

Review-Session nach `docs/arbeitsweise.md` › Review-Session auf Branch `sprint/mg9`; Diff `origin/develop...sprint/mg9` lesen.

## Erlaubte Dateien

- `docs/sprints/aktiv/MG9-papier-figuren-nachbessern/**`
- `docs/backlog/**` (Status der Tickets B-716 bis B-719, Folgetickets)

## Nicht-Ziele

Neue Gestaltung; Gerätabnahmen.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander (check:race lokal ohne gcc: als übersprungen nennen).
2. Diff lesen, Kriterien AC-01 bis AC-04 gegen die Session-Ergebnisse prüfen.
3. Abnahme (höchstens fünf Zeilen) in die Sprint-README, Tickets archivieren, Sprint auf erledigt.

## Fertig, wenn

- [ ] AC-01: B-716/AC-01 bis AC-03 gegen Ergebnis MG9.1 geprüft.
- [ ] AC-02: B-717/AC-01 bis AC-03 gegen Ergebnis MG9.2 geprüft.
- [ ] AC-03: B-718/AC-01, AC-02 gegen Ergebnis MG9.3 geprüft.
- [ ] AC-04: B-719/AC-01 bis AC-03 gegen Ergebnis MG9.4 geprüft.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

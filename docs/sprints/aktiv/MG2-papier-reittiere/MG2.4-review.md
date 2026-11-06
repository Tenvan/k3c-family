# MG2.4 · Review MG2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** reittiere
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg2/4-review
- **Abhängig von:** MG2.3
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG2 ist geprüft und abgenommen: alle Reittiere gültig, gerendert, Golden aktuell, Befunde behoben oder als D-Zeile/Ticket.

## Kontext

Sprint-README und Ergebnisse MG2.1–MG2.3; Prüfung nur über `assets_list`, `assets_preview`, `assets_review` mit `checkout`; Sichtprüfung durch 🧑 in PS1.

## Erlaubte Dateien

- `public/grafik/reittiere/**` (nur Befund-Fixes)
- `docs/sprints/*/MG2-papier-reittiere/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Assets.

## Schritte

1. `assets_list {gruppe: reittiere}`, `assets_review {basis: origin/develop}` bzw. gegen den Start-Commit.
2. Je Asset alle drei Gangarten Frame 0 und 4 per `assets_preview` ansehen; Stichprobe Monarch beim Reiten (`monarch galopp`) mit pferd.
3. `task gen:paper`, `task test -- paper`, `task check:diff`.
4. Abnahme in der Sprint-README, D-Zeilen bestätigen.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis im Ergebnis.
- [ ] Abnahme eingetragen.

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

# MG5.3 · Review MG5

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** bosse
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg5/3-review
- **Abhängig von:** MG5.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG5 ist geprüft und abgenommen: alle Bosse gültig, gerendert, Golden aktuell, Befunde behoben oder als D-Zeile.

## Kontext

Sprint-README und Ergebnisse MG5.1–MG5.2; Prüfung nur über `assets_list`, `assets_preview`, `assets_review` mit `checkout`; Sichtprüfung durch 🧑 in PS1.

## Erlaubte Dateien

- `public/grafik/bosse/**` (nur Befund-Fixes)
- `docs/sprints/*/MG5-papier-bosse/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Assets.

## Schritte

1. Getrennter Reviewer (Opus) über `assets_list {gruppe: bosse}`, `assets_review` gegen den Start-Commit, `assets_preview` je Boss.
2. Befunde beheben (kleine Patches) oder als D-Zeile.
3. `task gen:paper`, `task test -- paper`, `task check:diff`; Abnahme in der Sprint-README.

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

# MG8.4 · Münze und Gold-Icons als requisite

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** objekte
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg8/4-muenzen
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Münze (`muenze`, mit `idle`-Kippen) und die Gold-Icons `icon-muenze`, `icon-muenzstapel`, `icon-muenzhaufen`, `icon-pokal`, `icon-goldbarren`, `icon-truhe`, `icon-krone`, `icon-schwerter` liegen als `requisite` unter `public/grafik/icons/`.

## Kontext

`docs/assets/zuordnung-welt.md` › Icons (`icon:1` … `icon:10`, `coin`, `guide:coin`). Icons klein (Ausgabe 32–48 px), klare Silhouette.

## Erlaubte Dateien

- `public/grafik/icons/**`
- `docs/sprints/aktiv/MG8-papier-natur-objekte-icons/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG8-Zeilen)

## Nicht-Ziele

Kein Icon-Sheet, kein `assets_style_set`, keine anderen Gruppen.

## Schritte

1. Je Asset `assets_create {gruppe: icons}` und `assets_set`.
2. Jedes Asset mit `assets_preview` prüfen.
3. `task gen:paper`, `assets_review`, `bunx vitest run tools/paper`.
4. Ergebnis, Commit `feat(pap): Münze und Gold-Icons (MG8.4)`.

## Fertig, wenn

- [ ] AC-01: Assets gültig.
- [ ] AC-02: Golden eingetragen, Tests grün.
- [ ] AC-03: alle mit `assets_preview` geprüft; Review ohne roten Eintrag.

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

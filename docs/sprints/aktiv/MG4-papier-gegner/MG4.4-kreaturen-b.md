# MG4.4 · Eisenkäfer, Feuergeist, Kristallspinne, Splitterling

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gegner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg4/4-kreaturen-b
- **Abhängig von:** MG4.3
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Eisenkäfer (`eisenkaefer`), Feuergeist (`feuergeist`), Kristallspinne (`kristallspinne`) und Splitterling (`splitterling`) als `requisite`-Specs mit Keyframe-Animationen `idle`, `lauf`, `angriff` unter `public/grafik/gegner/`.

## Kontext

- `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling` aus `data/enemies.json`; Spinne als `requisite` mit Keyframes (B-701 D-08), Beine als eigene Teile.
- Aufbau wie MG4.3.

## Erlaubte Dateien

- `public/grafik/gegner/**`
- `docs/sprints/aktiv/MG4-papier-gegner/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG4-Zeilen)

## Nicht-Ziele

Keine neuen Rig-Typen, kein `assets_style_set`.

## Schritte

1. Ergebnis MG4.3 lesen.
2. Je Gegner `assets_create {rig: requisite}` und `assets_set`.
3. Jede Animation mit `assets_preview` prüfen.
4. `task gen:paper`, `assets_list`, `assets_review`, `tools/paper`-Tests.
5. Ergebnis, Commit `feat(pap): Gegner Kreaturen B (MG4.4)`.

## Fertig, wenn

- [ ] AC-01: alle 17 Assets der Gruppe gültig.
- [ ] AC-02: Golden vollständig, `tools/paper` grün.
- [ ] AC-03: jede Animation geprüft; Review ohne roten Eintrag.

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

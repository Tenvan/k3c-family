# MG4.5 · Review des Sprints MG4

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** gegner
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg4/5-review
- **Abhängig von:** MG4.1, MG4.2, MG4.3, MG4.4
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG4 geprüft: alle 17 Gegner gültig, gerendert, mit Golden, lesbar unterscheidbar; Abnahme ausgefüllt.

## Kontext

Sprint-README, Ergebnisse MG4.1–MG4.4; Review mit frischem Blick (Opus) auf alle PNG-Sheets, wie MG3.3.

## Erlaubte Dateien

- `public/grafik/gegner/**` (Korrekturen über `assets_set`)
- `docs/sprints/aktiv/MG4-papier-gegner/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Assets; Sichtprüfung durch 🧑 in PS1.

## Schritte

1. `assets_list {gruppe: gegner}`, `assets_review` gegen den Start-Commit.
2. Review aller Sheets (frischer Kontext), wichtige Befunde über `assets_set` beheben, kleine als D-Zeile.
3. `task gen:paper`, `tools/paper`-Tests, `task check`.
4. Abnahme, Commit `chore(pap): Review MG4`.

## Fertig, wenn

- [x] AC-01 bis AC-03 mit Nachweis.
- [x] `task check` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `assets_list {gruppe: gegner}` meldet 17 Assets, 0 ungültig.
- AC-02 geprüft: `task gen:paper` (fremde Gruppen zurückgesetzt), `bunx vitest run tools/paper tests/planning.test.ts` 1110/1110, `task check` grün (42,9 s).
- AC-03 geprüft: Ein Opus-Reviewer hat alle 51 Sheets angesehen, dazu Randprüfung und Größen-Aufreihung, und 6 wichtige sowie 9 kleine Befunde gemeldet. Behoben über `assets_set`:
  - Troll und Kristallwächter 300 px (größer als der Monarch), Grubengeist 150 px.
  - Wächter-Angriff mit Ausholen über den Kopf und Schlag nach unten.
  - Fledermaus mit weiterem Flügelschlag und Sturzflug (Korpus +50°).
  - Lavaschleim mit tropfender Kontur und glühender Kruste statt Braun.
  - Eisenkäfer-Zangen öffnen sichtbar.
  - Ratten wippen ±12° und tauchen im Angriff nicht mehr unter den Boden.
  - Rahmen unten erweitert (Wölfe, Käfer, Splitterwicht).
- Rest als D-MG4-07 zurückgestellt bis PS1. Vor allem: Wächter dem Troll zu ähnlich, Bogen/Pfeil des Goblin-Bogenschützen, Feuerball und Flammenkrone.
- `task check:go`/`check:race` nicht gefahren: keine Go-Änderung im Sprint.
- Manuell geprüft: niemand. Sichtprüfung durch Ralf in PS1. Neue Tickets: keine.

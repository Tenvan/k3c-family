# MG7.3 · Review MG7

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** hintergruende
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** mg7/3-review
- **Abhängig von:** MG7.2
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Sprint MG7 ist geprüft und abgenommen: alle 30 Hintergründe gültig, gerendert, Golden aktuell, Befunde behoben oder als D-Zeile.

## Kontext

Sprint-README und Ergebnisse MG7.1–MG7.2; Prüfung nur über `assets_list`, `assets_preview`, `assets_review` mit `checkout`; Sichtprüfung durch 🧑 in PS1.

## Erlaubte Dateien

- `public/grafik/hintergruende/**` (nur Befund-Fixes)
- `docs/sprints/*/MG7-papier-hintergruende/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md`

## Nicht-Ziele

Keine neuen Assets.

## Schritte

1. Getrennter Reviewer (Opus) über `assets_list {gruppe: hintergruende}`, `assets_review` gegen den Start-Commit, `assets_preview` je Asset; Kachelbarkeit der Profile prüfen.
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

- Getrennter Reviewer (Opus, nur lesend): `assets_list`, `assets_review` gegen e3c95f7f, 30 `assets_preview`, Kachelprüfung per PIL (Spalte 0 gegen 767: 26/30 ohne Abweichung, 4 mit 1–2 Zeilen Kantenglättung, kein Motiv am Rand abgeschnitten).
- **AC-01 geprüft:** 30/30 gültig, Rig `hintergrund`, 768 × 480.
- **AC-02 geprüft:** `task gen:paper` grün, `golden.json` und `index.paper.json` je 30 Einträge, Hashes passen zu den PNGs; `task test -- paper` 187/187 grün.
- **AC-03 geprüft:** alle 30 per Preview (Umsetzung und Review), `assets_review` ohne roten Eintrag. Sichtprüfung durch 🧑 in PS1 offen.
- Befund behoben: ausgestanzte Löcher am Säulenfuß in `hg-hoehle-mitte-*` (Stalagmiten jetzt eigenes Teil `kegel`).
- Übrige Befunde (Lesbarkeit, Tiefe, Varianten) als D-MG7-06 für PS1.

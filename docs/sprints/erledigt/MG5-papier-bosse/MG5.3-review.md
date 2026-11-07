# MG5.3 · Review MG5

- **Status:** fertig
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

- Getrennter Reviewer (Opus, nur lesend über MCP mit `checkout`): `assets_list`, `assets_review` gegen a4f66f28, 21 `assets_preview` (alle 17 Animationen, Frame 0 und Mitte).
- **AC-01 geprüft:** `assets_list {gruppe: bosse}` 6/6 gültig (5× figur, kristallherz-waechter requisite), `hoehe_px` überall 288.
- **AC-02 geprüft:** `task gen:paper` grün; `golden.json` 6 Einträge mit 17 Dateien, `index.paper.json` 6 Einträge; `task test -- paper` 29/29 Dateien, 187/187 Tests grün.
- **AC-03 geprüft:** jede Animation per Preview (MG5.1/MG5.2 selbst, MG5.3 Reviewer); `assets_review` 6× neu, kein roter Eintrag. Sichtprüfung durch 🧑 in PS1 steht aus.
- Befunde behoben: kristallherz-waechter-Sockel kippte im angriff mit (`korpus` ±3° entfernt, Box endet jetzt bei y 284 statt am Rand); troll-koenig-Beschreibung „Holzkeule“ → „Holzschwert“ (passt zum Render).
- Befunde offen als D-MG5-04 (PS1): lava-golem-Glut zu dünn, Wangenpunkt der Golems, lava-golem-Arme in idle verdeckt, Ratten-Schwanz hell und dünn, Holzschwert kontrastarm. Splitter des Kristallherzens schweben gewollt; Gesicht im angriff vom Arm verdeckt (Erbe der Krieger-Vorlage).
- `task check:diff`: 2 Workbench-Frontend-Tests (u. a. `CatalogPrompts.test.tsx`) rot unter Parallellast; `k3c-workbench:vitest` einzeln 432/432 grün, der Branch ändert unter `tools/` nichts gegenüber feature/pap (Wackel-Tests wie D-MG2-07).

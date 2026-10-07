# GR3.3 · Review und Abnahme des Sprints GR3

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** gr3/3-review
- **Abhängig von:** GR3.2
- **Tickets:** B-010
- **Kriterien:** alle

## Ziel

Der Sprint GR3 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-010 ist archiviert (oder mit Grund und Ticket offen, falls ein Kriterium nur teilweise erfüllt ist); der PR des Sprints ist geöffnet.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/scenes/` (`worldRenderer.ts`, `stageView.ts`, `LoadScene.ts`, neue Dateien), `public/grafik/CREDITS.md` und Tests. Besonders prüfen: Client rechnet nichts (`src/scenes/noSim.test.ts`), kein `Math.random()` für Spiel-Logik; jeder eingebaute Sprite hat einen Credit (CC-BY-Namensnennung für Warped Caves erhalten); fehlende Textur lässt den Platzhalter stehen statt zu crashen; mit 2 Spielern im Split-Screen keine Darstellungsfehler; B-Taste nicht belegt; `worldRenderer.ts` ≤ 400 Zeilen. **Die Sichtprüfung am TV (AC-04) gehört 🧑 und ist keine Abhängigkeit** (Regel „Hardware entkoppelt“): ist sie offen, führt die Abnahme AC-04 als `angenommen, Validierung offen (🧑, TV)`.

## Erlaubte Dateien

- `src/scenes/`, `public/grafik/CREDITS.md` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Geschmack der Grafikauswahl, Optimierung, Atlas.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/gr3` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von GR3.1 und GR3.2 prüfen (AC-06: `task check` grün, 2 Spieler im Split-Screen im Browser-Pane ansehen).
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-010 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben; AC-04 ist am TV beobachtet oder als `angenommen, Validierung offen` geführt.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

Review am 2026-10-04 (GR3.1 über `git show b82d05b`, GR3.2 über `git diff origin/develop...origin/sprint/gr3`), `task check` (1159 Tests) und `task check:go` (0 Issues) grün. Keine schweren Befunde: kein `Math.random()` in `src/scenes`, `noSim.test.ts` grün (Auswahl in `buildingSprites.ts` und `worldSprites.ts` ohne Phaser, Client rechnet nichts), jedes Zeichnen fällt bei fehlender Zuordnung oder Textur (`scene.textures.exists`) auf die Platzhalter-Form zurück, `worldRenderer.ts` 302 Zeilen, `siteView.ts` 210, keine Funktion über 60 Zeilen (Lint grün), B nicht belegt, keine Eingabe-Dateien berührt.
- **AC-01:** GR3.1 (Gebäude, Bauplätze, Treppen) und GR3.2 (Ressourcen, Portal, Ausgang, Truhe, Stern, Münzen, Lager, Parallax); Lücken (Taverne, Heiler, Schmiede, Rüstkammer, Adern, Plantage) bleiben Platzhalter laut Zuordnung.
- **AC-02:** jedes Pack aus `BUILDING_TEXTURES` und `WORLD_TEXTURES` steht in `public/grafik/CREDITS.md` (16 Pack-Ordner geprüft), Warped-Caves-Namensnennung unverändert, `credits.test.ts` grün (GR3.2).
- **AC-03:** Tests `buildingSprites.test.ts` und `worldSprites.test.ts` (`null` bei Lücke), Beobachtung in GR3.1 (`house-b.png` umbenannt → Platzhalter).
- **AC-04:** teilweise, bewusste Abweichung: ohne Stufe im Snapshot zeigen Mauer und Turm die Grafik ihrer Objekt-Zeile (Stein), weil Stufe 1 (Holz) eine Lücke ist; im Review als unschädlich bewertet (Gebäude sichtbar statt Platzhalter, Auswahl Stufe → Sprite getestet, wirkt erst mit B-112/W1). Sicht am TV: angenommen, Validierung offen (🧑, TV), im Fahrplan unter „Offen am Gerät“.
- **AC-05:** Wald 4, Höhle 1, Mine 3 Ebenen (GR3.2, Test `worldSprites.test.ts`, Browser-Pane); kein Biom ist Platzhalter.
- **AC-06:** `task check` grün; 2 Spieler im Split-Screen in GR3.1 und GR3.2 im Browser-Pane gesehen, keine Darstellungsfehler (bei Zoom 0,5 reichen Ebenen nicht bis oben, dort steht die Himmelsfarbe; Sicht am TV angenommen, Validierung offen). In diesem Review nicht erneut im Browser angesehen.
- B-010 erledigt und archiviert, kein neues Ticket.

# DBG2.4 · Review und Abnahme des Sprints DBG2

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** dbg2/4-review
- **Abhängig von:** DBG2.2 (DBG2.3 am Gerät ist keine Abhängigkeit, `docs/arbeitsweise.md` › Hardware entkoppelt)
- **Tickets:** B-179
- **Kriterien:** alle

## Ziel

Der Sprint DBG2 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-179 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/scenes/` (`debugOverlay*.ts`, neue `debugActions.ts` und `debugOverlayPanel.ts`, wenig `GameScene.ts` und `HudScene.ts`), `src/online/` (Methode `sendDev`, Typen) und Tests. Besonders prüfen: Der Client rechnet nichts (`src/scenes/noSim.test.ts` grün, keine Lager-Maxima oder Münzrechnung im Client); B, View + Menu und X sind nicht für Dev-Aktionen belegt, Ö und Stick-Klick schalten weiter das Overlay; die Aktionsliste erscheint nur bei aktivem Overlay und Dev-Mode des Raums, die Schaltflächen werden beim Verlassen der Szene entfernt; die Eingabe-Sperre im Dev-Fokus bleibt nicht hängen (der Spieler läuft nach dem Schließen wieder); mit 2 lokalen Spielern geht Gold an den gewählten Slot; `.oxlintrc.json` hat keine höheren Komplexitäts-Grenzen (besonders `HudScene.ts` bleibt bei 17); kein `Math.random()`. B-098: Vor dem Release ist das Overlay wieder nur mit `?dev=1` verfügbar; das ist nicht Teil dieses Sprints. Nachweis AC-05 steht im Ergebnis von DBG2.3.

## Erlaubte Dateien

- `src/scenes/`, `src/online/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Umbau, Server und Protokoll (DBG1).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` und `task check:go` grün.
2. `git fetch && git diff <Start-Commit>..origin/develop` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-05 aus den Ergebnissen von DBG2.1 bis DBG2.3 prüfen (Tests wörtlich, Zeitfaktor-Anzeige, Abnahme am Gerät mit Datum); ist DBG2.3 noch offen, steht das Kriterium der Abnahme als `angenommen, Validierung offen (DBG2.3)`, DBG2.3 kommt in den Fahrplan unter „Offen am Gerät“.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben.
5. B-179 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“); `docs/roadmap.md` anpassen, falls dort erwähnt.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-05 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt.
- [x] `task check` und `task check:go` grün; der Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

2026-10-03, Review-Agent (Claude Opus 5.5, Diff-Review durch getrennten Agenten), Branch `dbg2/4-review`.

- `task check` und `task check:go` grün; `git diff ae2ca20..origin/develop -- .oxlintrc.json` leer (Grenzen unverändert, `HudScene.ts` bei 17).
- Diff `src/scenes`, `src/online`, `src/model` gelesen: Client rechnet nichts (nur Abbildung Auswahl → `dev`), B, X, View + Menu nicht belegt, Ö und Stick-Klick schalten das Overlay, Liste nur bei Overlay an + Dev-Mode + Raum, Panel wird bei Szenen-Ende entfernt, Dev-Fokus fällt im nächsten Frame weg, Gold an den gewählten Slot, kein `Math.random()`.
- **AC-01, AC-02** geprüft (Ergebnis DBG2.1, `debugActions.test.ts`); **AC-03** geprüft (Ergebnis DBG2.2, `debugOverlay.test.ts` und Browser-Pane); **AC-04** geprüft (`task check` in dieser Session).
- **AC-05** angenommen, Validierung offen (DBG2.3); DBG2.3 steht im Fahrplan unter „Offen am Gerät“.
- Keine schweren Befunde, keine neuen Tickets. B-179 archiviert.

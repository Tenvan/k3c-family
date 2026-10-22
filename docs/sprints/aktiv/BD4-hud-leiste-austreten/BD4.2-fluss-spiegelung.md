# BD4.2 · Fluss mit gespiegeltem Hintergrund unter der Bodenlinie

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/bd4
- **Abhängig von:** BD4.1
- **Tickets:** B-413
- **Kriterien:** AC-03

## Ziel

AC-03 ist dokumentiert und am Code belegt; die Beobachtung am Gerät bleibt BD4.4.

## Kontext

Der Fluss steckt seit #277 in `src/scenes/render/stageView.ts` (`addRiver`: dunkles Wasser, kopfüber gespiegelte Parallax-Ebenen, Wellen als `tileSprite` mit Tween). Offene POC-Frage: Hintergrund-Höhe je Biom gegen `WORLD_TOP`; Default siehe Ticket `Entscheidung: Hintergrund-Höhe`.

## Erlaubte Dateien

- `src/scenes/render/stageView.ts`
- `src/scenes/render/` (nur ein neuer reiner Test)
- `src/scenes/render/codemap.md`

## Nicht-Ziele

Spiegelung der Figuren; neue Grafiken; Gerätebeobachtung (BD4.4).

## Schritte

1. Fluss im Browser-Pane prüfen (Dev-Server über `svc_start`).
2. Fehlt etwas an AC-03, in `stageView.ts` nachbessern; sonst Befund im Ergebnis.

## Fertig, wenn

- [ ] AC-03: Fluss mit gespiegeltem Hintergrund und Wellen im Browser sichtbar (Beobachtung am Gerät folgt BD4.4).

## Prüfen

```bash
task check:diff
```

## Ergebnis

AC-03 geprüft im Browser-Pane (2 Spieler, Dev-Build auf Port 5193): unter der Bodenlinie liegt der Fluss mit kopfüber gespiegeltem Hintergrund und Wellen in beiden Feldern. Keine Codeänderung nötig (`stageView.ts` aus #277). Beobachtung am Gerät bleibt BD4.4. Hintergrund-Höhe je Biom: Default fester `WORLD_TOP`, B-526.

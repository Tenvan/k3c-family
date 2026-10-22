# BD4.2 · Fluss mit gespiegeltem Hintergrund unter der Bodenlinie

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

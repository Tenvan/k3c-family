# PF1.1 · Split-Screen im Browser-Pane profilieren

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** pf1/1-profil
- **Abhängig von:** –
- **Tickets:** B-194
- **Kriterien:** AC-02

## Ziel

Messwerte zum Split-Screen mit zwei Spielern liegen vor (FPS, Frame-Zeit, Snapshot-Abstand; eine gegen zwei Kameras, Tag gegen Nacht), und die vermutete Ursache des Ruckelns steht in B-194.

## Kontext

- 🧑 sah am 2026-10-03 auf der Xbox (Edge 150, Server `pi-gaming`) starkes Ruckeln im Split-Screen der Testseite `testing.html` (Script `src/tools/testing.ts`). 2000 Sprites liefen dort mit 60 FPS (`docs/game-design.md` › Xbox-Messung); reine Zeichenlast ist als Ursache also unwahrscheinlich (ungeprüft).
- Kameras: `src/scenes/GameScene.ts` › `layoutCameras()` baut eine Kamera je Feld (`computeLayout` in `src/scenes/layout.ts`, Stufe je Feld aus `src/scenes/cellStages.ts`), `showOnly()` in `src/scenes/stageView.ts` blendet Layer je Kamera ein. Jede Kamera bekommt in `addNightFx()` einen ColorMatrix-Filter (WebGL); vermutet: Der Filter rendert jede Kamera über einen eigenen Framebuffer und kostet auf der Xbox doppelt.
- Weitere Kandidaten (alle vermutet): `startFollow` mit Lerp je Kamera, Zeichnen in `src/scenes/worldRenderer.ts` je Snapshot, Snapshot-Takt oder Puffer (`src/online/clientTimeline.ts`, `src/online/clientInterpolation.ts`).
- Das Debug-Overlay (`src/scenes/debugOverlay.ts`, Ansicht `debugOverlayView.ts`) zeigt schon FPS, Snapshot-Hz (Soll), „letzter Snapshot … ms“, Puffer und Latenz. Eine Frame-Zeit fehlt; falls nötig, als reine Funktion in `debugOverlay.ts` mit Test ergänzen.
- Browser-Pane: `window.game`, Frames bei verdecktem Pane manuell takten (`CLAUDE.md` › Im Browser-Pane testen), zweiter Spieler über einen gemockten Controller. Dienste über `svc_start` (k3c-dev), sonst `task dev`.
- Fallstrick: `src/scenes/GameScene.ts` hat schon 403 Zeilen; dort nichts hinzufügen, Neues in eigene Dateien.
- Die offene Frage aus B-194 (Szenario, `game.html`) blockiert nicht (Beschluss 🧑 2026-10-06); gemessen wird `game.html` mit zwei lokalen Spielern und, falls abweichend, `testing.html`.

## Erlaubte Dateien

- `src/scenes/debugOverlay.ts`, `src/scenes/debugOverlay.test.ts`, `src/scenes/debugOverlayView.ts`
- `docs/backlog/B-194-*.md`, `docs/sprints/`

## Nicht-Ziele

Optimierung (PF1.2), Messung an der Xbox (PF1.4), Netz-Latenz (N2), Atlas (GR4), Server-Leistung (B-042).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. Dev-Server starten, `game.html` im Browser-Pane mit zwei Spielern im Split-Screen öffnen.
3. Je Fall FPS und Frame-Zeit über mindestens 600 Frames messen (z. B. `game.loop.actualFps`, `game.loop.delta`, Performance-Profil): eine Kamera, zwei Kameras, jeweils Tag und Nacht; Snapshot-Abstand und Puffer aus dem Debug-Overlay ablesen.
4. Falls eine Frame-Zeit für die Messung an der Xbox fehlt: Zeile im Debug-Overlay ergänzen, mit Test in `debugOverlay.test.ts`.
5. Messwerte als Tabelle und die vermutete Hauptlast in B-194 › Notizen eintragen (Ticket-Kopf nicht ändern).

## Fertig, wenn

- [ ] AC-02: B-194 › Notizen enthält FPS und Frame-Zeit für eine/zwei Kameras bei Tag und Nacht, den Snapshot-Abstand und die vermutete Hauptlast.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

–

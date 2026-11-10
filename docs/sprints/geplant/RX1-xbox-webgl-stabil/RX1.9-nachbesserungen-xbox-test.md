# RX1.9 · Nachbesserungen aus dem ersten Xbox-Test

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** rx1/9-nachbesserungen-xbox-test
- **Abhängig von:** RX1.4
- **Tickets:** B-767
- **Kriterien:** AC-24, AC-25, AC-26, AC-27

## Ziel

Die vier Befunde des ersten Xbox-Tests sind behoben: frischer Client, Löschen in der Lobby, Pad auf allen Seiten, Stress ohne Dev-Mode (B-767).

## Kontext

Auswertung des Pi-Logs (Session „PI Diagnose-Logs“, 2026-11-09): K3C_DEV=0 am Pi, 488 `spawn` mit `forbidden`; 192.168.2.209 (Edge 150) verlor den WebGL-Kontext dreimal nach 3,8 bis 7,1 s bei 233 bis 331 Texturen (338 bis 374 MiB); Canvas danach ca. 11 FPS. Spec: B-767.

## Erlaubte Dateien

- `src/landing/fresh.ts`, `landing.ts`, `src/core/padNav.ts`, `shell.ts`, `src/scenes/lobbyPad.ts`, `LobbyScene.ts`, `src/stress/realStress.ts`, Texte, Seiten-Skripte (`padNav: false`)
- zugehörige `*.test.ts`

## Nicht-Ziele

Kein Service-Worker, keine Server-Änderung am Caching.

## Schritte

1. Frisch laden bei Versions-Abweichung, `v` im iframe (AC-24).
2. Lobby: X/Y löschen, Pad-Tasten ins Log (AC-25).
3. `padNav` in `installPageChrome`, Opt-out für Seiten mit eigener Bedienung (AC-26).
4. Stresslauf endet bei `forbidden` mit K3C_DEV-Hinweis (AC-27).

## Fertig, wenn

- [ ] AC-24 bis AC-27: Tests grün (`fresh.test.ts`, `lobbyPad.test.ts`, `padNav.test.ts`, `realStress.test.ts`).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-24 bis AC-27 umgesetzt und getestet. X löst in der Lobby mit Standard-Mapping aus (im Browser mit Mock-Pad geprüft); warum es auf der Xbox nicht ankam, bleibt offen, Y ist zusätzlich belegt und das Lobby-Log nennt die ankommenden Tasten.
- Die Pi-Ursache für „keine Spawns“ ist K3C_DEV=0 (Release-Image); am Pi `K3C_DEV=1 docker compose up -d`.
- Offen: Abnahme am Gerät (B-767/AC-05).

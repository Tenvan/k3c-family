# WZ64 · DEV · Spieltest-Aufnahme: Workbench nimmt Raummikro und Spielbild auf

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-1218, B-1219
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22

## Ausgangslage

Feedback aus Spieltests am Sofa wird nicht festgehalten. Die Workbench startet das Spiel nur aus dem Bereich Assets und kann nichts aufnehmen. ffmpeg 9.0.2 mit `ddagrab`, `h264_qsv` und `whisper` ist installiert (geprüft 2026-11-22).

## Ziel

Ralf startet einen Spieltest per Knopf in der Workbench; Sprache und Spielbild landen lokal in `playtests/<id>/`, ohne dass jemand mitschreibt.

Am Ende sichtbar: Seite Spieltest im Bereich Development mit Liste, Player und Aufräum-Knöpfen; nach einem Spiel liegen `voice.wav`, `video.mkv` und `session.json` vor.

## Beteiligte und Zielgruppen

Ralf (🧑) und Familie als Spieler; Claude setzt um; 🧑 gibt frei.

## Anforderungen

- B-1218 › Anforderungen (Seite, Ablage, Ordner-Vertrag, Aufräumen per Knopf).
- B-1219 › Anforderungen (Mikro, ffmpeg ddagrab + QSV, Spielstart mit `--playtest`).

## Nicht-Ziele

- Transkript und Auswertung (WZ65).
- Marker, REC, Fragebogen im Spiel (WZ66).
- Live-KI in jeder Form.

## Regeln und Einschränkungen

- Domäne DEV; das Spiel ändert WZ66.
- Startet erst nach WZ63, weil WZ63 `App.tsx`, `Header.tsx` und die Prompt-Knöpfe umbaut.
- Alles bleibt lokal; Schreiben nur im Repo.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/app_playtest.go`, `tools/k3c-workbench/frontend/src/playtest/**`, `tools/k3c-workbench/frontend/src/Header.tsx`, `tools/k3c-workbench/frontend/src/App.tsx`
- **Abhängig von:** WZ63
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

- B-1218 › Beispiele, B-1219 › Beispiele.

## Ausnahme- und Fehlerfälle

- B-1218 und B-1219 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Seite Spieltest listet Sessions und räumt per Knopf mit Bestätigung auf (B-1218/AC-01 bis AC-03).
- **AC-02** Spieltest starten nimmt Sprache und Bild auf und stoppt mit Spielende (B-1219/AC-01 bis AC-03).
- **AC-03** `check_run task:check:diff` grün (B-1218/AC-04, B-1219/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ64.1 | `WZ64.1-seite-ablage-aufraeumen.md` | Umsetzung | autonom | fertig |
| WZ64.2 | `WZ64.2-aufnahme-ffmpeg-spielstart.md` | Umsetzung | autonom | fertig |
| WZ64.3 | `WZ64.3-review.md` | Review | autonom | fertig |

- WZ64.1 Seite, Ablage, Aufräumen (B-1218, AC-01)
- WZ64.2 Aufnahme mit ffmpeg und Spielstart (B-1219, AC-02; braucht B-1220 aus WZ66)
- WZ64.3 Review (AC-03)

## Abnahme

2026-11-22: AC-01 (WZ64.1) und AC-02 (WZ64.2) laut Session-Ergebnissen, AC-03 (WZ64.3) geprüft: go vet, go test, tsc, Vitest 324, projectRules und atomicDesign grün; golangci-lint fehlt, check:diff nicht vollständig.
Behoben: Überbleibsel-Sperre nach Workbench-Absturz (session.json laeuft), Monitor-Index-Prüfung.
Offen: Aufnahme nicht im Header, Kill beim Stopp. Keine neuen Tickets; Abnahme-Ticket der Kette ist B-1225 (WZ65), dort auch die Probe am Gerät.

# WR2 · SRV, DEV, INF · Reste des Server- und Web-Rückbaus aufräumen

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** SRV, DEV, INF
- **Reife:** bereit
- **Tickets:** B-897, B-898, B-907
- **Start-Commit:** 6802a69d9
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle; Entscheidung B-850)

## Ausgangslage

WR1 (B-887) hat Spielserver, `engine/net`, `engine/watch`, Lasttest, TUI und Starter entfernt (Entscheidung B-850). Liegen geblieben sind Server-Teile in `engine/room`, `k3c-sim bot` mit `simexe.StartBot`, Texte und Mocks zu Spielserver und Vite in der Workbench, `testdata/protocol/` und Doku, die noch den Spielserver beschreibt (B-897, B-898).

## Ziel

Kein toter Server-Code und keine Doku mehr, die einen Spielserver oder Browser-Client beschreibt.

Am Ende sichtbar: `k3c-sim` kennt keinen Unterbefehl `bot`, `testdata/protocol/` fehlt, `docs/arbeitsweise.md` nennt weder `engine/net` noch `task start`, `task ops:backup` sichert die Spielstände des nativen Spiels.

## Beteiligte und Zielgruppen

- Arbeitet: Claude (autonom).
- Profitiert: Ralf und alle Agenten (weniger toter Code, Doku stimmt).
- Entscheidet und prüft: 🧑 Ralf (Rechte-Datei, Abnahme-Ticket B-907).

## Anforderungen

- `B-897 › Anforderungen` (Server-Reste in Code, Workbench und Testdaten).
- `B-898 › Anforderungen` (Doku, Konfiguration, Backup-Skript).

## Nicht-Ziele

- `native/k3c-game` (parallel EP1/ST2, BI2).
- `.claude/settings.json` (ändert nur 🧑, siehe B-898 › Notizen).
- Beobachter-Logik, Vorschau und Spielmetrik-Report in `engine/room` (im Zweifel behalten).
- `check:all`, `check:race`, Gen: im Abnahme-Ticket B-907.

## Regeln und Einschränkungen

- Entscheidung B-850 deckt den Rückbau; was das native Spiel nutzt, bleibt.
- Simulationen und Balancing gesperrt; je Commit `task check:diff`.

- **Bereich:** `engine/room/**`, `cmd/k3c-sim/**`, `tools/k3c-workbench/internal/**`, `tools/k3c-workbench/frontend/src/api/**`, `tools/checkdiff/**`, `testdata/protocol/**`, `scripts/**`, `docs/arbeitsweise.md`, `docs/glossar.md`, `docs/protocol.md`, `docs/rules/**`, `docs/standards/**`, `.serena/memories/**`, `.codemap/state.json`
- **Abhängig von:** WR1
- **Braucht 🧑:** –
- **Sperre:** protokoll

## Beispiele

- `k3c-sim bot --bot saver` → unbekannter Unterbefehl.
- Natives Spiel startet einen Raum → unverändert, ohne Messreihen.
- `task ops:backup -- E:/` → Kopie von `dist/k3c-game/saves`.

## Ausnahme- und Fehlerfälle

- Ein Teil von `engine/room` wird noch genutzt → bleibt, steht im Session-Ergebnis.
- Backup-Quelle fehlt → Fehler auf stderr, Exit ≠ 0.

## Akzeptanzkriterien

- **AC-01** Go-Code ohne `_test.go` in `engine/room`, `cmd/` und `tools/k3c-workbench` nennt `engine/net`, Spielserver und `k3c-server` nur als Historie mit Ticket-Verweis (`B-897/AC-01`).
- **AC-02** `k3c-sim bot`, `simexe.StartBot`/`Pool` und die nur dem Server dienenden Teile von `engine/room` (Messreihen, Reopen, Changed) sind entfernt, Behaltenes ist begründet (`B-897/AC-02`).
- **AC-03** `testdata/protocol/` ist entfernt (`B-897/AC-03`).
- **AC-04** Dienste-Infrastruktur bleibt; Port-Versatz-Texte und Frontend-Mocks nennen weder Spielserver noch Vite (`B-897/AC-04`).
- **AC-05** `docs/arbeitsweise.md` und `docs/glossar.md` nennen `task start`, `task serve`, `k3c-server` nur als Historie (`B-898/AC-01`).
- **AC-06** `scripts/backup-saves.sh` sichert die Spielstände des nativen Spiels ohne Docker (`B-898/AC-02`).
- **AC-07** `.serena/memories/*` und `.codemap/state.json` nennen keine entfernten Ordner (`B-898/AC-03`).
- **AC-08** `task check:diff` je Commit grün; `check:all` im Abnahme-Ticket B-907 (`B-897/AC-05`).

## Offene Fragen

keine (Dienste bleiben, Backup wird umgestellt; autonom entschieden, gemeldet im Bericht).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WR2.1 | `WR2.1-server-reste-code.md` | Umsetzung | autonom | fertig |
| WR2.2 | `WR2.2-workbench-dienste-mocks.md` | Umsetzung | autonom | offen |
| WR2.3 | `WR2.3-doku-backup.md` | Umsetzung | autonom | offen |
| WR2.4 | `WR2.4-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.

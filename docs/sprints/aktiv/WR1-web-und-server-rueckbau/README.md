# WR1 · SRV, INF, PLAT · Rückbau von Browser-Client, Spielserver und Online-Werkzeugen

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** SRV, INF, PLAT
- **Reife:** bereit
- **Tickets:** B-887, B-848, B-849
- **Start-Commit:** 80af0d633
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14

## Ausgangslage

Entscheidung B-850 (Ralf, 2026-11-14): kein Online-Spiel mehr, das Spiel ist nativ (Go + Ebitengine, Entscheidung 008). Im Repo liegen noch Spielserver, WebSocket-Schicht, Lasttest, TUI, Windows-Starter, der Phaser-Browser-Client und eine Präsentationsseite mit Pixel-Grafik.

## Ziel

Das Repo enthält nur noch das native Spiel, Engine, Werkzeuge (Workbench, Papierschnitt) und die Präsentationsseite mit Papierschnitt-Bildern.

Am Ende sichtbar: `task --list` ohne Server-, Web- und Starter-Tasks; CI ohne Docker- und Browser-Build; `site/` mit Papierschnitt-Bildern.

## Beteiligte und Zielgruppen

Claude (Umsetzung, autonom); 🧑 Ralf liest das Ergebnis und prüft `site/` per Sichtprüfung.

## Anforderungen

- `B-887 › Anforderungen` (WR1.1)
- `B-848 › Anforderungen` (WR1.2)
- `B-849 › Anforderungen` (WR1.3)

## Nicht-Ziele

`engine/sim`, `engine/level`, `engine/room`, `engine/store`, das native Spiel und `tools/paper` bleiben. Kein neues Seitenlayout für `site/`.

## Regeln und Einschränkungen

Sim, Balancing, Lasttest gesperrt (Beschluss 🧑 2026-11-11). Entfernt Abhängigkeiten (Go: websocket, bubbletea, fsnotify; JS: phaser).

- **Bereich:** `cmd/**`, `engine/net/**`, `engine/watch/**`, `tools/k3c-starter/**`, `tools/k3c-workbench/**`, `tools/checkdiff/**`, `src/**`, `tests/**`, `site/**`, `.github/workflows/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** deps, taskfile, exe

## Beispiele

`task check:all` nach dem Sprint grün; `task native:game:build` unverändert.

## Ausnahme- und Fehlerfälle

Ein Werkzeug nutzt noch Code eines entfernten Teils → vorher dorthin verschieben, wo es gebraucht wird, oder Ticket anlegen.

## Akzeptanzkriterien

- **AC-01** Spielserver, `engine/net`, `engine/watch`, Lasttest, TUI und Starter sind entfernt; Tasks, CI, Docker und Workbench kommen ohne sie aus (`B-887/AC-01`, `B-887/AC-03`).
- **AC-02** `src/`, Spielseiten und `phaser` sind entfernt; CI und Release bauen ohne `dist/` (`B-848/AC-01`, `B-848/AC-02`).
- **AC-03** `site/` zeigt Papierschnitt-Bilder statt Pixel-Grafik (`B-849/AC-01`).
- **AC-04** `task check:all` grün (`B-887/AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WR1.1 | `WR1.1-server-entfernen.md` | Umsetzung | autonom | fertig |
| WR1.2 | `WR1.2-browser-client-entfernen.md` | Umsetzung | autonom | offen |
| WR1.3 | `WR1.3-site-papier-bilder.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.

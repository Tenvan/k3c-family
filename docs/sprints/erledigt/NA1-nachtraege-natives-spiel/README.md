# NA1 · CLI, PAP · Nachträge natives Spiel: rote Tests, Tasten-Icons, Rig-Export, Steg-Eingabe

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI, PAP
- **Reife:** bereit
- **Tickets:** B-1004, B-1005, B-912, B-1006, B-911, B-885
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Nach EP1, MG10 und WE4 sind im nativen Spiel Nachträge offen (Stand 2026-11-15): Drei Audio-Tests lesen entfernte Browser-Quellen und sind rot (B-1004, CI auf `main` rot), `golangci-lint` meldet QF1001 im Lizenz-Generator (B-1005), `TestFlow` scheitert am Skill-Punkt von Spieler 2 (B-912), ein gehaltenes A zahlt nach der Zielwahl am Steg (B-1006), Tasten-Hinweise sind noch gemalt statt Papierschnitt (B-911) und der native Rig-Export der Bosse steht auf dem Stand vor MG10 (B-885).

## Ziel

Das native Spiel ist wieder ohne rote Tests und Lint-Befunde, die Seekarte kostet kein versehentliches Gold und die Tasten-Hinweise sind Papierschnitt. Am Ende sichtbar: `go test ./...` und `golangci-lint run ./...` in `native/k3c-game` grün, Skill-Menü mit Papier-Icons am PC.

## Beteiligte und Zielgruppen

Spielende (2 Spieler, Tastatur und Pad); Agenten CLI und PAP; Prüfer `k3c-hud`, `k3c-grafik`; 🧑 gibt die Specs frei (außer B-885, Klasse A) und nimmt die Icons am PC ab.

## Anforderungen

`B-1004 › Anforderungen`; `B-1005 › Anforderungen`; `B-912 › Anforderungen`; `B-1006 › Anforderungen`; `B-911 › Anforderungen`; `B-885 › Anforderungen`. Sprint-eigen: Die Sessions laufen in der Reihenfolge der Tabelle, rote Tests zuerst.

## Nicht-Ziele

Spielregeln, Welt- und Biom-Daten; Simulation und Balancing (gesperrt); neue Belegungen; `TestFlow` oder `rig:export:check` in die CI aufnehmen.

## Regeln und Einschränkungen

Das Spiel zeichnet nur, Regeln bleiben in `engine/sim`. Grafik nur über `assets_*`. Gen-Läufe nur in NA1.4 (Icons) und NA1.6 (Rig-Export), sonst im Abnahme-Ticket (B-742).

- **Bereich:** `native/k3c-game/audio/events_test.go`, `native/k3c-game/audio/music_test.go`, `native/k3c-game/menu/licensefile/**`, `native/k3c-game/e2e/**`, `native/k3c-game/hud/**`, `native/k3c-game/game/**`, `public/grafik/icons/**`, `data/paper.json`, `native/k3c-rig/assets/**`
- **Abhängig von:** B-962, B-894, B-720
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

`go test ./...` in `native/k3c-game` → grün, auch `TestFlow`. Spieler wählt am Steg ein Ziel und hält A → kein Gold weg. Skill-Menü am Pad → Papier-LB statt gemaltem Kasten.

## Ausnahme- und Fehlerfälle

`TestFlow` ist rot wegen eines Fehlers im Spiel → neues Ticket, NA1.2 blockiert, der Sprint läuft mit NA1.3 weiter. Rig-Export scheitert an einer Spec → Ticket, B-885 verlässt den Sprint. Icon fehlt → gemalte Glyphe.

## Akzeptanzkriterien

- **AC-01** Die Audio-Tests sind grün und lesen keine Browser-Quelle (`B-1004/AC-01`, `B-1004/AC-02`).
- **AC-02** `golangci-lint run ./menu/...` ohne Befund (`B-1005/AC-01`, `B-1005/AC-02`).
- **AC-03** `TestFlow` ist zweimal hintereinander grün, Ursache genannt (`B-912/AC-01`, `B-912/AC-02`).
- **AC-04** Gehaltenes A am Steg zahlt nach der Zielwahl erst nach dem Loslassen, je Spieler (`B-1006/AC-01`, `B-1006/AC-02`, `B-1006/AC-03`).
- **AC-05** Jeder Glyph-Schlüssel hat ein Papierschnitt-Icon (`B-911/AC-01`).
- **AC-06** Ohne Icon zeichnet das HUD die gemalte Glyphe (`B-911/AC-02`).
- **AC-07** Der Rig-Export der Bosse entspricht den Specs (`B-885/AC-01`, `B-885/AC-02`).
- **AC-08** 🧑 bestätigt die Tasten-Icons am PC (`B-911/AC-03`).

## Offene Fragen

- Tastenkappen als zwei leere Kappen plus Schrift des Spiels statt je Taste ein Icon? → Default: ja (vorläufig, 🧑 bestätigt; B-911).
- Sperre am Steg bis zum Loslassen von A? → Default: ja (vorläufig, 🧑 bestätigt; B-1006).
- `TestFlow`: Position des Skill-Punkts aus dem Frame lesen statt feste Strecke? → Default: ja (vorläufig, 🧑 bestätigt; B-912).
- Audio-Tests umbauen statt streichen? → Default: umbauen (vorläufig, 🧑 bestätigt; B-1004).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NA1.1 | `NA1.1-audio-tests-und-lint.md` | Umsetzung | autonom | fertig |
| NA1.2 | `NA1.2-testflow-skillpunkt.md` | Umsetzung | autonom | fertig |
| NA1.3 | `NA1.3-steg-gehaltenes-a.md` | Umsetzung | autonom | fertig |
| NA1.4 | `NA1.4-tasten-icons-papierschnitt.md` | Umsetzung | autonom | fertig |
| NA1.5 | `NA1.5-hud-zeichnet-tasten-icons.md` | Umsetzung | autonom | fertig |
| NA1.6 | `NA1.6-rig-export-bosse.md` | Umsetzung | autonom | verworfen |
| NA1.7 | `NA1.7-review.md` | Review | autonom | verworfen |
| NA1.8 | `NA1.8-abnahme-tasten-icons-pc.md` | Workshop | Mensch | verworfen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.

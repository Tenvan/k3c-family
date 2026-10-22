# BD3 · SRV, SIM · Raum-Pause im Couch-Raum, Schutz des pausierten Monarchen online

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** SRV, SIM
- **Reife:** bereit
- **Tickets:** B-214
- **Start-Commit:** 013dcb34
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-10-22

## Ausgangslage

Die Optionen-Szene (S5.2) hält nur den Client an; der Raum rechnet weiter. Protokoll und Server kennen keine Pause (`engine/room`, `engine/net`, `docs/protocol.md`). Regel: `docs/rules/bedienung.md` › Pause (Q01). RM1.1 wurde mit PJ3 an BED abgegeben.

## Ziel

Ein Gerät kann den Raum pausieren: Im Couch-Raum steht die Simulation wirklich still, online ist der Monarch des pausierten Geräts höchstens 60 s unverwundbar. Am Ende sichtbar: `task check:go` grün, neue Protokollversion mit Beispiel unter `testdata/protocol/`.

## Beteiligte und Zielgruppen

Spielende an einem Gerät und online; Agent in SRV; 🧑 gibt die Spec frei.

## Anforderungen

B-214 › Anforderungen. Sprint-eigen: Pause und Fortsetzen sind eine **eigene Geräte-Nachricht** (`pause`/`resume`), kein Feld in `input` (Beschluss 🧑 2026-10-13 im Chat).

## Nicht-Ziele

Optionen-Szene und Client-Anbindung (S5.2, Client sendet die Nachricht in einem späteren CLI-Sprint); Verbindungsverlust (`docs/rules/bedienung.md` § 3).

## Regeln und Einschränkungen

Domäne SRV (Client-Anbindung folgt in einem CLI-Sprint); `docs/decisions/001`; deterministisch (`engine/rng`, Frist in Ticks wie `WaitFor` in `engine/room/room.go`); 2+ Spieler je Gerät; Protokoll-Änderung mit Spiegel in `docs/protocol.md`. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Couch-Raum mit 2 Spielern sendet `pause` → Nacht und Gegner stehen; `resume` → weiter. Zwei Geräte: A sendet `pause`, B spielt weiter, der Monarch von A ist 60 s unverwundbar, danach verwundbar.

## Ausnahme- und Fehlerfälle

Pausiertes Gerät verliert die Verbindung → Pause endet, Frist laut § 3. `resume` ohne vorherige Pause → ignoriert. Zweites `pause` während der Pause → keine Verlängerung der 60 s.

## Akzeptanzkriterien

- **AC-01** Go-Test: Im Couch-Raum ändert sich der Zustand während der Pause nicht, nach dem Fortsetzen läuft er weiter (B-214/AC-01).
- **AC-02** Go-Test: Online ist der pausierte Monarch 60 s unverwundbar, danach verwundbar; andere Geräte laufen unverändert (B-214/AC-02).
- **AC-03** `docs/protocol.md` beschreibt `pause`/`resume` mit neuer Version und Beispiel unter `testdata/protocol/`.

## Offene Fragen

keine (Protokollform beschlossen: eigene Nachricht, 🧑 2026-10-13). Beim autonomen Aktivieren mit Default entschieden und zur Prüfung durch 🧑 als Tickets angelegt: B-528 (Regeldetails: Couch-Raum, Schutz, Frist), B-529 (Protokoll v8 ohne Rückmeldung im Zustand).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD3.1 | `BD3.1-protokoll-pause.md` | Umsetzung | autonom | offen |
| BD3.2 | `BD3.2-sim-schutz.md` | Umsetzung | autonom | offen |
| BD3.3 | `BD3.3-raum-pause.md` | Umsetzung | autonom | offen |
| BD3.4 | `BD3.4-review.md` | Review | autonom | offen |

Der Schutz braucht ein Laufzeit-Feld in der Simulation; dafür kam beim Aktivieren die SIM-Session BD3.2 dazu (Review damit BD3.4). BD3.1 (SRV) Protokoll (AC-03), BD3.2 (SIM) geschützter Monarch (AC-02, Teil Simulation), BD3.3 (SRV) Raum (AC-01, AC-02), BD3.4 Review (alle).

## Abnahme

–

# U4 · CLI · Debug-Overlay

- **Status:** aktiv
- **Domäne:** CLI
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-093
- **Start-Commit:** 9fbf698
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1

## Ausgangslage

Außer einer FPS-Zahl zeigt der Client nichts über Verbindung und Zustand; auf der Xbox gibt es keine Browser-Konsole (B-093).

## Ziel

Ein umschaltbares Overlay zeigt Verbindung, Snapshot-Takt, Raum, Gerät und Entitäten. Am Ende sichtbar: mit `?dev=1` Overlay per Taste ein- und ausblenden, ohne `?dev=1` unverändertes Spiel.

## Beteiligte und Zielgruppen

Entwickler und 🧑 beim Test auf Xbox und Handy.

## Anforderungen

B-093 › Anforderungen.

## Nicht-Ziele

Dev-Aktionen (B-080), Ping per neuer Protokoll-Nachricht, Aufzeichnen (B-093 › Nicht-Ziele).

## Regeln und Einschränkungen

Domäne CLI (`src/scenes`, `src/online/client*`). Nur lesend, rechnet nichts. B-Taste und View + Menu unbelegt. Datei ≤ 400 Zeilen, Funktion ≤ 60. Die Abnahme am Gerät macht nur 🧑.

## Beispiele

`game.html?dev=1`, Overlay an: Raum, Gerät, Status, Takt, Alter des Snapshots, Entitäten (B-093 › Beispiele).

## Ausnahme- und Fehlerfälle

Keine Verbindung → „verbindet…“; fehlende Werte → „–“ (B-093 › Ausnahme- und Fehlerfälle).

## Akzeptanzkriterien

- **AC-01** Zeilen-Funktion getestet (B-093/AC-01).
- **AC-02** Overlay nur mit `?dev=1`, per Tastatur und Controller umschaltbar (B-093/AC-02).
- **AC-03** FPS-Anzeige aufgegangen, `task check` grün, keine Spiel-Logik in `src/scenes` (B-093/AC-03).
- **AC-04** 🧑 hat das Overlay am Gerät abgenommen (B-093/AC-04).

## Offene Fragen

Keine. Entschieden (🧑, 2026-10-02): **F3** (Tastatur) und **Klick auf den linken Stick** (Controller) schalten um; die FPS-Anzeige gibt es nur noch im Overlay, ohne `?dev=1` entfällt sie.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| U4.1 | `U4.1-zeilen-funktion.md` | Umsetzung | autonom | fertig |
| U4.2 | `U4.2-overlay-zeichnen.md` | Umsetzung | autonom | offen |
| U4.3 | `U4.3-review.md` | Review | autonom | offen |

## Abnahme

–

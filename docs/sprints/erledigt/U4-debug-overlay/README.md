# U4 · CLI · Debug-Overlay

- **Status:** erledigt
- **Domäne:** CLI
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-093
- **Start-Commit:** 9fbf698
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-02, Chat (Ralf), Revision 1; Revision 2 (Ö statt F3, dev standardmäßig an) auf Zuruf von Ralf am 2026-10-02

## Ausgangslage

Außer einer FPS-Zahl zeigt der Client nichts über Verbindung und Zustand; auf der Xbox gibt es keine Browser-Konsole (B-093).

## Ziel

Ein umschaltbares Overlay zeigt Verbindung, Snapshot-Takt, Raum, Gerät und Entitäten. Am Ende sichtbar: in der Entwicklungsphase Overlay per Taste ein- und ausblenden (`?dev=0` schaltet es ganz ab).

## Beteiligte und Zielgruppen

Entwickler und 🧑 beim Test auf Xbox und Handy.

## Anforderungen

B-093 › Anforderungen.

## Nicht-Ziele

Dev-Aktionen (B-080), Ping per neuer Protokoll-Nachricht, Aufzeichnen (B-093 › Nicht-Ziele).

## Regeln und Einschränkungen

Domäne CLI (`src/scenes`, `src/online/client*`). Nur lesend, rechnet nichts. B-Taste und View + Menu unbelegt. Datei ≤ 400 Zeilen, Funktion ≤ 60. Die Abnahme am Gerät macht nur 🧑.

## Beispiele

`game.html`, Overlay an: Raum, Gerät, Status, Takt, Alter des Snapshots, Entitäten (B-093 › Beispiele).

## Ausnahme- und Fehlerfälle

Keine Verbindung → „verbindet…“; fehlende Werte → „–“ (B-093 › Ausnahme- und Fehlerfälle).

## Akzeptanzkriterien

- **AC-01** Zeilen-Funktion getestet (B-093/AC-01).
- **AC-02** Overlay standardmäßig verfügbar, mit `?dev=0` nicht erzeugt, per Tastatur und Controller umschaltbar (B-093/AC-02).
- **AC-03** FPS-Anzeige aufgegangen, `task check` grün, keine Spiel-Logik in `src/scenes` (B-093/AC-03).
- **AC-04** 🧑 hat das Overlay am Gerät abgenommen (B-093/AC-04).

## Offene Fragen

Keine. Entschieden (🧑, 2026-10-02): **Ö** (Tastatur; F3 ist im Browser belegt, D ist „laufen“) und **Klick auf den linken Stick** (Controller) schalten um; in der Entwicklungsphase ist das Overlay ohne Parameter verfügbar (Revision 2); die FPS-Anzeige gibt es nur noch im Overlay, ohne `?dev=1` entfällt sie.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| U4.1 | `U4.1-zeilen-funktion.md` | Umsetzung | autonom | fertig |
| U4.2 | `U4.2-overlay-zeichnen.md` | Umsetzung | autonom | fertig |
| U4.3 | `U4.3-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-02, Review U4.3 (Agent): AC-01 belegt (U4.1 › Ergebnis), AC-02 und AC-03 belegt (U4.2 › Ergebnis, Revision 2: Ö, dev standardmäßig an).
- AC-04 abgenommen: 🧑 hat das Overlay am Gerät geprüft (Chat, 2026-10-02).
- Behobene Befunde: keine (keine schweren Befunde). Neue Tickets: B-098 (dev-Standard vor dem Release zurücknehmen).

# U4 · CLI · Debug-Overlay

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-093
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Tastenbelegung und Sichtbarkeit der FPS-Anzeige ohne `?dev=1` (B-093 › Offene Fragen, entscheidet 🧑 vor der Freigabe).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- U4.1 Zeilen-Funktion aus Verbindungs- und Weltzustand mit Tests (AC-01).
- U4.2 Overlay in der HUD-Szene, Umschalten, FPS aufnehmen (AC-02, AC-03).
- U4.3 Review (alle); AC-04 ist die Abnahme durch 🧑.

## Abnahme

–

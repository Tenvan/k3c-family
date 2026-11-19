# RU54 · CLI · Smoke-Befunde: Pausenmenü sichtbar und satte Farben im Rust-Spiel

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1127, B-1128, B-1129
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Ralf hat den Rust-Smoke-Client abgenommen (sehr gut) und zwei Befunde gemeldet: Das Pausenmenü erscheint nicht, obwohl das Spiel steht (B-1127), und die Farben sind blasser als in Go (B-1128).

## Ziel

Beide Befunde sind mit belegter Ursache behoben und durch Tests gesichert; der Bildvergleich im Fenster läuft im Abnahme-Ticket B-1129 (Lane zweit, mit B-1120).

## Beteiligte und Zielgruppen

Hauptrechner setzt um; Ralf nimmt im Fenster ab; Second Worker führt den Bildvergleich (B-1129).

## Anforderungen

Go ist das Orakel; Rust-Leitplanken RU42 (keine Allokation je Bild, Ratsche nicht lockern), Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen, keine neuen Abhängigkeiten. Jede Mechanik mit 2 Spielern.

## Nicht-Ziele

Umbau der Plattform-Sortierung, neues Farbmanagement, Nacht-/Licht-Overlay (B-1056), Sim, Balancing, Gen, check:race.

## Regeln und Einschränkungen

Smoke-Abnahme durch Ralf; neue Funde als Tickets in diesem Sprint.

Planungsangaben für die Arbeitswellen, je eine Zeile:

- **Bereich:** `rust/crates/k3c-game/src/game/**`, `rust/crates/k3c-platform/src/backend/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Pause im Fenster: Karten liegen über Welt und HUD. Himmel im Fenster: gleiche Farbe wie in Go.

## Ausnahme- und Fehlerfälle

Zeigt der Bildvergleich weitere Farbabweichungen, entsteht ein Ticket in diesem Sprint.

## Akzeptanzkriterien

- **AC-01** Der oberste Screen und die Overlay-Drawer liegen über Welt und HUD (B-1127, Test).
- **AC-02** Die Fensterfläche nutzt kein sRGB-Format, wenn ein lineares angeboten wird (B-1128, Test).
- **AC-03** Ralf bestätigt Menü und Farben im Fenster (manuell, Abnahme).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU54.1 | `RU54.1-pausenmenue-ueber-welt-und-hud.md` | Umsetzung | autonom | fertig |
| RU54.2 | `RU54.2-flaeche-ohne-srgb.md` | Umsetzung | autonom | fertig |
| RU54.3 | `RU54.3-review.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

–

# EP1 · CLI · Eingabe-Profile je Spieler und Tasten-Icons je Gerät

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-893, B-894
- **Start-Commit:** 6802a69d9
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-14 (Auftrag Ralf: nächste autonome Welle)

## Ausgangslage

Das native Spiel (`native/k3c-game`) kennt zwei Tastatur-Hälften als eigene Geräte (`keys1` WASD, `keys2` Pfeile und Ziffernblock, `input/bindings.go`) und Gamepads. Zwei Spieler an einer Tastatur sind unbequem; eine Wahl des Eingabegeräts je Spieler gibt es nicht. Tasten-Hinweise (Skill-Menü, Tastenbelegung, F1) zeichnet das HUD als gemalte Glyphen (`hud/glyphs.go`), abhängig vom Gerät, nicht vom Profil.

## Ziel

Jeder Spieler wählt in Optionen › Steuerung sein Eingabe-Profil (WASD, Pfeile, Gamepad), höchstens einer die Tastatur; alle Tasten-Hinweise zeigen die Taste im Profil des jeweiligen Spielers.

**Am Ende sichtbar:** Optionen › Steuerung mit einer Zeile je Spieler; nach dem Umschalten von WASD auf Pfeile steuert die Tastatur sofort mit den Pfeilen, und das Skill-Menü zeigt Pfeil-Tasten. Go-Tests in `native/k3c-game/input`, `view`, `menu`, `hud` grün.

## Beteiligte und Zielgruppen

- Spielende am PC und an der Xbox (Couch-Koop: einer an der Tastatur, weitere am Gamepad).
- Claude setzt um (autonom).
- 🧑 Ralf prüft Bild und Spielgefühl (Abnahme-Ticket).

## Anforderungen

- `B-893 › Anforderungen` (Profile `wasd`, `pfeile`, `gamepad`; Auswahl je Spieler in Optionen › Steuerung; höchstens ein Tastatur-Spieler; UI-Tasten F1–F12 unabhängig; alte Zwei-Spieler-Tastatur entfällt).
- `B-894 › Anforderungen` (eine zentrale Funktion Aktion + Profil → Icon; Split-Screen je Hälfte; F1 zeigt Tastenkappen; fehlende Papierschnitt-Icons als PAP-Ticket).
- Sprint-eigen: Die Tastatur ist genau **ein** Gerät (`keys1`); ihre Belegung folgt dem Tastatur-Profil der Einstellungen.

## Nicht-Ziele

- Frei belegbare Einzeltasten (Rebinding), später eigenes Ticket.
- Papierschnitt-Grafik der Tasten-Icons selbst (Assets und `gen:paper`): PAP-Ticket aus EP1.2.
- Feste Zuordnung Spielernummer ↔ Pad-Nummer: Spielernummer folgt weiter dem Beitritt in der Lobby.

## Regeln und Einschränkungen

- Spiel-Code fragt Aktionen ab, nie Tasten; nur Xbox-Symbole für jedes Pad; Icons nur über `assets_*`.
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; nicht anfassen: `native/k3c-game/props/**` (BI2), `engine/room/**`, `cmd/k3c-sim/**` (WR2).

- **Bereich:** `native/k3c-game/input/**`, `native/k3c-game/view/**`, `native/k3c-game/menu/**`, `native/k3c-game/hud/**`, `native/k3c-game/game/**`, `native/k3c-game/devtools/**`, `native/k3c-game/e2e/**`, `native/k3c-game/local/settings*.go`
- **Abhängig von:** B-888
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

- Spieler 1 WASD, Spieler 2 Gamepad (Standard) → Tastatur steuert mit A/D, Leertaste, E; Pad wie bisher.
- Ralf stellt Spieler 1 auf Pfeile → ab dem nächsten Bild läuft der Monarch mit ←/→, zahlt mit Enter, das Skill-Menü zeigt Pfeil-Tasten.
- Spieler 1 hat WASD, Spieler 2 blättert sein Profil durch → er bekommt nur Gamepad, Tastatur-Profile werden übersprungen.

## Ausnahme- und Fehlerfälle

- Gespeicherte Einstellungen mit zwei Tastatur-Profilen (Hand-Edit) → beim Laden bekommt der spätere Spieler Gamepad.
- Unbekannter Profil-Wert → Standard des Platzes (Spieler 1 WASD, sonst Gamepad).
- Kein Spieler hat ein Tastatur-Profil → die Tastatur bleibt mit WASD bedienbar (Menüs, Beitritt), damit niemand ausgesperrt ist.

## Akzeptanzkriterien

- **AC-01** Jedes Profil liefert alle Spiel-Aktionen (Test gegen die Aktionsliste) (`B-893/AC-01`).
- **AC-02** Zwei Spieler können nicht beide ein Tastatur-Profil haben (Test) (`B-893/AC-02`).
- **AC-03** Umschalten im Menü wirkt sofort und bleibt nach Neustart erhalten (Test) (`B-893/AC-03`).
- **AC-04** Für jede Aktion und jedes Profil gibt es ein Icon (Test) (`B-894/AC-01`).
- **AC-05** Ein Wechsel des Profils ändert die Hinweise sofort (Test) (`B-894/AC-02`).

Sichtprüfung und Spieltest durch 🧑 an TV und PC (`B-893/AC-04`, `B-894/AC-03`) laufen im Abnahme-Ticket des Sprints.

## Offene Fragen

keine. Entschieden mit Default (autonom, Claude, 2026-11-14): Belegungstabelle schlägt Claude vor (EP1.1 › Ergebnis und `input/bindings.go`), 🧑 bestätigt bei der Abnahme; Spielernummer folgt dem Lobby-Beitritt, das Profil bestimmt die Belegung des Geräts.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| EP1.1 | `EP1.1-eingabe-profile.md` | Umsetzung | autonom | fertig |
| EP1.2 | `EP1.2-tasten-icons.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14 (autonom, Claude): AC-01 bis AC-03 siehe EP1.1 › Ergebnis, AC-04 und AC-05 siehe EP1.2 › Ergebnis. Review (k3c-reviewer, Sonnet): keine schweren Befunde. Neue Tickets: B-911 (Papierschnitt-Tasten-Icons), B-912 (`TestFlow` schon auf `main` rot). Abnahme-Ticket für Sichtprüfung und Spieltest: B-913 (Sim, Balance gesperrt; Gen und Race nicht betroffen).

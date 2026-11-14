# CH1 · CLI · Cheats, Entwickler-Tasten, Debug-Werte und HUD-Icons im nativen Spiel

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14

## Ausgangslage

Das native Spiel (`native/k3c-game`) kennt nur F2 (Leistungsanzeige) und F3 (Fluss-Fassung); Cheats, Tastenhilfe, Debug-Werte und Zusammenfassungs-Logs fehlen, das HUD nennt Ressourcen mit Namen. Die Dev-Befehle der Sim und des Raums gibt es (`engine/sim/dev*.go`, `engine/room/dev.go`).

## Ziel

Entwickler steuern und prüfen das native Spiel über F-Tasten (Hilfe, Speichern, Pause, Zeitraffer, Cheats, Debug-Overlay, Screenshot), sehen die Werte der Sim und lesen knappe Logs und einen Sitzungsbericht; das HUD zeigt Icons mit Zahl.

Am Ende sichtbar: F1 zeigt die Tastentabelle, F9 das Cheat-Menü, F10 Hitboxen und die Werte-Seite, `reports/` den Sitzungsbericht, das HUD Icons.

## Beteiligte und Zielgruppen

- 🧑 Ralf: Freigabe (erteilt 2026-11-14), Sichtprüfung.
- Claude: Umsetzung autonom.

## Anforderungen

- `B-888 › Anforderungen` (Tastentabelle, F1–F12, Cheat-Menü).
- `B-889 › Anforderungen` (Werte-Seite im F10-Overlay, Logs, Sitzungsbericht).
- `B-890 › Anforderungen` (Icons im HUD).

## Nicht-Ziele

- Simulationen, Balancing, Lasttests (gesperrt).
- Server und Browser-Client (Sprint WR1).

## Regeln und Einschränkungen

Importregel des nativen Spiels (`native/k3c-game/game/imports_test.go`): Fachpakete nur view, art, sim, data. Datei ≤ 400 Zeilen, Funktion ≤ 60, Log-Emojis laut CLAUDE.md, 2 Spieler.

- **Bereich:** `native/k3c-game/**`, `data/*.json`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Siehe Tickets B-888, B-889, B-890.

## Ausnahme- und Fehlerfälle

- Aufnahme-Modus (`--shots`): keine F-Taste wirkt, kein Cheat-Menü.
- Kein Spiel aktiv: F5–F10 tun nichts.

## Akzeptanzkriterien

- **AC-01** F1 zeigt die Tastenübersicht aus der zentralen Tabelle, jede ausgewertete F-Taste steht darin (B-888/AC-01, Go-Test).
- **AC-02** F5, F6, F7 (Pause, Einzelschritt), F8 (Zeitraffer), F11 (Vollbild), F12 (Screenshot) wirken laut Tabelle; F für Vollbild entfällt (Go-Test).
- **AC-03** F9 öffnet und schließt das Cheat-Menü je Spieler; Gold, Material, Zeitraffer, nächste Tageszeit und Nacht wirken sofort (B-888/AC-02, Go-Test).
- **AC-04** F10 zeigt Hitboxen, Chunk-Grenzen und Kamera (Go-Test der Geometrie).
- **AC-05** Die Werte-Seite des F10-Overlays zeigt Tageszeit, Spieler, Objektanzahl, Takt und ausgewähltes Objekt aus der Sim (B-889/AC-01, AC-02).
- **AC-06** Logs nur bei Wechsel bzw. als Zusammenfassung, Sitzungsbericht als JSON in `reports/` (B-889/AC-03, AC-04).
- **AC-07** Das HUD zeigt Icon + Zahl je Ressource, jede Ressource hat ein Icon (B-890/AC-01, AC-02).
- **AC-08** Sichtprüfung durch 🧑 (B-888/AC-03, B-889/AC-05, B-890/AC-03).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| CH1.1 | `CH1.1-tastentabelle-und-f-tasten.md` | Umsetzung | autonom | fertig |
| CH1.2 | `CH1.2-cheat-menue.md` | Umsetzung | autonom | fertig |
| CH1.3 | `CH1.3-debug-overlay.md` | Umsetzung | autonom | fertig |
| CH1.4 | `CH1.4-debug-werte.md` | Umsetzung | autonom | fertig |
| CH1.5 | `CH1.5-logs-und-bericht.md` | Umsetzung | autonom | fertig |
| CH1.6 | `CH1.6-hud-icons.md` | Umsetzung | autonom | fertig |
| CH1.7 | `CH1.7-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14: AC-01 bis AC-07 laut Ergebnissen CH1.1–CH1.6, Review CH1.7 (Befunde behoben bzw. dort vermerkt). AC-08 (Sichtprüfung 🧑) offen im Abnahme-Ticket B-899, ebenso `gen:paper` mit Golden und `check:race`.

# B-888 · Das native Spiel hat wieder einen Cheat-Dialog (F9) und eine Tastenhilfe der UI-Tasten (F1)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** CH1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 („Ja, alle so“)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Phaser-Client hatte einen Cheat-Dialog (`src/scenes/debug/debugActions.ts`: Gold, Material, Ausbau, Burgfall, Zeitraffer, Tageszeit, Grad, Boss, Event, Nacht). Die Sim kennt die Dev-Befehle weiter (`engine/sim/dev*.go`). Das native Spiel hat nur F2 (Leistungsanzeige) und F3 (Fluss-Fassung) in `native/k3c-game/game/devkeys.go`, ohne Hilfe und ohne Cheats.

## Ziel

Im nativen Spiel öffnet F9 den Cheat-Dialog mit den Dev-Befehlen der Sim, F1 blendet alle belegten UI-Tasten ein.

## Beteiligte und Zielgruppen

- 🧑 Ralf: entwickelt und testet das native Spiel, entscheidet die Belegung.
- Spielende: sehen die Tasten nicht, solange sie sie nicht drücken.

## Anforderungen

- Zentrale Tastentabelle (UI-/Entwickler-Tasten, keine Spieltasten); Auswertung und F1-Übersicht lesen daraus.
- F1 Tastenübersicht ein/aus · F2 Leistungsanzeige (besteht) · F3 Fluss-Fassung (besteht) · F5 Schnellspeichern · F6 Spielstand laden · F7 Pause, in der Pause erneut F7 = ein Takt Einzelschritt · F8 Zeitraffer 1×→2×→4×→8×→1× (beendet auch die Pause) · F9 Cheat-Menü ein/aus (Tastatur und Gamepad, Gruppen wie im alten Dialog `src/scenes/debug/debugActions.ts`: Vorrat, Ausbau, Zeit, Grad, Kampf; wirkt auf den öffnenden Spieler) · F10 Debug-Overlay (Hitboxen, Chunk-Grenzen, Kamera) · F11 Vollbild (Taste F für Vollbild entfällt) · F12 Screenshot nach `reports/`.
- Keine dieser Tasten im Aufnahme-Modus (`--shots`).

## Nicht-Ziele

- Debug-Werte und Logs der Sim (eigenes Ticket im Sprint CH1).
- Icons im HUD (eigenes Ticket im Sprint CH1).

## Regeln und Einschränkungen

Dev-Befehle über `engine/room/dev.go` (`Room.Dev`) und `engine/sim/dev*.go`, nicht neu schreiben. Datei ≤ 400 Zeilen, Funktion ≤ 60. Jede Mechanik mit 2 Spielern.

- **Bereich:** `native/k3c-game/**`, `engine/room/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

- Spieler 2 drückt am Pad die Cheat-Kombination bzw. F9 auf seiner Tastatur-Hälfte → Cheat-Menü für Spieler 2, Gold fällt bei Monarch 2.
- F8 dreimal → Zeitraffer 8×, die Anzeige oben nennt ihn.

## Ausnahme- und Fehlerfälle

- Kein Spiel aktiv: F5–F9 tun nichts außer einer Log-Meldung.
- F6 ohne Spielstand: Hinweis „kein Spielstand“, Spiel läuft weiter.

## Akzeptanzkriterien

- **AC-01** F1 zeigt die Tastenübersicht, erneut F1 blendet sie aus; jede ausgewertete F-Taste steht in der Übersicht (Test gegen die Tabelle).
- **AC-02** F9 öffnet und schließt das Cheat-Menü; Gold, Material, Zeitraffer, nächste Tageszeit und Nacht sofort wirken (Go-Test über die Sim-Befehle).
- **AC-03** Sichtprüfung durch 🧑.

## Offene Fragen

Keine (Tastenbelegung von 🧑 am 2026-11-14 bestätigt: „Ja, alle so“).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

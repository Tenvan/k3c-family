# B-888 · Das native Spiel hat wieder einen Cheat-Dialog (F9) und eine Tastenhilfe der UI-Tasten (F1)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Phaser-Client hatte einen Cheat-Dialog (`src/scenes/debug/debugActions.ts`: Gold, Material, Ausbau, Burgfall, Zeitraffer, Tageszeit, Grad, Boss, Event, Nacht). Die Sim kennt die Dev-Befehle weiter (`engine/sim/dev*.go`). Das native Spiel hat nur F2 (Leistungsanzeige) und F3 (Fluss-Fassung) in `native/k3c-game/game/devkeys.go`, ohne Hilfe und ohne Cheats.

## Ziel

Im nativen Spiel öffnet F9 den Cheat-Dialog mit den Dev-Befehlen der Sim, F1 blendet alle belegten UI-Tasten ein.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- F1 schaltet eine Tastenübersicht aller UI-/Entwickler-Tasten (F-Tasten, keine Spieltasten) ein und aus; die Liste kommt aus einer zentralen Tabelle, aus der auch die Auswertung liest.
- F9 schaltet ein Cheat-Menü, bedienbar mit Tastatur und Gamepad, Gruppen wie im alten Dialog (Vorrat, Ausbau, Zeit, Grad, Kampf), wirkt auf den Spieler, der es öffnet.
- Nicht im Aufnahme-Modus (`--shots`).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** F1 zeigt die Tastenübersicht, erneut F1 blendet sie aus; jede ausgewertete F-Taste steht in der Übersicht (Test gegen die Tabelle).
- **AC-02** F9 öffnet und schließt das Cheat-Menü; Gold, Material, Zeitraffer, nächste Tageszeit und Nacht sofort wirken (Go-Test über die Sim-Befehle).
- **AC-03** Sichtprüfung durch 🧑.

## Offene Fragen

- Weitere Tasten (Vorschlag Claude, Default übernehmen nach Rückmeldung Ralf): F5 Schnellspeichern, F6 Spielstand laden, F7 Pause/Einzelschritt, F8 Zeitraffer durchschalten (1×/2×/4×/8×), F10 Debug-Overlay (Hitboxen, Chunk-Grenzen, Kamera), F11 Vollbild, F12 Screenshot nach reports/.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

# B-1117 · Menü bekommt den Stress-Starter des Spiels (MenuSet und Verdrahtung)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU30
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU30.5 liefert `Game::stress_starter()` (`game/stress.rs`, `StarterLink`: erfüllt `view::stress::StressStarter`, die Schleife führt den Auftrag gleich nach dem `update` des Screens aus). Das Menü hält den Starter in `Menus::starter` (`set_stress_starter`), aber `menu::MenuSet` (Feld `shared` privat) bietet nach `menu::new_set` keinen Weg dorthin; `game/packages.rs` baut das Menü vor dem `Game`. Ohne Starter zeigt das Testing-Menü `TstNoRunner`, der Stresstest ist im Rust-Spiel nicht aus dem Menü startbar.

## Ziel

Der Starter des Spiels kommt ins Menü: `MenuSet::set_stress_starter` (menu), Weitergabe über die Rolle `Menus`/`Shared` und Aufruf nach `Game::new` (packages bzw. Client-Start), sodass das Testing-Menü den Stresstest startet.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

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

- **AC-01** `menu::MenuSet` bietet `set_stress_starter(Box<dyn StressStarter>)`; ein Test belegt, dass das Testing-Menü danach startet statt `TstNoRunner` zu zeigen.
- **AC-02** Nach `Game::new` setzt die Verdrahtung `game.stress_starter()` im Menü (Test mit `packages()` und `FakeSim` samt Dev-Rolle).
- **AC-03** `task rust:lint` grün, Dateien <= 400 Zeilen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

# RU63 · CLI · Smoke-Nachträge: HUD mit Insel-Sicht, Stress-Starter im Menü, ein Messbericht

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1117, B-1125, B-1107, B-1116
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Smoke-Abnahme und Reviews der Rust-Umstellung haben Nachträge hinterlassen: Das HUD und die Anzeige-Naht rechnen ohne Insel-Sicht (B-1125, B-1107), das Testing-Menü kennt den Stress-Starter nicht (B-1117), der Messbericht der Wiedergabe führt eine zweite Fassung (B-1116). Das Cheat-Fenster (RU56) und der Go-Rückbau (RU37, RU59) laufen parallel und werden nicht berührt.

## Ziel

Das HUD und die Anzeige rechnen mit derselben Insel-Sicht wie die Sim (Go-gleich), der Stresstest startet aus dem Testing-Menü, ein Satz Messtypen.

## Beteiligte und Zielgruppen

Hauptrechner setzt um; Ralf nimmt im Fenster ab (Dev-Bonus Lager, Endboss-Bau, Stresstest aus dem Menü).

## Anforderungen

Go ist das Orakel; RU42 (kein unwrap/expect/panic außerhalb Tests, Alloc-Ratsche nicht lockern), Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen, keine neuen Abhängigkeiten. Jede Mechanik mit 2 Spielern.

## Nicht-Ziele

Spielregeln, Protokoll, Spielstand-Format, neue Abhängigkeit (B-1118 Ressourcen-Crate), Geschichte im HUD (B-974), Cheat-Fenster, Sim, Balancing, Gen, Messläufe, check:race.

## Regeln und Einschränkungen

Planungsangaben für die Arbeitswellen, je eine Zeile:

- **Bereich:** `rust/crates/k3c-sim/src/state/**`, `rust/crates/k3c-sim/src/island/mod.rs`, `rust/crates/k3c-game/src/hud/state.rs`, `rust/crates/k3c-game/src/hud/hints.rs`, `rust/crates/k3c-game/src/props/simview.rs`, `rust/crates/k3c-game/src/menu/mod.rs`, `rust/crates/k3c-game/src/game/packages.rs`, `rust/crates/k3c-game/src/game/stress.rs`, `rust/crates/k3c-game/src/replay/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Dev-Bonus Lager: Spielerkarte zeigt das erhöhte Maximum. Testing-Menü, Stresstest: der Lauf startet.

## Ausnahme- und Fehlerfälle

Ohne Insel (Standbild, Replay, Kampagne) bleibt `None` der Fallback.

## Akzeptanzkriterien

- **AC-01** Beutel-Maximum und Tausch-Hinweis des HUD rechnen mit der Insel-Sicht (B-1125, Test).
- **AC-02** `LiveSimView` rechnet Wirtschaft, Zahlbarkeit und Lair mit der Insel-Sicht (B-1107, Test).
- **AC-03** Das Testing-Menü startet den Stresstest (B-1117, Test).
- **AC-04** `replay/` hat keine eigene Fassung der Messtypen mehr (B-1116, Test).
- **AC-05** Ralf bestätigt Dev-Bonus-Maximum, Endboss-Bau und Stresstest im Fenster (manuell, Abnahme).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU63.1 | `RU63.1-stress-starter-im-menue.md` | Umsetzung | autonom | fertig |
| RU63.2 | `RU63.2-insel-spiegel-und-hud.md` | Umsetzung | autonom | fertig |
| RU63.3 | `RU63.3-simview-mit-insel.md` | Umsetzung | autonom | fertig |
| RU63.4 | `RU63.4-replay-bericht-auf-game-measure.md` | Umsetzung | autonom | fertig |
| RU63.5 | `RU63.5-review.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.

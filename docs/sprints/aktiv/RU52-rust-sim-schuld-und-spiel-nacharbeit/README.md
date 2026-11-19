# RU52 · SIM, CLI · Rust-Schuld: Sim-Testrümpfe, Platzhalter und Spiel-Nacharbeit

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM, CLI
- **Reife:** bereit
- **Tickets:** B-1087, B-1082, B-1100, B-1099, B-1103, B-1111, B-1110
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Die Rust-Sim hat nach RU18 und RU19 leere oder ignorierte Tests (21 leere Rümpfe, 76 `ignore`), einen Platzhalter mit `unimplemented!` im Takt-Pfad (`protected_night`) und eine Allokation je Stufenschritt. Im Rust-Spiel fehlen die Dev-Rolle für Cheats und der Bruchwert-Leser für `native.json`.

## Ziel

Die Test- und Platzhalter-Schuld der Rust-Sim ist abgebaut (Port ohne Regeländerung, Go ist das Orakel) und zwei Spiel-Nacharbeiten aus RU29/RU30 sind erledigt.

## Beteiligte und Zielgruppen

Der Second Worker setzt um; 🧑 liest das Ergebnis. Review durch den Hauptrechner bei Bedarf.

## Anforderungen

Verhalten bleibt wie in Go, Golden werden nie zugunsten von Rust neu geschrieben, Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen, keine neuen Abhängigkeiten. Jede Mechanik funktioniert mit 2 Spielern.

## Nicht-Ziele

Spielregeln, Protokoll, Spielstand-Format, Go-Code. B-1106 (Room-Aktionen Lernen/Respec/Inselwahl in `k3c-room`), B-1107, B-1109/B-1113, B-1115, B-1116, B-1117, B-1118 (braucht Freigabe für Ressourcen-Crate) und B-1114 sind nicht Teil dieses Sprints (RU33, RU35, RU36 oder später).

## Regeln und Einschränkungen

Sim-Code der Rust-Umstellung (`rust/crates/k3c-sim`) liegt nicht in `engine/`; die Wurzel-Workbench bleibt unberührt. Reihenfolge laut `Abhängig von` in den Sessions (gleiche Dateien in der k3c-sim-Kette).

Planungsangaben für die Arbeitswellen, je eine Zeile:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-game/src/view/session.rs`, `rust/crates/k3c-game/src/devtools/**`, `rust/crates/k3c-data/src/native.rs`, `rust/crates/k3c-data/tests/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Platz fällt auf 0 HP: vorher Panic (`unimplemented!`), nachher wie in Go. Cheat-Menü im lokalen Spiel: Cheat löst aus statt „Cheats gehen nur im lokalen Spiel“.

## Ausnahme- und Fehlerfälle

Ein portierter Test ist rot: Ursache klären, Erwartung nie abschwächen; ist eine Spielregel betroffen, nicht umsetzen, sondern im Ergebnis unter „Für den Hauptrechner“ beschreiben.

## Akzeptanzkriterien

- **AC-01** `hit_site` panict nicht mehr, Platzhalter `protected_night` entfällt (B-1087).
- **AC-02** `Island::stage_ref` klont keinen `Grade` je Aufruf (B-1082).
- **AC-03** Keine leeren Testrümpfe in crown, portal, views, save (B-1100).
- **AC-04** Keine `unimplemented!("RU18")`-Rümpfe und keine leeren Tests in k3c-sim (B-1099).
- **AC-05** Kein `ignore` wartet auf einen Testhelfer (B-1103).
- **AC-06** Kein `ignore` trägt „Ursache ungeprüft“ (B-1103).
- **AC-07** Cheats wirken in der lokalen Sitzung, `devtools/pending.rs` ist gelöscht (B-1111).
- **AC-08** Rust liest `native.json` wie Go (B-1110).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU52.1 | `RU52.1-protected-night-platzhalter.md` | Umsetzung | autonom | offen |
| RU52.2 | `RU52.2-island-grade-ohne-klon.md` | Umsetzung | autonom | offen |
| RU52.3 | `RU52.3-leere-ruempfe-crown-portal-views.md` | Umsetzung | autonom | offen |
| RU52.4 | `RU52.4-unimplemented-ruempfe-ru18.md` | Umsetzung | autonom | offen |
| RU52.5 | `RU52.5-testhelfer-inseln.md` | Umsetzung | autonom | offen |
| RU52.6 | `RU52.6-ignore-tests-ursache.md` | Umsetzung | autonom | offen |
| RU52.7 | `RU52.7-dev-session-rolle-im-view.md` | Umsetzung | autonom | offen |
| RU52.8 | `RU52.8-native-json-bruchwerte.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.

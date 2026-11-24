# B-1271 · Vier Rust-Tests in k3c-sim sind seit WL2 rot (Standard-Welt, eingebettete Welten)

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU33
- **Projekt:** RSU
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Seit WL2 (Standard-Welt mit fünf Inseln, Welten Britannien, Nil, Sengoku eingebettet) sind in `k3c-sim` vier Tests rot: `island::takt_tests::insel_nach_1000_schritten_wie_go`, `island::worlds_tests::eingebettete_welten_gueltig`, `island::worlds_tests::standard_welt_gleich_bisherigen_inseln`, `story::mod_tests::standard_welt_erzaehlt_eine_geschichte`. Die Go-Tests zog RU36 nach, die Rust-Tests nicht. Aufgefallen in RU33.1 (Lint-Korrektur in k3c-sim). `task check:diff` fährt sie nur, wenn Dateien in `k3c-sim` geändert sind.

## Ziel

Die vier Tests geben das Verhalten nach WL2 wieder und sind grün; Golden werden nicht zugunsten von Rust geändert.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Erwartungen an die fünf-Inseln-Standard-Welt und die eingebetteten Welten anpassen oder, wo Rust von Go abweicht, den Port prüfen.

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

- **AC-01** `cargo test -p k3c-sim` ist grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

# B-1258 · e2e-Ablauf ablauf_wie_go_zwei_laeufe_gleich ist ausgesetzt, weil die Referenz aus RU32 veraltet ist

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WN4
- **Projekt:** WZN
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Test `ablauf_wie_go_zwei_laeufe_gleich` in `rust/crates/k3c-game/tests/e2e_flow.rs` ist rot, auch auf `main`. Die Referenz `testdata/rust/e2e/flow.json` stammt aus RU32 (#751), als das Go-Spiel noch Vorlage war. Seitdem hat sich das Spiel absichtlich geändert (fünf Inseln B-1215, dichterer Wald RU36.7, Truhen-Gold RU36.3, EQ1, FG1). Gemessen: Spieler 2 findet den Skill-Punkt bei Takt 16354, die Referenz erwartet 535. Die festen Takte des Ablaufs (Pause 15900, Verlassen 15912, Mauer bis 3000) passen ebenfalls nicht mehr. Der Test ist deshalb mit `#[ignore]` ausgesetzt (Begründung im Attribut).

## Ziel

Der rote Test `ablauf_wie_go_zwei_laeufe_gleich` soll wieder die heutige Spielwelt prüfen statt einer Go-Zeit-Referenz. Nutzen: Die Prüfkette wird wieder grün, ohne Verhalten zu prüfen, das es nicht mehr gibt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

1. Referenz `testdata/rust/e2e/flow.json` aus dem heutigen Rust-Stand neu aufnehmen.
2. Feste Takte des Ablaufs an den heutigen Stand anpassen.
3. Den `#[ignore]`-Marker entfernen; der Test läuft wieder in `task check:diff`.
4. Kein Go-Orakel mehr: Die neue Referenz beschreibt das heutige Verhalten, nicht die Go-Zeit.

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

- **AC-01**: `ablauf_wie_go_zwei_laeufe_gleich` läuft ohne `#[ignore]` und ist grün.
- **AC-02**: Die Referenz nennt Takte und Stationen des heutigen Stands; die Begründung steht im Commit.
- **AC-03**: Kein Spielverhalten und keine Standbilder ändern sich durch diese Arbeit.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Entstanden bei der Untersuchung des roten Tests auf `fix/alloc-ratsche` (2026-11-24). Ursache und Messung stehen in der Ausgangslage. Ausgesetzt auf Ralfs Entscheidung.

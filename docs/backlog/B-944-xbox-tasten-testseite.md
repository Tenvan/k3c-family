# B-944 · Xbox-Controller-Tasten stimmen wieder, mit Testseite in den Entwickleroptionen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** RU61
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Live-Test 🧑 2026-11-15: Die Tasten des Xbox-Controllers passen nicht mehr zu den Aktionen (vermutet seit den Eingabe-Profilen EP1).

## Ziel

Alle Pad-Tasten lösen wieder die richtige Aktion aus; eine kleine Testseite in den Entwickleroptionen zeigt live jede gedrückte Taste, Achse und die zugeordnete Aktion je Pad.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game` (Eingabe über `k3c-platform`); die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/input/**`, `rust/crates/k3c-game/src/menu/**`, `rust/crates/k3c-game/src/devtools/**`, `rust/crates/k3c-platform/src/**`
- **Abhängig von:** –
- **Braucht 🧑:** Gerät
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Testseite in den Entwickleroptionen (Pad-Tasten, Achsen, Aktion, Geräte-ID), B geht zurück.
- **AC-02** Zuordnung korrigiert, Test je Aktion.
- **AC-03** Prüfung am Xbox-Controller durch 🧑.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

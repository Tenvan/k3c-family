# B-947 · Die durchgezogenen Hintergrundmauern haben keine Lücken mehr

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Live-Test 🧑 2026-11-15: Die durchgezogenen Hintergrundmauern zeigen Lücken zwischen den Kacheln.

## Ziel

Hintergrundmauern lückenlos (Kachelbreite, Rundung, Überlappung oder Filterung prüfen).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013). Der Look im Rust-Spiel ändert sich gerade (RU55): vor Beginn prüfen, ob die Lücken in den Hintergrundmauern dort schon behoben sind.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/view/**`, `data/paper.json`
- **Abhängig von:** RU55
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ursache benannt, Test für lückenlose Kachelung.
- **AC-02** Sichtprüfung am TV durch 🧑.

## Offene Fragen

keine

## Notizen

Ralf (2026-11-20, Screenshots im Rust-Spiel): Die Lücken in den Mauern sind weiterhin sichtbar. Der Mauerzug ist nicht durchgängig, auch zwischen den Mauerstücken vor dem Hub. Umgehängt von RU61 nach LA1 (Session LA1.7, zusammen mit B-1175 und B-1176).

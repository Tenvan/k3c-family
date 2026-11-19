# B-953 · Truhen geben zehnmal mehr Gold und werfen es als Fontäne auf den Boden

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** RU60
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Hinweis 🧑 2026-11-15: Truhen geben zu wenig Gold (heute 3–6, `data/economy.json`).

## Ziel

Truhen geben zehnmal so viel Gold und werfen es als schöne Fontaine auf den Boden (Münzen im Bogen, deterministisch), statt es direkt gutzuschreiben.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Sim in `rust/crates/k3c-sim`, Anzeige in `rust/crates/k3c-game`; Go-Vorlage nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-sim/**`, `data/**`, `rust/crates/k3c-game/src/**`, `testdata/golden/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Truhenwert ×10 in `data/` (Test).
- **AC-02** Münzen fliegen als Fontaine und liegen danach aufsammelbar am Boden (Test in `engine/sim`, Anzeige im Spiel).
- **AC-03** Sichtprüfung am TV durch 🧑.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

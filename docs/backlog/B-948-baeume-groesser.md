# B-948 · Bäume erscheinen in passender Größe statt zu klein

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
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

Live-Test 🧑 2026-11-15: Bäume sind in der Skalierung zu klein.

## Ziel

Bäume in glaubwürdiger Größe zu Figuren und Gebäuden; Wert in den Daten (z. B. `data/paper.json` bzw. Natur-Werte), nicht im Code.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013). Maßstab und Wald im Rust-Spiel ändert RU55 (RU55.2, RU55.6): vor Beginn prüfen, ob die Baumgröße dort schon stimmt.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `data/paper.json`, `rust/crates/k3c-game/src/world/**`, `rust/crates/k3c-game/src/props/**`
- **Abhängig von:** RU55
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Skalierung der Bäume angehoben, Wert in den Daten.
- **AC-02** Sichtprüfung am TV durch 🧑.

## Offene Fragen

Um welchen Faktor? Default: 1,5-fach, nach Sichtprüfung justieren. (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

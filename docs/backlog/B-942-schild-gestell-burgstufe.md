# B-942 · Schilde gibt es erst ab höherer Burgstufe, gestapelt auf einem Gestell neben der Burg

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** KS7
- **Projekt:** KSY
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sichtprüfung 🧑 2026-11-15: Das Schild lässt sich sofort an der Burg bauen (KS4, `engine/sim/combat.go`), die Zahlstelle ist ein Icon.

## Ziel

Schilde gibt es erst ab einer höheren Burgausbaustufe. Gebaute Schilde liegen als Stapel auf einem Gestell neben der Burg; das Gestell erscheint erst, wenn Schilde baubar sind.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `data/**`, `public/grafik/**`, `native/k3c-game/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden
- Sim-Code im eigenen Worktree (B-925).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Mindest-Burgstufe für Schilde in `data/` (Test: darunter keine Zahlstelle).
- **AC-02** Gestell-Asset mit Stapel (Anzahl gebauter, nicht abgeholter Schilde sichtbar), Gestell nur ab der Stufe.
- **AC-03** Sichtprüfung am TV durch 🧑.

## Offene Fragen

Ab welcher Burgstufe? Default: Hub-Stufe 2. Höchstzahl Schilde auf dem Gestell? Default: 5. (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

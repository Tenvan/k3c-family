# B-953 · Truhen geben zehnmal mehr Gold und werfen es als Fontäne auf den Boden

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** WI1
- **Projekt:** WIR
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

- **Bereich:** `engine/sim/**`, `data/**`, `native/k3c-game/**`, `testdata/golden/**`
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

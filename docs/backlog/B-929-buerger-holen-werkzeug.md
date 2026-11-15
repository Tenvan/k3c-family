# B-929 · Bürger ohne Waffe oder Werkzeug holen das nächste verfügbare, nur ohne Gegner in der Nähe

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** KS2
- **Projekt:** KSY
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Bauern holen Waffen an der Werkstatt (`fetchBow`/`fetchSword`, `unit_warrior.go:116-121`) oder vom Boden (`pickup`); eine Abstandsprüfung zu Gegnern fehlt, es gilt nur die globale Gefahr `isDangerous` (Nacht oder lebende Welle).

## Ziel

Bürger ohne Waffe oder Werkzeug holen das nächste verfügbare (Händler bzw. Werkstatt oder fallen gelassen), aber nur, wenn kein Gegner innerhalb einer Distanz aus `data/` ist.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `data/**`
- **Abhängig von:** B-928
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ein Bürger nimmt das nächste Werkzeug/Waffe, Boden vor Händler bei gleicher Distanz (Test).
- **AC-02** Mit Gegner innerhalb der Distanz wartet er (Test).

## Offene Fragen

Entschieden 🧑 2026-11-15: 8 Units Gegner-Distanz, Wert in `data/troops.json`; ersetzt für diesen Fall die globale Nacht-Gefahr.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

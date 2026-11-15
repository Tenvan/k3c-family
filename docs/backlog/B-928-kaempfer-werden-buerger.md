# B-928 · Getroffene Kämpfer verlieren die Waffe und werden Bürger, keine Kaskade bis zum Landstreicher

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
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

Heute: Kampf-HP ≤ 0 → `loseLayer` (`engine/sim/unit_warrior.go:260`), Waffe fällt als Drop, Kämpfer wird Bauer; ein getroffener Bauer lässt eine Münze fallen und wird Landstreicher.

## Ziel

Kämpfer verlieren bei Treffer ihre Waffe und werden Bürger. Die Kaskade bis zum Landstreicher entfällt ganz (Entscheidung 🧑 2026-11-15).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `data/**`
- **Abhängig von:** B-926
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Getroffener Kämpfer → Bürger, Waffe als Drop (Test).
- **AC-02** Kein Treffer macht mehr einen Landstreicher (Test).

## Offene Fragen

Entschieden 🧑 2026-11-15: Getroffene Bürger ohne Waffe fliehen zur Burg, ohne Statusverlust.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

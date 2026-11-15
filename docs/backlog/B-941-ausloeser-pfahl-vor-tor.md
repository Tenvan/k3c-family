# B-941 · Der Angriffs-Auslöser steht als Horn auf einem Pfahl vor dem Tor statt schwebend dahinter

- **Domäne:** SIM
- **Typ:** Problem
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

Sichtprüfung 🧑 2026-11-15 zu KS5/KS6 (B-934, B-940): Der Auslöser erscheint als schwebendes Horn (`icons/zahlstelle-ausloeser`) 3,5 Units innen vor dem Tor (`engine/sim/combat.go`).

## Ziel

Der Auslöser ist ein Horn auf einem Pfahl, platziert vor (außerhalb) dem äußersten Tor; gleiches für das Banner-Gestell.

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
- **Größe:** S
- **Sperre:** golden
- Sim-Code im eigenen Worktree (B-925).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Zahlstelle Auslöser (und Banner) liegt außerhalb des Tors, Abstand in `data/economy.json › combat` (Test).
- **AC-02** Asset Horn auf Pfahl über `assets_*` (Objekt mit Fußpunkt am Boden) ersetzt das schwebende Icon.
- **AC-03** Sichtprüfung am TV durch 🧑.

## Offene Fragen

Gilt „vor dem Tor“ auch für das Banner? Default: ja, beide außen. (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

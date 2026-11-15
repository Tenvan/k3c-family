# B-919 · HUD zeigt einen Geldsack statt Healthbar und P1/P2

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** FB1
- **Projekt:** NAT
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Live-Test 🧑 2026-11-15: Das HUD zeigt eine Healthbar des Spielers und die Marke P1/P2.

## Ziel

Healthbar und P1/P2 entfallen; stattdessen zeigt das HUD einen Geldsack als Icon, doppelt so groß wie die Ressourcen-Icons. Die drei Größen des Geldsacks kommen aus B-922.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Geldsack-Grafik als Papierschnitt-Asset über `assets_*` (Entscheidung 005).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Abhängig von:** –
- **Braucht 🧑:** Abnahme

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** HUD ohne Healthbar und ohne P1/P2, auch im Split-Screen.
- **AC-02** Geldsack-Icon doppelt so groß wie ein Ressourcen-Icon.

## Offene Fragen

Soll der Geldsack den Münzstand zusätzlich als Zahl zeigen? (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

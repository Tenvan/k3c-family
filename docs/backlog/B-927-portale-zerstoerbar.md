# B-927 · Portale haben HP, nur ausrückende Truppen zerstören sie; ohne Portale ist die Stufe dauerhaft sicher

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

Portale sind nur X-Koordinaten ohne HP (`engine/sim/types.go:324`), Spawnpunkt (`enemies_waves.go:58`) und Fluchtziel (`enemies.go:124`). Einen Zustand „Stufe sicher“ gibt es nicht; nur geräumte Wachenlager (`GuardsCleared`).

## Ziel

Portale sind Ziele mit HP; zerstören nur ausrückende Truppen (B-932, B-933). Sind alle Portale einer Stufe weg, ist sie dauerhaft sicher: keine Wellen mehr, gespeichert im Spielstand.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `engine/room/**`, `data/**`
- **Abhängig von:** B-926
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Portal-HP je Tiefe in `data/`, Schaden nur durch Truppen im Angriff (Test).
- **AC-02** Ein angegriffenes Portal spawnt Verteidiger (Test).
- **AC-03** Ohne Portale startet die Stufe keine Welle mehr, auch nach Laden (Test).
- **AC-04** Deterministisch, 2 Spieler.

## Offene Fragen

Entschieden 🧑 2026-11-15: Solange Truppen ein Portal angreifen, spawnt es laufend Verteidiger in Wellen (Takt und Menge je Tiefe in `data/`, vorläufig). Endboss im Bau bleibt in einer sicheren Stufe angreifbar (Inselwechsel), der Miniboss entfällt mit den Wellen. Offen nur: Portal-HP-Werte (vorläufig, `provisional`).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

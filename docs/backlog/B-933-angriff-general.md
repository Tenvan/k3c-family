# B-933 · General-Angriff: Der Monarch nimmt das Banner am Tor, ein Trupp folgt ihm, Ausfälle werden nachbesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** KS4
- **Projekt:** KSY
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Wie B-932; Variante `general` je Welt.

## Ziel

Am Tor steht immer ein Angriffs-Banner. Der Monarch nimmt es auf; ihm folgt genau ein Trupp aus bis zu fünf Soldaten und fünf Bogenschützen, der Rest bleibt. Geht der Monarch hinter das äußerste Tor zurück, wird das Banner wieder aufgestellt. Fällt ein Kämpfer, ersetzt ihn ein neuer, der vom letzten Tor losgeht.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `data/**`
- **Abhängig von:** B-932
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Banner aufnehmen und am Tor zurückstellen (Test).
- **AC-02** Trupp 5+5 folgt, Rest bleibt (Test, Größen in `data/`).
- **AC-03** Ausfall wird vom letzten Tor nachbesetzt, solange Truppen da sind (Test).
- **AC-04** 2 Spieler: Verhalten mit zwei Bannern (je Seite eins?) geklärt und getestet.

## Offene Fragen

Entschieden 🧑 2026-11-15: Ein Banner je Monarch (an jedem Tor verfügbar), also ein Trupp je Monarch. Offen nur: Kann der Monarch mit Banner bauen und zahlen? Default: ja.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

# B-932 · Gruppen-Angriff: Schild macht zum Gruppenführer, Auslöser am Tor schickt alle Gruppen los

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

Truppen sind rein defensiv (`engine/sim/unit_warrior.go:218-230`, `unit_archer.go`). Ein Tor ist die äußerste Mauer einer Seite (`outerWall`, `engine/sim/common.go:45`), kein eigener Typ. Kampf-Optionen je Welt fehlen (`engine/sim/island_worlds.go:106-117`).

## Ziel

Welt-Feld `combat.mode` (`groups` Standard, `general`). Im Modus groups wird ein Soldat, der sich ein Schild von der Burg holt, Gruppenführer einer Gruppe aus ihm und fünf Soldaten (Größe je Welt). Links und rechts am Tor steht ein Auslöser, der alle Gruppen zum Angriff schickt: zum nächsten Portal oder Gegner auf dem Weg.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `data/**`
- **Abhängig von:** B-927, B-928
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `combat.mode` und `combat.groupSize` in `data/worlds/*.json`, Standard groups/5 (Test).
- **AC-02** Schild aus der Burg macht zum Gruppenführer, Gruppe füllt sich (Test).
- **AC-03** Auslöser am Tor schickt alle Gruppen los; sie kämpfen bis Portal zerstört oder Gruppe aufgerieben (Test).

## Offene Fragen

Entschieden 🧑 2026-11-15: Schild an der Burg bauen kostet 10 Gold, Auslösen kostet 2 Gold (Werte in `data/`). Schilde lassen sich nicht wieder aufsammeln; der Gruppenführer verliert sein Schild erst als Letzter seiner Gruppe. Nach Sieg oder ohne Ziel kehren die Gruppen ans Tor zurück; nur Soldaten, Bogenschützen bleiben an Mauer und Turm.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

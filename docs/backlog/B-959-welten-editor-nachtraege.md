# B-959 · Welten-Editor-Nachträge: Endboss je Insel-Thema, Hafen-Ablauf, Event- und Sites-Overrides, Gegnerarten je Welt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WE3
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Praxistest Welten-MCP (WE3, 2026-11-15): Jede Insel muss auf crystal enden (Endboss nur Tiefe 4), ein Biom nur einmal je Insel; Hafen muss vor dem Anhängen der nächsten Insel gesetzt sein (zwei Aufrufe nötig); `events` und `hub › sites` sind nicht übersteuerbar, die Fehlermeldung nennt die erlaubten Dateien nicht; Gegnerarten je Welt gehen nicht.

## Ziel

Editor und Welt-Format decken thematische Welten ab: Endboss je Insel wählbar (nicht nur crystal), Hafen im selben `world_set` setzbar, Overrides für `events` und `hub › sites`, klare Fehlermeldung mit erlaubten Dateien, Gegner-Varianten je Welt (Name, Asset, Werte aus Basisart).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `tools/k3c-workbench/**`, `engine/sim/**`, `docs/rules/welten.md`
- **Abhängig von:** B-954
- **Braucht 🧑:** –
- **Größe:** L
- **Sperre:** –
- Sim-Code im eigenen Worktree (B-925).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Je Punkt ein Test in Workbench bzw. `engine/sim/island_worlds.go`.
- **AC-02** `docs/rules/welten.md` nachgezogen.

## Offene Fragen

Zerlegt am 2026-11-15 (Planungs-Session): Dieses Ticket bleibt Sammelticket. Endboss je Insel-Thema → B-981 (Engine), B-993 (Spiel); Hafen im selben `world_set` → B-985; Overrides `events` → B-982; Overrides `hub › sites` → B-983; Fehlermeldung mit erlaubten Dateien → B-986; Gegner-Varianten je Welt → B-984 (Format), B-994 (Spiel); Editor und MCP für alle neuen Felder → B-987. Sprints WE5 und WE6.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

# B-926 · Kampfregeln stehen in docs/rules/kampf.md und ersetzen den Burgfall als Niederlage

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** KS1
- **Projekt:** KSY
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Vorgabe 🧑 2026-11-15 (Kampfsystem nach K2C): Portale angreifbar, aktiver Angriff (Gruppen oder General je Welt), Kämpfer und Monarch sterben nicht (Waffe bzw. Krone fällt), Monarch zahlt Treffer mit Gold. Heute rein defensiv (Ist-Stand: Portale ohne HP `engine/sim/types.go:324`, Monarch 100 HP mit Respawn `data/monarch.json:3`, Niederlage per Burgfall `engine/sim/island_defeat.go`).

## Ziel

Eine verbindliche Regeldatei `docs/rules/kampf.md`, auf die KS2–KS5 verweisen; `game-design.md`, `monarch.md`, `gegner.md`, `buerger.md` angeglichen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Entscheidungen aus dem Interview 🧑 2026-11-15:
- Portale: HP je Tiefe in `data/`, nur ausrückende Truppen beschädigen sie; sind alle Portale einer Stufe zerstört, kommen dort nie wieder Wellen (im Spielstand).
- Varianten je Welt `combat.mode`: `groups` (Standard) und `general`.
- Kämpfer werden bei Treffer Bürger (Waffe fällt), keine Kaskade bis Landstreicher.
- Monarch: keine HP, fester Goldbetrag je Treffer; ohne Gold fällt die Krone. Eine Krone je Monarch; jede ins Portal geraubte Krone ist Niederlage. Der Mitspieler darf eine fremde Krone zurücktragen.
- Kronenraub ersetzt den Burgfall als Niederlage; Burgfall bleibt Schaden ohne Spielende.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/**`, `docs/game-design.md`, `docs/glossar.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `docs/rules/kampf.md` beschreibt alle Punkte der Anforderungen mit Werten-Platzhaltern für `data/`.
- **AC-02** Widersprüche in `docs/game-design.md` und `docs/rules/*.md` sind angeglichen.

## Offene Fragen

Siehe Offene Fragen in B-927 bis B-933; werden im Regel-Workshop mit 🧑 entschieden.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

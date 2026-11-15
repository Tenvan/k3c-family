# B-950 · Nur Handwerker mit Hammer bauen; jedes Spiel startet mit mindestens einem Handwerker

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
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

Hinweis 🧑 2026-11-15: Handwerker sind vergessen bzw. falsch definiert. Wie in K2C sollen Bürger ohne Werkzeug sich einen Hammer holen; nur diese Handwerker bauen. Heute bauen andere Figuren (Ist-Stand ungeprüft, mit `k3c-explorer` erheben).

## Ziel

Bauen (Gebäude, Mauern, Türme, Ausbau) erledigen ausschließlich Handwerker mit Hammer. Jedes Spiel startet mit mindestens einem Handwerker, Anzahl je Welt einstellbar.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Bürger ohne Werkzeug holen einen Hammer (vom Handwerkerbedarf oder vom Boden, mit Gegner-Abstand aus B-929).
- Welt-Feld für Start-Handwerker (Standard 1) in `data/worlds/*.json`.
- Regeln in `docs/rules/buerger.md` und `materialien-gebaeude.md` angeglichen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `data/**`, `docs/rules/**`, `testdata/golden/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** golden, spielstand
- Sim-Code im eigenen Worktree (B-925); keine Sim-/Balancing-Läufe.

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ohne Handwerker wird nichts gebaut (Test).
- **AC-02** Start mit eingestellter Zahl Handwerker (Test, 2 Spieler).
- **AC-03** Bürger holt Hammer und wird Handwerker (Test).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

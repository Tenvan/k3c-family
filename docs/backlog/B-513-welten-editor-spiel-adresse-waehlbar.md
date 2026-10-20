# B-513 · Im Welten-Editor lässt sich die Spiel-Adresse wählen: interne Engine oder Server

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wunsch von 🧑 bei der Abnahme TS1.7 (2026-10-20, Chat): „Interne URL oder Server-URL auswählbar im Welten-Editor, weil man die nicht im Kopf haben kann.“ Heute öffnet „Im Spiel öffnen“ (`WorldOpenGame`, `tools/k3c-studio/backend/app_game.go`) das Spiel über die eigene Engine des Studios (Loopback, Port 8091); der Parameter `base` bleibt nur aus Kompatibilität und muss Loopback sein. Den Spielserver (Port 8080, Heimnetz) kann man nicht wählen, die Adresse muss man kennen.

## Ziel

Im Welten-Editor wählt man vor „Im Spiel öffnen“ aus einer Liste, wo das Spiel läuft: die interne Engine des Studios oder ein erkannter Spielserver (Adresse wird angezeigt, nicht eingetippt).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

- Woher kommt die Server-Adresse (Standardport 8080 auf Loopback prüfen, Eintrag aus `.mcp.json`/k3c-dev, manuell gespeichert)? Entscheidet 🧑.
- Der Server kennt „voll ausgebaut“ und Dev-Aktionen nur im Dev-Mode; reicht die Welt-Wahl, oder soll der Studio-Spielstand auch dorthin? Entscheidet 🧑.
- Spec ist Entwurf, Freigabe fehlt.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

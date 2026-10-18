# B-479 · Prüfung der Kandidaten-hub.json: geplante Chunks nach WZ8 mitprüfen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

WZ15.3 prüft eine Kandidaten-`hub.json` (`sim.ValidateHub`), aber nicht die geplanten Chunks (WZ8, `data/islands.json`), weil WZ8 beim Bau von WZ15.3 nicht im Branch lag. Das Ergebnis trägt dafür den Hinweis `NotePlannedChunksUnchecked` („geplante Chunks nicht geprüft“). Nach dem Zusammenführen von WZ8 und WZ15: geplante Chunks gegen die Kandidaten-Linien (`level.GenerateWith` mit `Options.Lines`) mitprüfen und den Hinweis entfernen. Dazu: `CreateOptions` aus WZ8.3 und WZ15.3 zusammenführen (beide legen die Struktur an, WZ15.3 mit Feld `Candidate`).

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

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

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

# B-408 · Bauplätze aller Stufen werden schon im Hub einer niedrigen Stufe angezeigt

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** W11
- **Projekt:** WRT
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-14, 🧑 im Chat (mit W11 Rev. 2)

## Ausgangslage

Bei der Sichtung von W6.2 (2026-10-11, 🧑) werden Bauplätze „von allen Stufen“ angezeigt, auch solche, die in der aktuellen Hub-Stufe oder Stufe noch nicht bezahlbar sind (`data/hub.json › sites` mit `hubLevel`, `fromDepth`, `needsDeeper`). Ob der Server sie im Zustand schickt oder der Client sie ungefiltert zeichnet (`src/scenes/render/worldRenderer.ts` › `this.sites.sync(world.sites)`), ist noch nicht geklärt.

## Ziel

Der Hub zeigt nur Bauplätze, die in der aktuellen Hub-Stufe und Stufe gelten; die übrigen erscheinen erst, wenn sie bezahlbar werden (oder bewusst abgesetzt als „gesperrt“, falls 🧑 das will).

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

Geklärt mit 🧑 (2026-10-14): Gemeint sind Plätze höherer Hub-Stufen und Linien 2–5 (Tiefen filtert die Sim schon). Der Server schickt Plätze bis Hub-Stufe + 1 mit `hubLevel` (W11.2), unbezahlte darüber nicht; die nächste Stufe zeigt W8/B-207 als gesperrt.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

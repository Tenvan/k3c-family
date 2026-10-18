# B-475 · Frage: Wie zählen feste Ereignis-Objekte in geplanten Chunks, und gilt für „kein Platz mehr“ das Minimum oder das Maximum?

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beim Prüfen geplanter Chunks (WZ8.2, `level.ValidatePlan`) waren zwei Stellen des Regeltexts `docs/rules/geplante-chunks.md` mehrdeutig; Claude hat nach Annahmen entschieden (Auftrag von Ralf, 2026-10-17):

1. Feste Truhen, Camps und Portale zählen als Entities auf die Biom-Summen (`eventChunks`, `portals.count`). Ein Ereignis-Chunk ohne Objektliste trägt sein Objekt in der Chunk-Mitte; Truhen und Camps dürfen auch als Objekt in anderen Chunk-Arten stehen (Beispiel des Regeltexts: Truhe in einer Höhle). Der Regeltext spricht von „Ereignis-Chunks“.
2. „Kein Platz mehr“ (§ 5) rechnet mit dem Maximum der Ereignis-Chunks (ungünstigster Wurf), damit kein Seed zur Laufzeit scheitert; der Regeltext nennt die Mindestzahl.

## Ziel

Ralf entscheidet beide Punkte; danach gleichen Regeltext und Code (`engine/level/planned*.go`, WZ8.3) sich an.

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

Entscheidet 🧑: (1) Zählen feste Ereignis-Objekte als Entities oder nur Chunks der Art `chest`/`recruitCamp`/`portal`? (2) Prüft „kein Platz mehr“ gegen das Maximum (Annahme, sicher) oder das Minimum der Ereignisse (Regeltext)? Vorschlag von Claude (ungeprüft): beides wie umgesetzt, Regeltext § 3 und § 5 anpassen.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

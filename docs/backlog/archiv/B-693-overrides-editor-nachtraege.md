# B-693 · Overrides-Editor: Nachträge aus dem Review WZ37.1

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ39
- **Projekt:** WZG
- **Erstellt:** 2026-11-01
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-02

## Ausgangslage

Review WZ37.1 (2026-11-01, ohne Blocker), offene Punkte:
1. `checkWorldValues` (`tools/k3c-studio/backend/app_world_values.go:172-190`): Ein bereits ungültiges Kind blockiert jedes Speichern der Eltern-Welt (fail closed, Meldung nennt das Kind). Prüfen, ob der Editor das Kind reparieren oder das Speichern der Eltern-Welt trotzdem erlauben soll.
2. Ein Kind, das `ParseWorld` nicht lädt, fehlt in `allWorldDefs` und in `Children` (Anzeige und Warnung unvollständig, `app_world_values.go:60-68`).
3. Die Anzeige „wirkt auch auf“ nennt nur direkte Kinder, die Prüfung deckt Enkel ab (`:80`, `ChildWorlds` in `WorldHeader.tsx`): transitive Nachfolger anzeigen.
4. `setKey` hängt `overrides` hinter die Inselliste und mischt Zeilenenden bei CRLF-Dateien (`:234`); Test prüft nur `Contains`.
5. Kommentar von `sim.ResolveWorldFiles` verspricht Prüfung auch bei leerer ID, tut es nicht.
6. `flattenJSON` (`:114-146`) und `listIndex` (`world_values_merge.go`) könnten bei gemischten Listen (manche Einträge mit, manche ohne `id`) verschiedene Pfade bilden; die Zeile `id` ist als Feld editierbar.
7. `PlanWorldCheck` liest bei jedem Lauf alle Welten von der Platte.

## Ziel

Die sieben Punkte sind je behoben oder bewusst dokumentiert.

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

- **AC-01** Je Punkt Test oder Eintrag in `docs/rules/welten.md`.
- **AC-02** `task check:diff` grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

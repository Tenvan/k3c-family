# B-403 · Review-Nachträge zu Sprint WZ2: Filter-Zählung, Schnellfilter, Splitter-Test und Kleinigkeiten

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die unabhängige Review WZ2.8 (2026-10-11) hat neben den sofort behobenen Befunden (H1, M2, M3, M4, B-360-Dateimeldung, Ticket-Test B-363, N2, N3, Glossar) diese Punkte offen gelassen:

- **M1** `SprintsBacklog.tsx` und `projects.ts`: In der Projekt-Ansicht verschwinden die Sprints ausgefilterter Projekte; „Sprints x von y“ zählt sie weiter mit. B-401/AC-02 sagt „ändert die Sprint-Liste nicht“. Mit 🧑 klären, ob das so gewollt ist (Sprints stehen nur in Projekten), und die Zählung anpassen.
- **M5** erledigt (🧑 2026-10-11): Die Sprint-Schnellfilter stehen im Projekt-Filter (B-401 Revision 2); „Erledigt“, „Prio hoch“, „Worktree-tauglich“ entfielen.
- **N1** `ticketnr.go`: Schlägt `isRepoRoot` fehl (Junction, `subst`) oder liefert `git worktree list` einen Fehler, fehlen Worktrees und Refs ohne Hinweis in der Antwort.
- **N4** `tables.go`: Ist das Feld `Projekt` eines Sprints leer statt `–`, springt die Fahrplan-Zeile bei jedem `plan_set` ans Tabellenende; nicht getestet.
- **N5** `check_job.go`: Jeder Job hält die volle Ausgabe (bis 20 Jobs); nur Fehlerzeilen reichten.
- **N6** `check_background_test.go`: Sleep-Schleife; mit Kanal statt Zeit warten.
- **AC-08** `Splitter.test.tsx` prüft nur statisch; Ziehen und Doppelklick sind nicht getestet, und der Test „fehlt in der Projekt-Ansicht“ ist tautologisch. Die Spec sagt „Breiten“, umgesetzt ist die Höhe des Detailbereichs (Balken waagerecht).
- **Doku** `docs/backlog/B-186-autospeichern-takt.md` und `docs/sprints/geplant/PF1-splitscreen-leistung/PF1.4-xbox-messung.md` nennen noch „Hardware-Bahn“ (§ 11.6 heißt jetzt „Hardware“).

## Ziel

Die offenen Review-Punkte zu WZ2 sind entschieden und, wo gewollt, umgesetzt oder verworfen.

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

- M1: Sollen Sprints ausgefilterter Projekte in der Projekt-Ansicht verschwinden und aus der Zählung fallen? 🧑
- Splitter: Reicht die Höhe des Detailbereichs, oder soll es zusätzlich einen senkrechten Balken zwischen den Spalten geben (Spec: „Breiten“)? 🧑

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

# B-694 · Nachträge aus dem Review WZ39/BAL10 (niedrig)

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BAL
- **Erstellt:** 2026-11-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review vor dem Commit von WZ39/BAL10 (2026-11-02), niedrige Befunde:
1. `ParseIslandSave` prüft ohne Wertesatz (`validateIslandSave(s, nil)`, `island_save_v3.go`); die volle Prüfung holt `FromIslandSaveIn` nach. Kommentar fehlt, damit niemand das Ergebnis ohne `FromIslandSaveIn` als geprüft nutzt.
2. `fullsave.go:100` prüft über `FromIslandSave` immer gegen die Standard-Welt (ungeprüft, ob Vollausbau nur Standard-Stände baut).
3. Studio `checkWorldValues`: „schon vorher ungültig“ per gleichem Fehlertext; ein zusätzlicher späterer Verstoß im Nachfolger warnt nur statt zu blockieren.
4. Studio `snapshotWorlds` sieht nur Welten aus `listWorlds` (ungeprüft, ob das alle `data/worlds/*.json` sind).
5. `assetkit.downloadRedirect` prüft Port und Userinfo nicht.

## Ziel

Die fünf Punkte sind behoben oder als Kommentar begründet.

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

- **AC-01** Je Punkt Test oder Kommentar an der Stelle.
- **AC-02** `task check:diff` grün.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

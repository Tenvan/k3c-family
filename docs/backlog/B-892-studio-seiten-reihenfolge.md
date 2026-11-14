# B-892 · Studio zeigt die Seiten in der Reihenfolge Styleguide, Katalog, Review, Sound &amp; Quellen (eine Seite), Welten-Editor, Balancing

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 (Defaults der offenen Fragen)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Der Bereich Studio der Workbench zeigt heute Review, Katalog, Styleguide, Sound, Quellen, Balancing, Welten-Editor (`tools/k3c-workbench/frontend/src/Header.tsx`, `ASSET_PAGES`). Sound und Quellen gehören inhaltlich zusammen.

## Ziel

Studio zeigt die Seiten Styleguide, Katalog, Review, Sound & Quellen, Welten-Editor, Balancing in dieser Reihenfolge.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Reihenfolge `ASSET_PAGES`: styleguide, katalog, review, sound (Sound & Quellen), welten, balancing.
- Sound und Quellen werden eine Seite „Sound & Quellen“ (zwei Reiter oder zwei Spalten), `quellen` entfällt als eigener Reiter; eine gemerkte Auswahl `quellen` führt auf die neue Seite.
- Startseite des Bereichs bleibt Review (offene Frage).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- AC-01: Die Reiter stehen in der genannten Reihenfolge (App.test.tsx).
- AC-02: Sound & Quellen zeigt beide Inhalte; Filter und Prompt-Buttons (WZ45) funktionieren weiter.
- AC-03: `task k3c-workbench:test` grün.

## Offene Fragen

- Startseite Review oder Styleguide? Default: Review bleibt Startseite.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

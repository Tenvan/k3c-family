# B-1191 · Je Schwierigkeitsgrad ein Wachssiegel als Papierschnitt-Symbol

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** DS1
- **Projekt:** PAP
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Schwierigkeit steht heute nur als Text (`data/difficulty.json`). 🧑-Idee 2026-11-21: Jeder Schwierigkeitsgrad bekommt ein Symbol, das wie ein Wachssiegel aussieht.

## Ziel

Je Schwierigkeitsgrad ein Papierschnitt-Asset „Siegel“ (Wachsklecks mit Prägung, eigene Farbe und Prägung je Grad), erkennbar auch klein im HUD.

## Beteiligte und Zielgruppen

🧑 nimmt Look und Lesbarkeit ab; Second Worker (Lane zweit, Assets nur über `assets_*`).

## Anforderungen

- Ein Asset je Grad aus `data/difficulty.json`, Gruppe `icons`, Stil wie die übrigen Papierschnitt-Icons (Entscheidung 005).
- Lesbar bei 24 px und 64 px; Farbe und Prägung unterscheiden die Grade auch ohne Farbe (Farbsehschwäche).
- Zuordnung Grad → Asset in `data/paper.json`.

## Nicht-Ziele

Anzeige im Spiel (B-1192), neue Schwierigkeitsgrade.

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

- **AC-01** Je Grad ein gerendertes Siegel-Asset, im Review der Workbench abgenommen.
- **AC-02** Zuordnung in `data/paper.json`, Test dazu grün.

## Offene Fragen

Default (autonom entscheidbar): Prägung je Grad ein Symbol (Blatt, Schwert, Totenkopf …), Wachsfarbe von grün über gelb zu rot. 🧑 ändert bei der Abnahme.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

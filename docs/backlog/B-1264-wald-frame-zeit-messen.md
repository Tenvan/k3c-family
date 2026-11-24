# B-1264 · Die Frame-Zeit des dichteren Waldes ist gemessen (Replay headless, Vorher/Nachher)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU43
- **Projekt:** RSU
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Reviewer RU36: RU36.7 hat den Wald dichter gemacht (`data/biomes/forest.json`: chunkWeights forest 12, clearing 1; Bäume je Chunk [32,42] statt [22,30]; vermutet etwa 2,1-fach). Frame-Zeit und Sim-Schritt sind nicht gemessen; die Allokationsgrenze in `alloc_je_bild` stieg von 786 auf 941.

## Ziel

Vorher/Nachher-Messung ohne Fenster: `k3c-game replay <trace> --headless --measure N --measure-frames <datei>` auf Trace mit altem und neuem Wald, dazu `alloc_je_bild`; p99 und Maximum von step und draw vergleichen (nicht den Mittelwert). Ergebnis in `docs/messung/`, bei Überschreitung der G1-Schwellen (längstes Bild ≤ 2 × Median) ein Ticket für Culling bzw. Allokationen (B-1077, B-1078). Mini-PC-Wert bleibt bei Ralf.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

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

- **AC-01** Ein Bericht in `docs/messung/` nennt p99 und Maximum von step und draw für alten und neuen Wald.
- **AC-02** Die Schwelle längstes Bild ≤ 2 × Median ist erfüllt oder ein Folgeticket liegt vor.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

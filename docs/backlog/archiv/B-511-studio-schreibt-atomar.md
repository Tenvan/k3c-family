# B-511 · Das Studio schreibt jede Datei in data/ und public/ atomar, der Server sieht nie eine halbe Datei

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ25
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit TS1.4 (B-507) beobachtet der Spielserver `data/` und `public/` und prüft nach 2 s Ruhe den neuen Stand. Die meisten Schreibwege des Studios sind atomar (`writeAtomic` in `tools/k3c-studio/backend/plan_file.go` und `tools/assetkit/register.go`); zu prüfen sind die übrigen, z. B. `assets.ImportFile` (Kopie nach `public/…/kandidaten`). Ziel: Jede Datei in beobachteten Ordnern entsteht per Temp-Datei und Umbenennen, damit Entprellen und Prüfung nie eine halbe Datei sehen.

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

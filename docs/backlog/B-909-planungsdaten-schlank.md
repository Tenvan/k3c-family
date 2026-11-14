# B-909 · Die Planungsseite lädt bei einer Änderung nur das Geänderte statt 5,5 MiB PlanningData

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ60
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Gemessen in WZ60.1 (B-895): `PlanningData` ist 5,5 MiB groß und braucht 1,9 s; `planning:changed` feuert bei laufenden Agenten alle 4–5 s. Ohne Drossel zog die Planungsseite 644 KiB/s, mit der Drossel aus B-895 (höchstens alle 15 s) noch bis zu ~370 KiB/s. Im Fenster geht jede Antwort als Skript durch den WebView2-Renderer.

## Ziel

Die Planungsseite bleibt bei laufenden Agenten aktuell, ohne je Änderung mehrere MiB zu laden.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Größe von `PlanningData` aufschlüsseln (was macht die 5,5 MiB aus).
- Nur Geändertes nachladen (Stempel je Datei oder Teil-Daten) oder die Nutzlast deutlich verkleinern.

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

- **AC-01** `PlanningData` bzw. ein Nachladen nach einer einzelnen Ticket-Änderung ist kleiner als 500 KiB (Messung im Ergebnis).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

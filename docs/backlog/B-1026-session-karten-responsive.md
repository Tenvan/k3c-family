# B-1026 · Session-Karten im Sprint nutzen die Breite: Kopfzeile einzeilig, mehrspaltig bei breitem Fenster

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WZ61
- **Projekt:** WZG
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ralf (2026-11-17, Screenshot der Planungsseite, Sprint RU3): Jede Session-Karte stapelt Haken, Status-Pill, Nummer, Titel, Pills und Icons untereinander, das sind rund 6 Zeilen je Session. Bei breitem Fenster bleibt der meiste Platz leer, und ein Sprint mit 6 Sessions füllt mehr als einen Bildschirm.

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

- **AC-01** Eine Session-Karte zeigt Haken, Status, Nummer und Titel in einer Zeile. Pills, Abhängigkeit und Icons stehen in derselben Zeile oder, wenn der Platz nicht reicht, in höchstens einer zweiten Zeile.
- **AC-02** Ab etwa 900 px Kartenbreite stehen die Sessions in mehreren Spalten (Grid mit `auto-fill`, ohne feste Spaltenzahl). Bei schmaler Breite fällt die Darstellung auf eine Spalte zurück.
- **AC-03** Auswahl, Markieren, Wiederholen und Prompt-Kopieren funktionieren wie vorher. Die Tests der Planungsseite sind grün, `tests/atomicDesign.test.ts` ist grün.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

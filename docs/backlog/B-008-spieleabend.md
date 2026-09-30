# B-008 · Familie hat einen Spieleabend gespielt und Feedback gegeben

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Spiel wurde noch nie von der Familie am TV gespielt; Balancing beruht auf Tests und Vermutungen.

## Ziel

Familie hat einen Spieleabend gespielt und Feedback gegeben. Nutzen: Balancing und Spaß lassen sich nur am TV mit echten Spielern prüfen.

## Beteiligte und Zielgruppen

Familie spielt, 🧑 leitet den Abend, der Agent protokolliert.

## Anforderungen

- Die Familie spielt einen Tag und eine Nacht am TV.
- Der Agent schreibt ein Protokoll nach `docs/playtests/`.

## Nicht-Ziele

Keine Code-Änderungen am Abend.

## Regeln und Einschränkungen

Werte stehen in `data/*.json`, Regeln in `docs/rules/` verweisen darauf; `game-design.md` bleibt die Übersicht. Jede Regel gilt für 2+ Spieler. Balancing-Änderungen nur in JSON, alles andere als Tickets. Sinnvoll nach SP08.

## Beispiele

Nacht 1 ist zu schwer → das Protokoll nennt die Stelle, die Wertänderung steht in `data/`, Wünsche an Mechaniken werden Tickets.

## Ausnahme- und Fehlerfälle

Spiel stürzt ab oder hängt → das Protokoll hält es fest, Ticket vom Typ Problem.

## Akzeptanzkriterien

- **AC-01** Ein Protokoll liegt in `docs/playtests/`.
- **AC-02** Balancing-Änderungen stehen nur in JSON, alles andere als Tickets im Backlog.

## Offene Fragen

keine

## Notizen

Sinnvoll nach SP08 (Spiel läuft wieder über den Go-Server).

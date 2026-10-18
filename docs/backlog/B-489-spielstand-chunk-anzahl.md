# B-489 · Spielstände laden Chunk-Anzahl und Planung aus den aktuellen Daten

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Befund des Reviews zu WZ17.2 (2026-10-18): Beim Laden eines Spielstands baut `savedIslandOptions` (`engine/sim/island_data.go`) die Erzeugungs-Parameter der Insel aus den **aktuellen** Daten (`CreateOptions()` mit `plannedChunks` und `chunkCounts`); gespeichert wird beides nicht. Wird später die Planung oder die Chunk-Anzahl einer Insel in `data/islands.json` geändert, baut das Laden die Stufe eines alten Spielstands neu, bei geänderter Anzahl mit anderer Länge und ohne den Längen-Wurf (der Haupt-Strom verschiebt sich). Gespeichert sind Positionen und Schlüssel (`NodesGone`/`PickupsTaken` über `key(kind, X)`, Bauplätze, Truppen): Abgebautes taucht wieder auf, Objekte und Bauplätze liegen woanders. Das widerspricht „die Länge gilt nur für neu erzeugte Stufen“ (B-485). Heute latent, weil `chunkCounts` nirgends eingecheckt ist; bei `plannedChunks` besteht dasselbe Risiko seit WZ8.

## Ziel

Ein gespeicherter Stand lädt seine Stufen mit derselben Planung und Chunk-Anzahl, mit der sie gebaut wurden; spätere Änderungen der Daten betreffen nur neu erzeugte Stufen.

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

Entscheidet 🧑: Lösungsvorschlag des Reviews: im Hub-Spielstand ein optionales Feld `chunkCount` (omitempty) speichern und beim Laden statt der Daten verwenden (fehlt es, gilt 0 = Biom-Länge, wie alte Stände gebaut wurden); dasselbe für die Planung (Hash oder Kopie der verwendeten Planung). Das ändert das Spielstand-Format (Version +1) und gehört deshalb zusammen mit der Welt-Migration in WZ18 / B-488, die den Spielstand ohnehin auf Welt-ID und Insel-ID umstellt. Soll es dort mitlaufen?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

# B-1239 · Spieltest zeigt Auswertungsdauer je Minute Aufnahme und die Zeitleiste ist kompakter

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ65
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Auftrag im Chat
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Nach dem Transkribieren zeigte die Seite keine Dauer. Kopfblock und Marker-Bilder der Zeitleiste nahmen viel Platz.

## Ziel

Dauer der Auswertung mit kleiner Statistik, gemerkt in transcript.json; Kopfdaten und Zeitleiste kompakt.

## Beteiligte und Zielgruppen

Ralf (🧑).

## Anforderungen

- transcript.json › stats: Gesamt-, Transkript- und Bilderdauer, Aufnahmelänge, Sekunden je Minute Aufnahme, Faktor, Sätze, Wörter, Bilder.
- Seite zeigt eine Zeile mit den Kennzahlen.
- Kopfdaten der Session in einer Zeile; Marker-Bild als Vorschau (44 px hoch) neben dem Marker-Text.

## Nicht-Ziele

Vergleich über mehrere Sessions.

## Regeln und Einschränkungen

Atomic Design.

## Beispiele

„Auswertung dauerte 1 min 6 s für 2 min 30 s Aufnahme · 26,4 s je Minute Aufnahme · …“

## Ausnahme- und Fehlerfälle

Ohne Aufnahmelänge entfällt die Verhältniszeile; alte Sessions ohne stats zeigen keine Zeile.

## Akzeptanzkriterien

- **AC-01** go test internal/playtest, tsc und Vitest grün.
- **AC-02** Abnahme durch Ralf: Statistik nach dem Auswerten sichtbar, Zeitleiste kompakt.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

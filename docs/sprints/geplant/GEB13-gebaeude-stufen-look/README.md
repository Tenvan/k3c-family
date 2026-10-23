# GEB13 · REG · Gebäude-Stufen in den Daten und Stufen-Look für Plantage, Camp, Heilplatz

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-569, B-568, B-570, B-571
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gebäude haben keine eigenen Stufen (B-569); Plantage, Camp und Heilplatz zeigen keinen deutlichen Stufen-Look (B-568, B-570, B-571).

## Ziel

Gebäude tragen eigene `levels` in `data/buildings.json`, die Grafik folgt der Stufe. Am Ende sichtbar: Plantage, Camp und Heilplatz am TV je Stufe unterscheidbar.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-569, B-568, B-570, B-571 › Anforderungen.

## Nicht-Ziele

Weitere Gebäude.

## Regeln und Einschränkungen

B-569 zuerst (Daten), danach die Looks. Asset-Weg über Studio und Regeln aus GEB.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `levels` je Gebäude in `data/buildings.json`, Test (B-569/AC).
- **AC-02** Plantage hat eine niedrige Pappel vor der Farm (B-568/AC).
- **AC-03** Camp und Heilplatz zeigen je Stufe einen eigenen Look (B-570/AC, B-571/AC).
- **AC-04** 🧑 nimmt die Looks am TV ab.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Stichpunkte (Reife Entwurf): GEB13.1 Stufen-Daten (AC-01); GEB13.2 Pappel (AC-02); GEB13.3 Camp, Heilplatz (AC-03); GEB13.4 Review; GEB13.5 Abnahme TV (AC-04, Mensch).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

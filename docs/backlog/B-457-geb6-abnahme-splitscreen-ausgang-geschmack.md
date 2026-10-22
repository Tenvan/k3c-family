# B-457 · GEB6: Split-Screen, Ausgang-Beschriftung und Geschmacksfragen später prüfen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** GEB10
- **Projekt:** GEB
- **Erstellt:** 2026-10-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22 (mit Sprint GEB10)

## Ausgangslage

Aus der Abnahme GEB6 (🧑, 2026-10-17): Punkt 11 (Ausgang-Beschriftung lesbar) nicht gesehen; die Beschriftung bleibt Text im Spiel, nicht in der Grafik (Mehrsprachigkeit). Punkt 12 (zwei Spieler im Split-Screen) und Punkt 14 (Geschmack: Mine-Fels, Feuerstelle, Ausgang, Kristall-Ader) prüft 🧑 später; generell „sieht schon gut aus“.

## Ziel

🧑 hat Ausgang-Beschriftung, Split-Screen und die Geschmacksfragen am laufenden Spiel geprüft oder Nachbesserungen benannt.

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

- **AC-01** Ausgang-Beschriftung („Tiefer“/„Gesperrt“) liegt lesbar über dem Ausgang.
- **AC-02** Zwei Spieler im Split-Screen: dieselben Objekte, nichts flackert oder fehlt.
- **AC-03** Geschmack: Mine-Fels, Feuerstelle, Ausgang-Platz, Kristall-Ader bestätigt oder Nachbesserung als Ticket.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

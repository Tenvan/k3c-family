# B-151 · Der Spieleabend hat einen kindgerechten Fragebogen und eine Playtest-Vorlage

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** P1
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das Spiel wurde noch nie von der Familie am TV gespielt (B-008). Ein Protokollformat und ein Fragebogen existieren nicht; `docs/playtests/` gibt es noch nicht.

## Ziel

Eine Playtest-Vorlage und ein kindgerechter Fragebogen liegen vor, sodass Spieleabende vergleichbar protokolliert werden. Nutzen: Rückmeldungen der Familie werden zu prüfbaren Zahlen und Tickets.

## Beteiligte und Zielgruppen

Familie (auch Kinder) spielt am TV; der Agent protokolliert; 🧑 legt Termin, Teilnehmer und Fragen fest.

## Anforderungen

- Vorlage `docs/playtests/vorlage.md`: Datum, Teilnehmer (Altersgruppe, Gerät), Build/Commit, Spielverlauf, Beobachtungen, Ergebnis des Spielmetrik-Reports (B-150), Eindrücke zu Verbindung und Eingabe-Latenz (B-144), Fehler (→ Tickets).
- Kindgerechter Fragebogen (kurz, Smileys oder Daumen statt Fließtext): Spaß, Verständlichkeit, Schwierigkeit, Lesbarkeit am TV, Ton, Steuerung; Fragen als Liste mit Skala.
- Auswertung: Wünsche an Mechaniken werden Tickets, Balancing-Änderungen nur in JSON (B-008/AC-02).

## Nicht-Ziele

Der Abend selbst (B-008), der Spielmetrik-Report (B-150), Umbau des Spiels am Abend.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` (Vorlagen, Prozess nur dort, keine weiteren Prozess-Dokumente); keine personenbezogenen Daten über Vornamen und Altersgruppe hinaus; Dateien ≤ 400 Zeilen.

## Beispiele

Nach dem Abend füllt der Agent die Vorlage aus; der Fragebogen zeigt „Nacht 2 zu schwer: 3 von 4 Daumen runter“ und führt zu einem Balancing-Ticket.

## Ausnahme- und Fehlerfälle

Ein Kind will nicht antworten → Frage bleibt leer, kein Zwang. Eine Frage wird nicht verstanden → im Protokoll als Problem vermerken.

## Akzeptanzkriterien

- **AC-01** `docs/playtests/vorlage.md` enthält alle oben genannten Abschnitte.
- **AC-02** Der Fragebogen hat 6–10 Fragen mit Skala, jede kindgerecht formuliert (Beobachtung: 🧑 hat sie gelesen und freigegeben).
- **AC-03** Die Vorlage nennt, wie Wünsche zu Tickets und Balancing zu JSON-Änderungen werden.

## Offene Fragen

Termin, Teilnehmer und endgültige Fragen: 🧑, `docs/fragenkatalog.md Q24`.

## Notizen

Aus Plan Phase 1 (P1). Der Abend selbst ist B-008.

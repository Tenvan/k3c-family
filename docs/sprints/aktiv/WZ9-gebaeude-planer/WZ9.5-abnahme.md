# WZ9.5 · Abnahme Gebäude-Planer durch 🧑

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz9/5-abnahme
- **Abhängig von:** WZ9.4
- **Tickets:** B-462
- **Kriterien:** AC-06

## Ziel

🧑 nimmt den Gebäude-Planer im Asset-Studio ab (AC-06).

## Kontext

Agent: Mensch. Ein Agent nimmt diese Session nicht, er darf sie nur vorbereiten. Abnahme am Gerät, entkoppelt vom Review (WZ9.4); weicht das Ergebnis von der Annahme ab, entsteht ein Ticket.

## Erlaubte Dateien

- Planungsdateien dieser Session (Status, Ergebnis)

## Nicht-Ziele

Keine Codeänderung. Das Speichern einer echten `data/hub.json` ist eine eigene REG-Entscheidung von 🧑, nicht Teil der Abnahme.

## Schritte

1. Asset-Studio starten (`bin/k3c-studio.exe`), Seite Gebäude-Planer öffnen.
2. Matrix prüfen: Hub-Stufen, Plätze, Tiefen, Linien, Tor, Händler.
3. Beispiel: Rüstkammer auf Stufe 3 setzen; Live-Vorschau zeigt sie ungespeichert in Hub-Stufe 3.
4. Speichern (auf einer Kopie bzw. mit anschließendem Zurücksetzen mit `git checkout`): Der Diff zeigt eine Zeile in `data/hub.json` und den neu erzeugten Block in § 3; der REG-Hinweis erscheint. „Regeltabelle veraltet“ erscheint nur, wenn das Schreiben des Blocks scheitert.
5. **Pflichtplätze bestätigen (Abnahmepunkt aus WZ15.3):** die Liste im Ergebnis von WZ15.3 lesen (heute: Werkstatt, Treppe runter, Treppe hoch, Farm je Seite). Versuch, einen Pflichtplatz zu entfernen, wird abgelehnt; einen optionalen (z. B. Heilplatz) zu entfernen geht mit Hinweis. Ändert 🧑 die Liste, entsteht ein Ticket.
6. Ergebnis eintragen und `Status: fertig` setzen.

## Fertig, wenn

- [ ] AC-06: 🧑 hat Matrix, Rüstkammer-Beispiel, Live-Vorschau ungespeichert und REG-Hinweis im Asset-Studio abgenommen (Ergebnis nennt, wer geprüft hat).
- [ ] Pflichtplätze: 🧑 hat die Liste aus WZ15.3 bestätigt oder geändert (Ergebnis nennt die Liste; Änderung → Ticket).

## Prüfen

Manuell durch 🧑 im Asset-Studio, siehe Schritte. Kein automatischer Befehl.

## Ergebnis

AC-06 geprüft: 🧑 (Ralf) hat am 2026-10-19 im Asset-Studio Matrix, Rüstkammer-Beispiel (Stufe 3), Live-Vorschau ungespeichert, Speichern mit erzeugtem Block und REG-Hinweis abgenommen; alles wie erwartet.
Pflichtplätze geprüft: 🧑 bestätigt die Liste aus WZ15.3 unverändert (Werkstatt, Treppe runter, Treppe hoch, Farm je Seite; Kaserne, Taverne, Lager, Heilplatz, Schmiede, Rüstkammer entfernbar).
Abweichungen vom Plan: keine. Wunsch aus der Abnahme (Bedienbarkeit): Live-Vorschau unten fixieren, Matrix scrollbar, gewähltes Gebäude in der Vorschau zeigen (blauer Rahmen, Namen über Gebäuden) → neues Ticket.

# WZ9.8 · Abnahme Gebäude-Planer durch 🧑

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz9/8-abnahme
- **Abhängig von:** WZ9.7
- **Tickets:** B-462
- **Kriterien:** AC-06

## Ziel

🧑 nimmt den Gebäude-Planer im Asset-Studio ab (AC-06).

## Kontext

Agent: Mensch. Ein Agent nimmt diese Session nicht, er darf sie nur vorbereiten. Abnahme am Gerät, entkoppelt vom Review (WZ9.7); weicht das Ergebnis von der Annahme ab, entsteht ein Ticket.

## Erlaubte Dateien

- Planungsdateien dieser Session (Status, Ergebnis)

## Nicht-Ziele

Keine Codeänderung. Das Speichern einer echten `data/hub.json` ist eine eigene REG-Entscheidung von 🧑, nicht Teil der Abnahme.

## Schritte

1. Asset-Studio starten (`bin/k3c-studio.exe`), Seite Gebäude-Planer öffnen.
2. Matrix prüfen: Hub-Stufen, Plätze, Tiefen, Linien, Tor, Händler.
3. Beispiel: Rüstkammer auf Stufe 3 setzen; Live-Vorschau zeigt sie ungespeichert in Hub-Stufe 3.
4. Speichern auf einer Kopie bzw. mit anschließendem Zurücksetzen: Diff nur in einer Zeile, REG-Hinweis und „Regeltabelle veraltet“ erscheinen (bzw. der Block ist mit erzeugt).
5. **Pflichtplätze bestätigen (Abnahmepunkt aus WZ9.3):** die Liste im Ergebnis von WZ9.3 lesen (heute: Werkstatt, Treppe runter, Treppe hoch, Farm je Seite). Versuch, einen Pflichtplatz zu entfernen, wird abgelehnt; einen optionalen (z. B. Heilplatz) zu entfernen geht mit Hinweis. Ändert 🧑 die Liste, entsteht ein Ticket.
6. Ergebnis eintragen und `Status: fertig` setzen.

## Fertig, wenn

- [ ] AC-06: 🧑 hat Matrix, Rüstkammer-Beispiel, Live-Vorschau ungespeichert und REG-Hinweis im Asset-Studio abgenommen (Ergebnis nennt, wer geprüft hat).
- [ ] Pflichtplätze: 🧑 hat die Liste aus WZ9.3 bestätigt oder geändert (Ergebnis nennt die Liste; Änderung → Ticket).

## Prüfen

Manuell durch 🧑 im Asset-Studio, siehe Schritte. Kein automatischer Befehl.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

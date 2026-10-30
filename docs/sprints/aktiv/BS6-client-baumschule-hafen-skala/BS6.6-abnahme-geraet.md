# BS6.6 · Abnahme am Gerät

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** bs6/6-abnahme-geraet
- **Abhängig von:** BS6.5
- **Tickets:** B-636, B-637
- **Kriterien:** AC-05

## Ziel

Ralf prüft AC-05 am Zielgerät (Edge auf der Xbox) und hält das Ergebnis je Punkt fest. Danach ist BS6 abnahmefähig.

## Kontext

Stand: BS6.1 bis BS6.5 sind fertig. Der Client zeigt die Baumschule mit dem Hinweis „erst roden“ (B-636) und Hafen und Schiff je Abschnitt (B-637). Die Kosten-Anzeige nutzt die Skala 2–30 (AC-03). Der Review BS6.5 hat den Bot-Fix für Rumpf und Segel bereits eingebaut (9b678135). Fallstrick: Der Bot betritt die Hafen-Stufe nicht sicher; Segel ist nie bezahlbar (ungeprüft, Lauf offen). Prüfung nur mit dem Server-Stand, den Ralf testen will.

## Erlaubte Dateien

Keine Code-Änderung. Geändert werden nur der Abschnitt Ergebnis dieser Datei und der Status der Tickets B-636 und B-637, nach Ralfs Abnahme.

## Nicht-Ziele

Kein Balancing, keine Bot-Verbesserung, keine Code-Änderung. Claude setzt keinen Abnahme-Vermerk für AC-05; das bleibt bei Ralf.

## Schritte

1. Ralf: Server mit dem zu prüfenden Stand starten.
2. Ralf: Spiel auf dem Zielgerät öffnen, Baumschule finden, Hinweis „erst roden“ prüfen.
3. Ralf: Hafen und Schiff je Abschnitt prüfen, Hinweis bei fehlender Bedingung prüfen.
4. Ralf: Kosten-Anzeige und HUD auf Skala 2–30 prüfen.
5. Ralf: Ergebnis je Punkt nennen; Claude trägt es in den Abschnitt Ergebnis ein.

## Fertig, wenn

- [ ] AC-01: Baumschule zeigt „erst roden“, „Farm“ ist in der Oberfläche nicht mehr zu sehen.
- [ ] AC-02: Hafen und Schiff je Abschnitt, Hinweis bei fehlender Bedingung sichtbar.
- [ ] AC-03: Kosten-Anzeige und HUD passen zur Skala 2–30.
- [ ] AC-05: Ralf hat die Abnahme am Gerät im Abschnitt Ergebnis festgehalten.

## Prüfen

Keine Befehle nötig. Manuelle Prüfung am Gerät durch Ralf, nicht durch Claude. Kein Commit durch diese Session.

## Ergebnis

- AC-01 geprüft: Ralf am Gerät, erfüllt (Baumschule mit Hinweis „erst roden“, „Farm“ nicht sichtbar).
- AC-02 geprüft: Ralf am Gerät, erfüllt (Hafen und Schiff je Abschnitt, Hinweis bei fehlender Bedingung).
- AC-03 geprüft: Ralf am Gerät, erfüllt (Kosten-Anzeige und HUD passen zur Skala 2–30).
- AC-05 geprüft: Abnahme am Gerät durch Ralf abgeschlossen.
- Manuell geprüft: Ralf.
- Abweichungen vom Plan: keine von Ralf genannt.
- Neue Tickets: keine von Ralf genannt.

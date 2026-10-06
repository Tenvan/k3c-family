# BR2.2 · Spieleabend 3 mit Protokoll

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Umgebung:** live
- **Branch:** br2/2-spieleabend-3
- **Abhängig von:** –
- **Tickets:** B-156
- **Kriterien:** AC-03

## Ziel

Die Familie hat Spieleabend 3 gespielt; das Protokoll liegt nach der Vorlage in `docs/playtests/` und prüft die Kampf-, Gegner- und Bosswerte.

## Kontext

Ein Agent nimmt die Session nicht an; er darf vorbereiten und mitschreiben, wenn 🧑 ihn dazu bittet. **Termin und Teilnehmer nennt 🧑 (Q24, `docs/fragenkatalog.md`)**; sie stehen nicht in dieser Datei und werden nicht vom Agenten festgelegt.

- **Entsteht in P1:** `docs/playtests/vorlage.md` und der kindgerechte Fragebogen (Q24: 8 Fragen, Daumen hoch/runter plus ein Satz); Protokolle früherer Abende liegen im selben Ordner (P1, BR1).
- **Spielmetrik-Report:** `reports/session-*.json` je Sitzung (S2, B-150, Schema 1 in `docs/protocol.md` › „Spielmetrik-Report“): Tod durch was, Nächte überlebt, Gold je Tag, Verbindungsabbrüche. Der Agent sammelt sie nach dem Abend ein und legt die Zusammenfassung ins Protokoll (nur Geräte-Kürzel, keine Namen).
- **Build-Stand:** Gegner-Traits, Bosse, Siege, Events und Kampfprotokoll aus K1 bis K5 mit den Werten aus BR2.1 (noch unverändert in `data/`). Der Abend nennt den Schwierigkeitsgrad je Sitzung im Protokoll (`data/difficulty.json`). Hardware-Regel (`docs/arbeitsweise.md` › Hardware entkoppelt): Der Abend ist keine Abhängigkeit einer App-Session; fehlt das Gerät, bleibt die Session offen und wird mit dem Gerät nachgeholt.
- **Regel:** Keine Code-Änderungen am Abend. Balancing-Änderungen nur in JSON (später in BR2.3), Wünsche an Mechaniken als Tickets (B-008). Absturz oder Netzausfall werden festgehalten; Wiederholen entscheidet 🧑.

## Erlaubte Dateien

- `docs/playtests/` (neues Protokoll nach `docs/playtests/vorlage.md`; Dateiname vorschlagsweise `<Datum>-spieleabend-3.md`)
- `docs/sprints/` (Status dieser Session), `docs/backlog/` (Status und neue Tickets)

## Nicht-Ziele

Wertänderungen in `data/` (BR2.3), Code-Änderungen, Auswertung und Pass/Fail (BR2.3), Wirtschaft (BR1), Release (RL1).

## Schritte

1. 🧑 nennt Termin und Teilnehmer; der Agent bereitet das Protokoll nach der Vorlage vor und trägt den Build-Commit und den Grad ein.
2. Abend spielen (Tag und Nacht am TV, mit Kämpfen und, sofern erreichbar, einem Boss); Beobachtungen zu Kampf (Schwierigkeit der Nächte, Gegnerarten, Bossdauer) mitschreiben, Fragebogen ausfüllen lassen (ein Kind ohne Antwort: Frage bleibt leer).
3. Reports aus `reports/` einsammeln, Zusammenfassung ins Protokoll.
4. Fehler als Tickets (Typ Problem) anlegen, Wünsche als Tickets.

## Fertig, wenn

- [ ] AC-03: Das Protokoll von Spieleabend 3 liegt in `docs/playtests/` nach der Vorlage, mit Datum, Teilnehmern, Grad und Spielmetrik-Report.
- [ ] Fehler und Wünsche sind Tickets in `docs/backlog/`.

## Prüfen

Manuell durch 🧑 (Abend am TV); danach `task check`.

## Ergebnis

–

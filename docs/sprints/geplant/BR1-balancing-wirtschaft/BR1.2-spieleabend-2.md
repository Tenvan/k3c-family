# BR1.2 · Spieleabend 2 mit Protokoll

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** br1/2-spieleabend-2
- **Abhängig von:** –
- **Tickets:** B-155, B-015
- **Kriterien:** AC-03

## Ziel

Die Familie hat Spieleabend 2 gespielt; das Protokoll liegt nach der Vorlage in `docs/playtests/` und prüft die Wirtschaftswerte.

## Kontext

Ein Agent nimmt die Session nicht an; er darf vorbereiten und mitschreiben, wenn 🧑 ihn dazu bittet. **Termin und Teilnehmer nennt 🧑 (Q24, `docs/fragenkatalog.md`)**; sie stehen nicht in dieser Datei und werden nicht vom Agenten festgelegt.

- **Entsteht in P1:** `docs/playtests/vorlage.md` (Datum, Teilnehmer nach Altersgruppe und Gerät, Build/Commit, Spielverlauf, Beobachtungen, Spielmetrik-Report, Verbindung und Latenz, Fehler) und der kindgerechte Fragebogen (Q24: 8 Fragen, Daumen hoch/runter plus ein Satz); Protokoll des Abends 1 liegt ebenfalls dort (`P1.1` bis `P1.3`).
- **Spielmetrik-Report:** `reports/session-*.json` je Sitzung (S2, B-150, Schema 1 in `docs/protocol.md` › „Spielmetrik-Report“); der Agent sammelt sie nach dem Abend ein und legt die Zusammenfassung ins Protokoll (nur Geräte-Kürzel, keine Namen).
- **Build-Stand:** Hub-Ausbau, Plantage, Gebäude und Bürger-Wiederbeleben aus W1 bis W6 und die Werte aus BR1.1 (noch unverändert in `data/`). Hardware-Regel (`docs/arbeitsweise.md` › Hardware entkoppelt): Der Abend ist keine Abhängigkeit einer App-Session; fehlt das Gerät, bleibt die Session offen und wird mit dem Gerät nachgeholt.
- **Regel:** Keine Code-Änderungen am Abend. Balancing-Änderungen nur in JSON (später in BR1.3), Wünsche an Mechaniken als Tickets (B-008). Ein Abbruch durch Absturz oder Netzausfall wird im Protokoll festgehalten; Wiederholen entscheidet 🧑.

## Erlaubte Dateien

- `docs/playtests/` (neues Protokoll nach `docs/playtests/vorlage.md`; Dateiname vorschlagsweise `<Datum>-spieleabend-2.md`)
- `docs/sprints/` (Status dieser Session), `docs/backlog/` (Status und neue Tickets)

## Nicht-Ziele

Wertänderungen in `data/` (BR1.3), Code-Änderungen, Auswertung und Pass/Fail (BR1.3), Kampf und Bosse (BR2).

## Schritte

1. 🧑 nennt Termin und Teilnehmer; der Agent bereitet das Protokoll nach der Vorlage vor und trägt den Build-Commit ein.
2. Abend spielen (Tag und Nacht am TV); Beobachtungen zu Wirtschaft (Gold, Bau, Hub-Ausbau, Material) mitschreiben, Fragebogen ausfüllen lassen (ein Kind ohne Antwort: Frage bleibt leer).
3. Reports aus `reports/` einsammeln, Zusammenfassung ins Protokoll.
4. Fehler als Tickets (Typ Problem) anlegen, Wünsche als Tickets.

## Fertig, wenn

- [ ] AC-03: Das Protokoll von Spieleabend 2 liegt in `docs/playtests/` nach der Vorlage, mit Datum, Teilnehmern und Spielmetrik-Report.
- [ ] Fehler und Wünsche sind Tickets in `docs/backlog/`.

## Prüfen

Manuell durch 🧑 (Abend am TV); danach `task check`.

## Ergebnis

–

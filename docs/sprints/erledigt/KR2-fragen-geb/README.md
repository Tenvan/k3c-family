# KR2 · REG · Kritische Fragen zu GEB

- **Status:** erledigt
- **Projekt:** KRT
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-538, B-539, B-540, B-541, B-542, B-543, B-544, B-545
- **Start-Commit:** df21c2c2
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Das Projekt GEB (Gebäude-Grafik) wurde in GEB9 bis GEB11 autonom umgesetzt; GEB12 hat die dabei getroffenen Entscheidungen D1–D8 mit Default festgelegt (D8 umgesetzt in GEB12.3). Offen bei Ralf sind außerdem die Abnahmen GEB9.6 (Stufenreihe in Vorschau und Spiel), GEB10.5 (Natur, Truhe, Wimpel mit zwei Spielern, B-457) und GEB11.6 (Wachstumsreihe); sie bleiben in ihren Sprints.

## Ziel

Ralf hat alle kritischen Fragen zu GEB an einer Stelle entschieden. Am Ende sichtbar: kein offenes Frage-Ticket im Projekt GEB, Folgearbeit (B-531, B-532 oder neue Tickets) mit klarer Entscheidung.

## Beteiligte und Zielgruppen

Entscheidet: 🧑 Ralf. Vorbereitet: Claude. Betroffen: Spielende am TV (Hub, Wald, Wimpel, Truhe).

## Anforderungen

Je Ticket `B-5NN › Notizen` (Frage, Default, Alternative, Folgen):

| Ticket | D | Thema | Default gilt |
|---|---|---|---|
| B-538 | D1 | Hub-Wachstum nur Grafik (A/B/C) | A |
| B-539 | D2 | Wimpel-Markierer `markedBy` (B-532) | nach GEB10.5 |
| B-540 | D3 | Wald-Dichte, Höhlenbüsche (B-531) | ohne Werte |
| B-541 | D4 | Baumhöhe 11,25 Kacheln | ja |
| B-542 | D5 | Plantage-Pappel hinter der Farm | ja |
| B-543 | D6 | Truhe ohne Serverzustand | ja |
| B-544 | D7 | GEB9-Defaults, Heilplatz dezent | ja |
| B-545 | D8 | Gerüst nur ohne Gefahr | umgesetzt |

D4, D5, D7 am besten zusammen mit den Abnahmen GEB9.6, GEB10.5, GEB11.6 ansehen.

## Nicht-Ziele

Umsetzung der Folgearbeit (eigene Tickets/Sprints im Projekt GEB); die Geräte-Abnahmen selbst.

## Regeln und Einschränkungen

Nur Ralf bestätigt; bis dahin gelten die Defaults. Protokoll-, Spielstand- und Balancing-Änderungen (D1 C, D2, D3) nur nach Ralfs Entscheidung. GEB12.1 ist dieselbe Durchsicht aus Sicht des Projekts GEB.

## Beispiele

B-545 bestätigt → erledigt. B-538 = C → Folge-Sprint SIM/REG „Hub-Ausbauschritte“ im Projekt GEB, B-538 erledigt mit Verweis.

## Ausnahme- und Fehlerfälle

Eine Frage hängt an einer Abnahme (D2 an GEB10.5) → Ticket bleibt eingeplant, KR2.1 bleibt offen.

## Akzeptanzkriterien

- **AC-01** Jedes der Tickets B-538 bis B-545 trägt unter Notizen Ralfs Entscheidung.
- **AC-02** Bestätigte Tickets sind erledigt; zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket oder Sprint im Projekt GEB.

## Offene Fragen

keine (die Fragen sind der Inhalt der Tickets; Defaults gelten).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KR2.1 | `KR2.1-fragen-geb.md` | Workshop | Mensch | fertig |

## Abnahme

2026-10-24: KR2.1 fertig, alle acht Fragen (B-538 bis B-545) entschieden und erledigt; Folgearbeit B-531, B-568 bis B-572 im Projekt GEB.

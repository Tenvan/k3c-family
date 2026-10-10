# K4 · SIM, SRV · Protokoll für Bosse, Events und Inselwechsel

- **Status:** erledigt
- **Projekt:** KMP
- **Domäne:** SIM, SRV
- **Reife:** bereit
- **Tickets:** B-154, B-080, B-383, B-345
- **Start-Commit:** 0a5accd6
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-09, 🧑 im Chat, Revision 3 (Inselwechsel durch Anwesenheit, B-345 und AC-06 in K4.2)

## Ausgangslage

Bosse, Events und Inselwechsel entstehen nach K1 bis K3 in der Simulation; das Protokoll kennt dafür keine Felder (B-154).

## Ziel

Der Client erhält Boss-HP, Phase, Warnkreis, Event und Inselwechsel-Zustand; die Wechsel-Bestätigung ist serverseitig geprüft.

Am Ende sichtbar: `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün.

## Beteiligte und Zielgruppen

Entwickler (Client und Server); 🧑 gibt die Spec frei.

## Anforderungen

B-154 › Anforderungen.

## Nicht-Ziele

Darstellung (K5), Simulation (K1 bis K3), Feedback-Events (F4, B-140).

## Regeln und Einschränkungen

`docs/protocol.md` ist die Quelle; Protokoll und beide Enden (`engine/net/`, `src/online/`) in einer Session, Version erhöhen, Testdaten in `testdata/protocol/`. Keine Simulationslogik im Client. Datei ≤ 400 Zeilen, Funktion ≤ 60. Der Sprint bleibt in der Domäne SRV.

## Beispiele

`snap` nennt Boss mit HP, Phase und Warnkreis.

## Ausnahme- und Fehlerfälle

Wechsel-Bestätigung vor dem Sieg über den Endboss → `bad_request`.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die neuen Felder, `testdata/protocol/` hat Beispiele, beide Enden parsen sie (Tests) (B-154/AC-01).
- **AC-02** Der Raum tauscht die Insel erst, wenn nach dem Sieg über den Endboss alle lebenden Spieler am Wechselpunkt stehen; Spieler und Geräte bleiben zugeordnet (Test) (B-154/AC-02, B-345/AC-01).
- **AC-03** Der Snapshot enthält Boss-HP, Phase, Warnkreis und das aktive Event mit Restzeit (Test auf Testdaten) (B-154/AC-03).
- **AC-04** Protokollversion erhöht, ältere Clients erhalten `version` (Test) (B-154/AC-04).
- **AC-05** Bytes je Tick in einer Bosswelle mit 4 Spielern gemessen und notiert, höchstens 200 Byte je Tick und Client (Q08), `task check:go` grün (B-154/AC-05).
- **AC-06** Eine Dev-Aktion wechselt den Schwierigkeitsgrad eines laufenden Raums ab der nächsten Welle, ohne Dev-Mode wird sie abgelehnt (Test) (B-080/AC-02).
- **AC-07** Die Welt stellt Endboss-Phase, Warnkreis, aktives Event mit Restzeit und Inselwechsel als Felder bereit (Test in `engine/sim/`) (B-383/AC-01).

## Offene Fragen

- Revision 3 (2026-10-09): K4.2 war blockiert, weil es seit K2 keine Wechsel-Bestätigung gibt (B-384). Beschluss 🧑: Anwesenheit am Wechselpunkt reicht, keine Nachricht `confirmIsland`; AC-02 geändert, B-345 (Raum tauscht Insel) und die Dev-Aktion Schwierigkeitsgrad (AC-06) gehören zu K4.2.

- Revision 2 (2026-10-09): K4.1 war blockiert, weil die Sim Phase, Warnkreis, Event und Inselwechsel nicht in der Welt bereitstellt (B-383). Beschluss 🧑: neue SIM-Session K4.1a vor K4.1, Feldnamen nach Vorschlag (siehe K4.1a).

- Reihenfolge der Versionssprünge K4 und W5: Beide erhöhen die Protokollversion und ändern dieselben Beispiele; K4.1 setzt W5 voraus (Fahrplan), bestätigt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| K4.1a | `K4.1a-sim-zustand-spiegeln.md` | Umsetzung | autonom | fertig |
| K4.1 | `K4.1-felder.md` | Umsetzung | autonom | fertig |
| K4.2 | `K4.2-eingabe-version-bytes.md` | Umsetzung | autonom | fertig |
| K4.3 | `K4.3-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-10, Review K4.3: AC-01 und AC-03 K4.1, AC-02 und AC-04 bis AC-06 K4.2, AC-07 K4.1a › Ergebnis; AC-05 mit Ereignissen 2,9 Byte je Tick (Q08), `delta` 913 Byte notiert.
`task check` und `task check:go` grün (Shell, B-275). Behoben: `docs/protocol.md` › Kampf nannte noch Version 5 und eine Wechsel-Eingabe.
Neu: B-385 (`islandSwitch`-Ereignis geht beim Inseltausch verloren). B-080 bleibt offen (AC-01 nicht im Sprint).
Version: v0.16.0 vorgeschlagen (Minor: Protokoll v6, Clients der Version 5 werden abgewiesen, Raum tauscht die Insel); gesetzt erst nach Bestätigung durch 🧑.

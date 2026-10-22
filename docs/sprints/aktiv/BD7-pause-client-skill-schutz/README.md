# BD7 · SIM, CLI · Pause-Client, Skill-Reset beim Tod, Schutz wartender Monarchen

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** SIM, CLI
- **Reife:** bereit
- **Tickets:** B-524, B-530
- **Start-Commit:** 013dcb34
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-22

## Ausgangslage

BD2 merkt den gehaltenen Skill-Slot in `Player.skillHeld`, vergisst ihn aber beim Tod nicht (B-524). BD3 hat die Raum-Pause (Protokoll v8, `pause`/`resume`, `engine/room/pause.go`) und den Schutz `sim.Player.Protected` gebaut; wartende Monarchen nach Verbindungsverlust sind in der Simulation trotzdem verwundbar (B-530, Regel `docs/rules/bedienung.md` § 3). Der Client sendet `pause`/`resume` noch nicht: die Optionen-Szene (S5.2) hält nur die eigenen Eingaben an.

## Ziel

Das Pause-Menü pausiert das Gerät wirklich am Server, getrennte Monarchen sind geschützt und ein Skill-Druck nach dem Respawn meldet wieder `castFailed`. Am Ende sichtbar: `task check` und `task check:go` grün, Server-Log zeigt beim Öffnen des Menüs „🚦 Gerät pausiert“.

## Beteiligte und Zielgruppen

Spielende am Gerät und online; Agent in SIM und CLI; 🧑 nimmt am Gerät ab.

## Anforderungen

- B-524 › Anforderungen.
- B-530 › Anforderungen (Rev. 2): Die Simulation behandelt freie Monarchen (`Player.Free`, vom Raum für `waiting` und `free` gesetzt) wie geschützte: kein Schaden, kein Ziel.
- Sprint-eigen (Client): Solange die Optionen-Szene offen ist und der Client im Raum ist, ist das Gerät am Server pausiert. Öffnen sendet einmal `{"t":"pause"}`, Schließen einmal `{"t":"resume"}` (Protokoll v8). Nach einem Wiederverbinden mit offenem Menü sendet der Client `pause` erneut (der Server hat die Pause beim Abbruch beendet). Die Entscheidung, was zu senden ist, ist eine reine Funktion mit Test.

## Nicht-Ziele

Ausblenden wartender Monarchen im Client (§ 3, eigenes Ticket bei Bedarf); Anzeige des Pause-Zustands anderer Geräte (B-529); Änderungen an Protokoll oder Raum (`engine/room`, `engine/net`).

## Regeln und Einschränkungen

`docs/rules/bedienung.md` § 1 und § 3; `docs/decisions/001` (keine Spiel-Logik im Client, `tests/noSim.test.ts`); Protokoll v8 unverändert (`docs/protocol.md`); deterministisch, 2 Spieler je Gerät; Datei ≤ 400 Zeilen, Funktion ≤ 60. Golden bleibt unverändert (Felder sind im Normalbetrieb false bzw. 0). Setzt BD2 (PR #431) und BD3 (in PR #440) voraus; der Branch enthält sie per Merge.

## Beispiele

- Monarch hält Skill 1 ohne Ziel (`castFailed`), stirbt, lebt wieder, drückt neu → zweites `castFailed`.
- Gerät verliert nachts die Verbindung, Gegner laufen über seinen Monarchen → keine HP-Verluste, kein Gegner wählt ihn als Ziel.
- Couch-Raum: Esc öffnet das Menü → `pause`, Nacht und Gegner stehen; Esc schließt → `resume`, Raum läuft.

## Ausnahme- und Fehlerfälle

- Menü offen, Verbindung bricht ab → kein Senden; nach dem Wiederbeitritt mit noch offenem Menü einmal `pause`.
- Menü wird geschlossen, während der Client nicht im Raum ist → nichts senden (Server hat die Pause schon beendet).
- „Spiel verlassen“ aus dem Menü → `leave`, kein `resume` nötig.
- Toter Monarch (`RespawnIn > 0`) mit gehaltener Taste: `skillHeld` ist 0, nach dem Respawn zählt der nächste Druck als neu.

## Akzeptanzkriterien

- **AC-01** Sim-Test: halten → `castFailed`, sterben, wiederbeleben, neu drücken → zweites `castFailed` (B-524/AC-01).
- **AC-02** Go-Test: Ein wartender Monarch verliert während der Frist keine HP (B-530/AC-01).
- **AC-03** Go-Test: Ein freier Monarch ist kein Ziel der Gegner und nimmt keinen Flächenschaden; besetzte Monarchen unverändert (B-530/AC-02).
- **AC-04** Vitest: Die reine Funktion liefert beim Öffnen genau ein `pause`, beim Schließen genau ein `resume`, außerhalb des Raums nichts; nach Wiederbeitritt mit offenem Menü erneut `pause` (Mock-Socket in `clientConnection`).
- **AC-05** `GameScene` meldet den Zustand der Optionen-Szene je Frame an den Client; `tests/noSim.test.ts` und `task check` grün.

## Offene Fragen

keine blockierenden. Beim autonomen Freigeben mit Default entschieden und als Ticket zur Prüfung durch 🧑 angelegt: erneutes `pause` nach Wiederverbinden (Schutz beginnt neu).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD7.1 | `BD7.1-skillheld-tod.md` | Umsetzung | autonom | fertig |
| BD7.2 | `BD7.2-wartende-geschuetzt.md` | Umsetzung | autonom | offen |
| BD7.3 | `BD7.3-pause-client.md` | Umsetzung | autonom | offen |
| BD7.4 | `BD7.4-review.md` | Review | autonom | offen |

BD7.1 (SIM) AC-01, BD7.2 (SIM) AC-02, AC-03, BD7.3 (CLI) AC-04, AC-05, BD7.4 Review (alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

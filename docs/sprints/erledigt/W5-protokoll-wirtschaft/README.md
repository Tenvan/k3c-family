# W5 · SRV · Protokoll für Berufe, Händler, Lager und Hub-Stufe

- **Status:** erledigt
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-153, B-283
- **Start-Commit:** e26de644
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-06, Chat, durch 🧑, Revision 3; B-330 Variante B

## Ausgangslage

Die Simulation liefert nach W1 bis W4 Hub-Stufe, Lager, Berufe und Händler; das Protokoll kennt davon nur, was B-123 liefert; Beruf ausbilden, Tauschen, Berufe der Bürger und Grabstein fehlen und stehen in B-283 (B-153, B-283).

## Ziel

Der Client erhält Hub-Stufe, Lagerstand, Wartegrund und Händler-Zustand; die Protokollversion ist dafür erhöht. Hub-Ausbau, Tausch beim Händler und Berufswahl bleiben Bezahlen am Ort (`input.pay`, A halten), es gibt dafür keine neuen Eingaben (B-330, Variante B).

Am Ende sichtbar: `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün.

## Beteiligte und Zielgruppen

Entwickler (Client und Server); 🧑 gibt die Spec frei.

## Anforderungen

B-153 › Anforderungen, B-283 › Anforderungen.

## Nicht-Ziele

Darstellung (W6), Simulation (W1 bis W4), Feedback-Events (F4, B-140).

## Regeln und Einschränkungen

`docs/protocol.md` ist die Quelle; Protokoll und beide Enden (`engine/net/`, `src/online/`) in einer Session, Version erhöhen, Testdaten in `testdata/protocol/`. Keine Simulationslogik im Client. Datei ≤ 400 Zeilen, Funktion ≤ 60. Der Sprint bleibt in der Domäne SRV.

## Beispiele

`snap` nennt je Bauplatz den Wartegrund und im Lager den Stand mit Maximum.

## Ausnahme- und Fehlerfälle

nicht relevant: keine neuen Eingaben (B-330). Am Ort ohne Händler passiert nichts, ohne Material wartet der Hub-Ausbau (Sim, W1).

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die neuen Felder, `testdata/protocol/` hat Beispiele, beide Enden parsen sie (Tests) (B-153/AC-01, B-283/AC-01).
- **AC-02** verworfen (B-330, Entscheidung 🧑 2026-10-06): ~~Der Server lehnt ungültige Eingaben mit `bad_request` ab, auch ungültiger Beruf und ungültiger Tausch (Test)~~; Hub-Ausbau, Tausch und Berufswahl bleiben `input.pay` am Ort (B-153/AC-02, B-283/AC-02).
- **AC-03** Der Snapshot enthält Hub-Stufe, Lagerstand mit Maximum, Wartegrund und Händler-Zustand (Test auf Testdaten) (B-153/AC-03).
- **AC-04** Protokollversion erhöht (wegen der Felder aus W5.1), ältere Clients erhalten `version` (Test) (B-153/AC-04).
- **AC-05** Bytes je Tick mit 4 Spielern und 3 Stufen gemessen und notiert, `task check:go` grün (B-153/AC-05).
- **AC-06** Die Ereignisse `revived` (Wiederbeleben durch einen Mitspieler, Q62), `disarmed` (Bürger verliert Ausrüstung) und `equipmentTaken` (Gegner trägt Ausrüstung weg, Q69) stehen in `docs/protocol.md`, in `testdata/protocol/` und in den Client-Typen, beide Enden parsen sie (Test) (Q62, Q69).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W5.1 | `W5.1-felder.md` | Umsetzung | autonom | fertig |
| W5.2 | `W5.2-eingaben-version-bytes.md` | Umsetzung | autonom | fertig |
| W5.3 | `W5.3-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-06 (W5.3, autonom): AC-01, AC-03, AC-06 (W5.1: `TestWirtschaftZustand`, `TestFormWieBeispiele`, `clientWirtschaft.test.ts`), AC-04, AC-05 (W5.2: `TestHandschlagFalsch` mit v4, Bytes vorher/nachher in `docs/protocol.md`) mit Nachweis; AC-02 verworfen (B-330, Variante B).
- Review des Diffs: keine schweren Befunde, keine behoben; `engine/sim/` unverändert, Version in Server, Client, Dokument und Beispielen 5. Die Änderung an `src/online/clientSkills.test.ts` (nur Version 4 → 5, außerhalb der erlaubten Dateien) ist als zwingende Folge des Versionssprungs abgenommen.
- Neue Tickets: keine. B-153 und B-283 erledigt und archiviert (Berufe, Grabstein und Wiederbeleben stehen als `troops[].profession`, `drops`, `revived`).
- Version: v0.14.0 vorgeschlagen (Minor: Protokoll v5, Clients der Version 4 werden abgewiesen, neue Zustandsfelder und Ereignisse); gesetzt erst nach Bestätigung durch 🧑.

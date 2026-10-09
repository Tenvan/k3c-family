# B-154 · Das Protokoll kennt Bosse, Phasen, Events und den Inselwechsel

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** K4
- **Projekt:** KMP
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-09, 🧑 im Chat, mit Sprint K4 Revision 3

## Ausgangslage

Bosse, Events und Inselwechsel entstehen in der Simulation (B-130, B-131, B-103, B-102); das Protokoll in `engine/net/protocol.go` und `docs/protocol.md` kennt dafür keine Felder.

## Ziel

Der Client erhält Boss-HP und Phase, Fähigkeits-Warnungen (Flächenschlag), aktive Events (Vollmond, Blutmond, Händler-Überfall) und den Zustand des Inselwechsels; die Bestätigung zum Wechsel ist serverseitig geprüft. Nutzen: B-132 kann Kämpfe lesbar zeichnen, ohne zu rechnen.

## Beteiligte und Zielgruppen

Entwickler (Client und Server); eigene Session laut `docs/arbeitsweise.md` › Protokoll.

## Anforderungen

- Zustand (s2c): Bosse mit HP, Phase und Warnkreis, aktives Event mit Restzeit, Inselwechsel (freigegeben, wer ist am Punkt), besiegte Bosse und Sieg/Niederlage-Ereignis.
- Eingabe (c2s): Inselwechsel bestätigen; nur gültig, wenn der Endboss besiegt ist.
- Protokollversion erhöhen, Testdaten in `testdata/protocol/`, serverseitige Prüfung aller Eingaben.
- Snapshot-Größe in einer Bosswelle messen (`engine/sim/island_bench_test.go`), Delta-tauglich (`engine/net/delta.go`).

## Nicht-Ziele

Darstellung (B-132), Simulation (B-130, B-131, B-103, B-102), Feedback-Events (B-140).

## Regeln und Einschränkungen

`docs/protocol.md` ist die Quelle; Protokoll und beide Enden in einer Session. Der Client rechnet nichts (`src/scenes/noSim.test.ts`). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; 2+ Spieler.

## Beispiele

`snap` nennt `boss:{hp:800,max:1000,phase:2,warn:{x:120,r:4}}` → der Client zeichnet Leiste und Warnkreis.

## Ausnahme- und Fehlerfälle

Inselwechsel vor dem Sieg über den Endboss oder von einem Spieler allein → Eingabe wird mit `bad_request` abgelehnt bzw. wartet auf die übrigen lebenden Spieler.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die neuen Felder (Boss, Phase, Warnkreis, Event, Inselwechsel); `testdata/protocol/` hat Beispiele; beide Enden parsen sie (Tests).
- **AC-02** Der Raum tauscht die Insel erst, wenn nach dem Sieg über den Endboss alle lebenden Spieler am Wechselpunkt stehen; eine eigene Wechsel-Bestätigung gibt es nicht (Revision 2, Beschluss 🧑 2026-10-09) (Test in `engine/room/`).
- **AC-03** Der Snapshot einer Bosswelle enthält Boss-HP, Phase und Warnkreis; der Snapshot einer Eventnacht das Event mit Restzeit (Test auf Testdaten).
- **AC-04** Protokollversion erhöht; ein älterer Client erhält `version` (Test).
- **AC-05** Bytes je Tick in einer Bosswelle mit 4 Spielern gemessen und in den Notizen festgehalten; höchstens 200 Byte je Tick und Client (Q08); `task check:go` grün.

## Offene Fragen

keine

## Notizen

Aus Plan Phase 3 (K4). Setzt K1 bis K3 voraus. Bandbreitenbudget kommt aus B-140.

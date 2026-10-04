# K4 · SRV · Protokoll für Bosse, Events und Inselwechsel

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-154
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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
- **AC-02** Der Server lehnt die Wechsel-Bestätigung vor dem Sieg über den Endboss ab (Test) (B-154/AC-02).
- **AC-03** Der Snapshot enthält Boss-HP, Phase, Warnkreis und das aktive Event mit Restzeit (Test auf Testdaten) (B-154/AC-03).
- **AC-04** Protokollversion erhöht, ältere Clients erhalten `version` (Test) (B-154/AC-04).
- **AC-05** Bytes je Tick in einer Bosswelle mit 4 Spielern gemessen und notiert, `task check:go` grün (B-154/AC-05).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| K4.1 | `K4.1-felder.md` | Umsetzung | autonom | offen |
| K4.2 | `K4.2-eingabe-version-bytes.md` | Umsetzung | autonom | offen |
| K4.3 | `K4.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–

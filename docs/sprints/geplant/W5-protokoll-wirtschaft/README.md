# W5 · SRV · Protokoll für Berufe, Händler, Lager und Hub-Stufe

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-153
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Simulation liefert nach W1 bis W4 Hub-Stufe, Lager, Berufe und Händler; das Protokoll kennt davon nur, was B-123 liefert (B-153).

## Ziel

Der Client erhält Hub-Stufe, Lagerstand, Wartegrund und Händler-Zustand; Eingaben sind serverseitig geprüft.

Am Ende sichtbar: `docs/protocol.md` mit neuen Feldern, Beispiele in `testdata/protocol/`, `task check:go` und `task check` grün.

## Beteiligte und Zielgruppen

Entwickler (Client und Server); 🧑 gibt die Spec frei.

## Anforderungen

B-153 › Anforderungen.

## Nicht-Ziele

Darstellung (W6), Simulation (W1 bis W4), Feedback-Events (F4, B-140).

## Regeln und Einschränkungen

`docs/protocol.md` ist die Quelle; Protokoll und beide Enden (`engine/net/`, `src/online/`) in einer Session, Version erhöhen, Testdaten in `testdata/protocol/`. Keine Simulationslogik im Client. Datei ≤ 400 Zeilen, Funktion ≤ 60. Der Sprint bleibt in der Domäne SRV.

## Beispiele

`snap` nennt je Bauplatz den Wartegrund und im Lager den Stand mit Maximum.

## Ausnahme- und Fehlerfälle

Tausch ohne Händler oder Hub-Ausbau ohne Material → `bad_request`.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt die neuen Felder, `testdata/protocol/` hat Beispiele, beide Enden parsen sie (Tests) (B-153/AC-01).
- **AC-02** Der Server lehnt ungültige Eingaben mit `bad_request` ab (Test) (B-153/AC-02).
- **AC-03** Der Snapshot enthält Hub-Stufe, Lagerstand mit Maximum, Wartegrund und Händler-Zustand (Test auf Testdaten) (B-153/AC-03).
- **AC-04** Protokollversion erhöht, ältere Clients erhalten `version` (Test) (B-153/AC-04).
- **AC-05** Bytes je Tick mit 4 Spielern und 3 Stufen gemessen und notiert, `task check:go` grün (B-153/AC-05).

## Offene Fragen

Abgrenzung der Berufe-Felder zu B-123: beim Planen der Protokoll-Sessions klären (🧑 bestätigt).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W5.1 Felder und Doku in `docs/protocol.md`, `engine/net/protocol.go`, Testdaten (AC-01, AC-03).
- W5.2 Eingaben prüfen, Protokollversion, Bytes-je-Tick-Messung (AC-02, AC-04, AC-05).
- W5.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–

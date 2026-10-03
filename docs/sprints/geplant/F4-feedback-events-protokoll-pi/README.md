# F4 · SRV · Feedback-Ereignisse im Protokoll und Pi-Betrieb

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-140, B-142, B-143
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Feedback-Ereignisse aus F3 erreichen den Client nicht; die Snapshot-Größe bei 4 Spielern × 3 Stufen ist ungemessen. Sicherungen liegen auf demselben Datenträger wie der Spielstand, Reports und Client-Log rotieren nicht, und schreibende Endpunkte sind im Heimnetz offen (`engine/net/handler.go`).

## Ziel

Ereignisse laufen im Protokoll zum Client mit gemessener Bandbreite, der Pi füllt seine SD-Karte nicht, und Restore ist abgesichert. Am Ende sichtbar: Benchmark mit Bytes je Tick, README-Abschnitt zum Backup, Test „Restore ohne Berechtigung abgelehnt“.

## Beteiligte und Zielgruppen

Entwickler (Server und Client), 🧑 als Betreiber des Pi und Entscheider über Budget (Q08), Backup-Ziel (Q18) und Schutzmodell (Q17).

## Anforderungen

B-140, B-142 und B-143 › Anforderungen.

## Nicht-Ziele

Ton und Effekte im Client (B-167, B-164), Pi-Einrichtung und Lastmessung am Pi (SP11), Protokoll für Berufe, Bosse, Skills (B-153, B-154, B-123).

## Regeln und Einschränkungen

Domäne SRV. F3 ist abgeschlossen. Protokolländerung als eigene Session, nur Protokoll und beide Enden (`docs/arbeitsweise.md` › Grenzfälle; Client-Typ `src/online/protocol.ts` ist dabei erlaubt). Von diesem Rechner aus wird nichts am Pi getestet. Standardbibliothek zuerst.

## Beispiele

4 Spieler, 3 Stufen, Nacht: der Benchmark meldet Bytes je Tick (Mittel, p99) und schlägt fehl, wenn das Budget überschritten ist.

## Ausnahme- und Fehlerfälle

Budget überschritten → Ereignisse niedriger Priorität werden im Server gekürzt, nicht im Client (Rückmeldung an F3-Obergrenze, neues Ticket).

## Akzeptanzkriterien

- **AC-01** Ein Ereignis erscheint im Snapshot nur der betroffenen Stufe (B-140/AC-01).
- **AC-02** Protokolldokument, Client-Typ und Beispiele unter `testdata/protocol/` kennen die Ereignisse (B-140/AC-02).
- **AC-03** Der Benchmark nennt Bytes je Tick, der Ist-Wert steht in `docs/protocol.md` (B-140/AC-03).
- **AC-04** Das Budget KB/s je Client steht in `docs/protocol.md` und wird geprüft (B-140/AC-04, B-142/AC-04).
- **AC-05** Reports und Client-Log rotieren, `saves/` bleibt unberührt (B-142/AC-01, B-142/AC-02).
- **AC-06** Der Backup-Befehl auf ein zweites Gerät ist in der README beschrieben und lokal getestet (B-142/AC-03).
- **AC-07** Restore ist geschützt, das Spiel läuft ohne Geheimnis weiter, abgelehnte Zugriffe stehen im Log (B-143/AC-01, B-143/AC-02, B-143/AC-03, B-143/AC-04).

## Offene Fragen

Q08 (Budget), Q17 (Schutzmodell), Q18 (Backup-Ziel); entscheidet 🧑. Q17 und Q08 blockieren die Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- F4.1 Protokoll: Ereignisse in Snapshot, Client-Typ, Beispiel, Test; Bandbreiten-Benchmark und Budget-Prüfung (AC-01, AC-02, AC-03, AC-04).
- F4.2 Rotation von Reports und Client-Log, Backup-Befehl und README (AC-05, AC-06).
- F4.3 Absicherung von Restore und Endpunkten (AC-07).
- F4.4 Review nach `docs/arbeitsweise.md` (alle Kriterien).

## Abnahme

–

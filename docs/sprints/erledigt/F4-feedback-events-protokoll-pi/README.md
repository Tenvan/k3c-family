# F4 · SRV · Feedback-Ereignisse im Protokoll und Pi-Betrieb

- **Status:** erledigt
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-140, B-142, B-143
- **Start-Commit:** 3ae4c1d
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-140, B-142, B-143; bestätigt die Auslegung „Token = `K3C_STATUS_TOKEN`“ (F4.3)

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

Q08, Q17 und Q18 sind entschieden (2026-10-03, `docs/fragenkatalog.md` › Beschlüsse): Budget ≤ 200 Byte je Tick und Client im Mittel (bei 30 Hz = 6 KB/s), im Delta mitgesendet; Restore nur mit Token, Report und Client-Log nur Limits und Rotation, kein Port-Forwarding (dokumentieren); Backup-Ziel USB-Stick am Pi, Restore-Probe einmal durchspielen (🧑 am Pi). Als Token gilt das vorhandene `K3C_STATUS_TOKEN` (Auslegung in F4.3, von 🧑 mit der Freigabe bestätigt).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| F4.1 | `F4.1-protokoll-ereignisse-bandbreite.md` | Umsetzung | autonom | fertig |
| F4.2 | `F4.2-rotation-backup.md` | Umsetzung | autonom | fertig |
| F4.3 | `F4.3-restore-absichern.md` | Umsetzung | autonom | fertig |
| F4.4 | `F4.4-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-03, Review F4.4 (Sonnet): `task check` und `task check:go` grün, keine schweren Befunde, keine neuen Tickets.
AC-01 bis AC-04 geprüft in F4.1, AC-05 und AC-06 in F4.2, AC-07 in F4.3 (Ergebnisse der Sessions).
Restore-Probe am Pi (AC-06, Q18, 🧑): angenommen, Validierung offen (Session); Backup lokal getestet, der Pi-Lauf folgt mit dem Gerät.
Version: v0.6.0 vorgeschlagen (Minor: Wirkung im Server — Rotation, Backup-Skript, Restore nur mit Token, Ereignis-Budget geprüft; aktuell v0.5.0).

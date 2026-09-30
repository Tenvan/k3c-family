# SP02 · SRV · Protokoll v2 & Raummodell

- **Status:** aktiv
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-030, B-036, B-038, B-039
- **Start-Commit:** 0fb32f8
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (Revision 2)

## Ausgangslage

Das Online-Protokoll v1 (`src/online/protocol.ts`, `room.ts`, `wsServer.ts`) kennt einen Monarchen pro Gerät, Räume
nur per URL-Parameter `?online=RAUM`, bis zu 8 Spieler pro Raum und ist nirgends beschrieben. Jeder Client baut das
Level selbst aus dem Seed; das geht nach dem Löschen der TS-Simulation (SP09) nicht mehr.

## Ziel

Protokoll und Raummodell für mehrere Räume, mehrere lokale Spieler pro Gerät und gemischten Couch-/Online-Koop sind
beschlossen. Am Ende sichtbar: `docs/protocol.md`, Beispiel-Nachrichten in `testdata/protocol/`,
`docs/decisions/002-protokoll-v2.md`.

## Beteiligte und Zielgruppen

🧑 hat das Raummodell entschieden (2026-09-30, siehe Regeln); die Umsetzung in SP07 (Server) und SP08 (Client)
arbeitet danach.

## Anforderungen

B-030, B-036 und B-038 › Anforderungen, B-039 (Snapshot-Takt). Sprint-eigen: Nachrichten für Eingaben pro lokalem
Spieler, Snapshot (voll/Delta), Level-Übertragung, Raumliste, Takt 30 Hz, Versionsfeld.

## Nicht-Ziele

Engine-Code, Änderungen an `src/online/` (Protokoll v1 läuft bis SP08 weiter). Die Umsetzung folgt in SP07 (Server)
und SP08 (Client). Lobby-Oberfläche (B-037).

## Regeln und Einschränkungen

Eine Protokoll-Änderung betrifft Client und Server (Grenzfall Protokoll in `docs/arbeitsweise.md`). Entscheidung 001.
Beschlüsse von 🧑 (2026-09-30, Chat), gelten für den ganzen Sprint:

1. **Beitreten:** Der Server liefert eine Raumliste (Heimnetz, nichts geheim); jeder Raum hat zusätzlich einen kurzen
   Code zum direkten Beitreten per URL.
2. **Grenzen:** höchstens 4 Monarchen pro Raum, 4 lokale Spieler pro Gerät, 4 Räume pro Server.
3. **Abbruch:** Die Monarchen eines abgebrochenen Geräts stehen still. Kommt es binnen 60 s mit seiner Geräte-ID
   zurück, steuert es sie weiter. Danach sind die Plätze frei und jedes Gerät kann sie übernehmen. Ein leerer Raum
   wird nach 10 min aufgeräumt.
4. **Spielstand:** Ein Raum ist genau ein Spielstand. Beim Erstellen wählt man neu oder einen gespeicherten Stand;
   der Raum speichert selbst (z. B. bei Stufenwechsel und beim Aufräumen).

## Beispiele

2 Controller an der Xbox + 1 Handy im selben Raum → drei Monarchen; je Nachricht ein JSON-Beispiel aus genau diesem
Ablauf in `testdata/protocol/`.

## Ausnahme- und Fehlerfälle

Das Protokoll legt das Verhalten fest bei: falscher Version, Abbruch und Wiederverbinden (auch nach der Frist),
vollem Raum, zu vielen lokalen Spielern, zu vielen Räumen, unbekanntem Raumcode.

## Akzeptanzkriterien

- **AC-01** `docs/protocol.md` beschreibt Raum, Gerät und lokale Spieler, Raumliste und Code, Beitreten, Verlassen,
  Wiederverbinden, Grenzen und Spielstand pro Raum nach den Beschlüssen 1–4.
- **AC-02** Je Nachricht liegt ein gültiges JSON-Beispiel in `testdata/protocol/`; die Snapshot-Größe für 4 Spieler ist
  geschätzt (voll und Delta, Bytes pro Sekunde bei 30 Hz).
- **AC-03** `docs/decisions/002-protokoll-v2.md` ist geschrieben; 🧑 beschließt sie mit der Abnahme des Review-PR.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP02.1 | `SP02.1-raummodell.md` | Umsetzung | autonom | fertig |
| SP02.2 | `SP02.2-nachrichten.md` | Umsetzung | autonom | offen |
| SP02.3 | `SP02.3-review.md` | Review | autonom | offen |

## Abnahme

–

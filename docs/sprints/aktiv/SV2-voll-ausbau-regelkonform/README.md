# SV2 · SRV · Voll-Ausbau-Stand regelkonform aus der Welt-Datei, Review-Nachträge SV1

- **Status:** aktiv
- **Projekt:** WRT
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-500, B-425, B-426
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt; offene Frage: DevBuildAll je Inselstufe reicht, Default ja)

## Ausgangslage

`fullIsland` (`engine/room/fullsave.go`) baut den Voll-Ausbau-Stand mit eigener Logik statt mit den Regeln der Simulation: Stufen von Mauern, Türmen und Gebäuden passen nicht zur Hub-Stufe (B-425, `sim.DevBuildAll` wird nicht genutzt). Die Stufen kommen aus allen Biomen (`islandDepths`) statt aus der Welt-Datei wie bei einem neuen Raum (B-500). Aus dem Review von SV1 sind zwei SRV-Befunde offen: Tests nur gegen Speicher-Stores, `POST /api/dev/fullsave` ohne Content-Type-Prüfung (B-426 Punkte 2 und 3; Punkte 1 und 4 hat 🧑 am 2026-10-18 verworfen).

## Ziel

Der Voll-Ausbau-Stand ist ein Zustand, den das Spiel auch regulär erreichen kann, mit denselben Stufen wie ein neues Spiel. Am Ende sichtbar: `task check:go` grün, Messläufe auf dem Voll-Stand sind aussagekräftig.

## Beteiligte und Zielgruppen

🧑 und Agenten bei Messläufen und Abnahmen auf dem Voll-Stand; Agent in SRV; 🧑 gibt die Spec frei.

## Anforderungen

B-500, B-425, B-426 › Anforderungen und Akzeptanzkriterien (Specs noch Entwurf).

## Nicht-Ziele

Neue Ausbauregeln; Voll-Stand für Release-Server (bleibt Dev-only); Migration alter Spielstände (B-288).

## Regeln und Einschränkungen

Domäne SRV (`engine/room/`, `engine/net/`). Braucht B-425 eine neue Variante von `DevBuildAll`, ist das eine eigene SIM-Session. Golden byte-gleich (B-500/AC-01). Deterministisch.

## Beispiele

Voll-Stand anlegen → Inselstufe mit Hub 2 hat keine Mauer Stufe 3, keine Rüstkammer; Tiefen wie `sim.StandardWorld().Islands[0].Depths`.

## Ausnahme- und Fehlerfälle

`POST /api/dev/fullsave` ohne `Content-Type: application/json` → abgelehnt, Log 🚫. Release-Server ohne Dev-Mode → Voll-Stand nicht verfügbar wie bisher.

## Akzeptanzkriterien

- **AC-01** Der Voll-Stand hat die Tiefen der Welt-Datei, `islandDepths` entfällt, wenn nichts anderes es braucht (B-500/AC-01).
- **AC-02** Der Voll-Stand hält die Ausbauregeln ein, die Tests aus SV1.3 bleiben grün (B-425/AC-01, B-425/AC-02, B-425/AC-03).
- **AC-03** Ein Go-Test legt `voll-<seed>` über einen echten `store.Saves` an; `/api/dev/fullsave` prüft den Content-Type (B-426/AC-02, B-426/AC-03).

## Offene Fragen

🧑: Reicht `DevBuildAll(isl, level)` je Inselstufe (B-425)?

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SV2.1 | `SV2.1-voll-stand-regelkonform.md` | Umsetzung | autonom | fertig |
| SV2.2 | `SV2.2-store-test-content-type.md` | Umsetzung | autonom | offen |
| SV2.3 | `SV2.3-review.md` | Review | autonom | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SV2.1 (SRV) Voll-Stand aus der Welt-Datei und über `sim.DevBuildAll` (AC-01, AC-02).
- SV2.2 (SRV) Test mit echtem `store.Saves`, Content-Type-Prüfung (AC-03).
- SV2.3 (SRV) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

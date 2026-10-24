# B-581 · Ein offener Raum lässt sich in der Lobby über Entfernen schließen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BD9
- **Projekt:** BED
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-24, Ralf im Chat (mit BD9 Revision 1)

## Ausgangslage

Bei der Abnahme BD5.4 (2026-10-24, PC) zeigte sich: Offene Räume stehen in der Spielstand-Liste der Lobby, lassen sich aber nicht löschen (gewollt, BD5.1: `Manager.DeleteSave` liefert `ErrSaveInUse`, sonst schriebe der Raum den Stand neu). Code (2026-10-24): Einen Weg, einen einzelnen Raum auf Anforderung zu schließen, gibt es nicht; geschlossen wird nur beim Herunterfahren (`Manager.Close`) und nach Leerlauf (`lockedSweep`: `saveNow()`, `closed = true`, `peer.Closed`). Clients kehren bei `room_closed` in die Lobby zurück (`clientConnection.ts`). Eine Host-Rolle gibt es nicht.

## Ziel

Bei einem offenen Raum bietet „Entfernen“ (X bzw. Entf) die Option, den Raum zu schließen; danach lässt er sich löschen.

## Beteiligte und Zielgruppen

Spieler am Gerät; 🧑 nimmt ab.

## Anforderungen

- Server: `POST api/room/close?slot=NAME` (wie `DELETE api/save`): speichert den Stand, schließt den Raum, schickt verbundenen Geräten `room_closed` (Grund „vom Gerät geschlossen“) und entfernt ihn aus der Raumliste; Antworten 200, 404 (`not_found`), 400. Ablauf wie `lockedSweep`, kein zweiter Speicherweg.
- Client: Entfernen (X bzw. Entf) auf einem offenen Raum öffnet „Raum schließen?“ mit „Abbrechen“ vorgewählt; die Rückfrage nennt die Zahl verbundener Geräte (`taken`). Nach dem Schließen steht der Stand als Spielstand in der Liste und lässt sich wie gewohnt löschen.
- Verbundene Geräte landen mit Meldung in der Lobby.

## Nicht-Ziele

Automatisches Löschen nach dem Schließen.

## Regeln und Einschränkungen

Protokoll laut `docs/protocol.md` (neuer HTTP-Endpunkt dokumentieren); Server SRV, Client CLI; 2 Spieler; B nicht belegen; Texte über `t()`.

## Beispiele

Offener Raum, X → „Raum schließen?“ → Ja → Raum zu, Spielstand bleibt, danach löschbar.

## Ausnahme- und Fehlerfälle

Mitspieler ist noch verbunden → Rückfrage nennt es.

## Akzeptanzkriterien

- **AC-01** Entfernen auf einem offenen Raum bietet „Schließen“ mit Rückfrage (Vitest, Beobachtung am PC).
- **AC-02** Nach dem Schließen ist der Stand gespeichert und lässt sich löschen; verbundene Geräte sind in der Lobby (Go-Test des Endpunkts mit verbundenem Gerät, Beobachtung).

## Offene Fragen

keine. Entschieden (🧑, 2026-10-24): Jedes Gerät darf schließen, mit Rückfrage, die verbundene Geräte nennt.

## Notizen

Aus BD5.4 (Ralf, 2026-10-24). Spec: Entwurf.

# BD9 · SRV, CLI · Welt-Anzeige größer und je Spielstand, offene Räume schließen

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** SRV, CLI
- **Reife:** bereit
- **Tickets:** B-604, B-581
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-24, Ralf im Chat (Revision 1)

## Ausgangslage

Aus der Abnahme BD5.4 (Ralf, 2026-10-24, PC): Die Welt im Pausenmenü ist zu klein, in der Lobby fehlt die Welt bei offenen Räumen, und offene Räume lassen sich nicht schließen (B-604, B-581). Code: Pausenmenü 28 px (`OptionsScene.ts`), `rooms[]` ohne Welt, kein Schließen-Weg im Server.

## Ziel

Die Welt ist im Pausenmenü groß lesbar und steht in der Lobby bei jedem Spielstand; ein offener Raum lässt sich über „Entfernen“ schließen. Am Ende sichtbar: Lobby und Pausenmenü am PC.

## Beteiligte und Zielgruppen

Spielende; Agent in SRV und CLI; 🧑 entscheidet die offene Frage zu B-581 und nimmt am PC ab.

## Anforderungen

B-604 › Anforderungen, B-581 › Anforderungen.

## Nicht-Ziele

Neue Welt-Auswahl (BD8), automatisches Löschen nach dem Schließen.

## Regeln und Einschränkungen

Nur zeichnen im Client (ADR 001). Zwei Protokoll-Änderungen: `rooms[].world` (WebSocket, Version + 1) und `POST api/room/close` (HTTP); erst die SRV-Session (BD9.1) mit `docs/protocol.md`, dann die CLI-Session (BD9.2). Review-Agent vor dem Commit (Protokoll, SRV). Texte zweisprachig über `t()`; B nicht belegen; 2 Spieler.

## Beispiele

Lobby mit drei Spielständen → jede Zeile nennt ihre Welt. Offener Raum, X → „Raum schließen?“ → Ja → Raum zu, Stand bleibt und ist löschbar.

## Ausnahme- und Fehlerfälle

Mitspieler noch verbunden → Rückfrage nennt es. Spielstand ohne Welt → Platzhalter „–“.

## Akzeptanzkriterien

- **AC-01** Das Pausenmenü zeigt die Welt in 40 px (B-604/AC-01).
- **AC-02** Jeder Eintrag der Lobby nennt seine Welt, auch offene Räume (B-604/AC-02).
- **AC-03** Entfernen auf einem offenen Raum bietet „Schließen“; danach ist der Stand gespeichert und löschbar, verbundene Geräte sind in der Lobby (B-581/AC-01, B-581/AC-02).
- **AC-04** Ralf hat AC-01 bis AC-03 am PC abgenommen.

## Offene Fragen

keine. Entschieden (🧑, 2026-10-24): Jedes Gerät darf einen Raum schließen, mit Rückfrage (B-581).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD9.1 | `BD9.1-server-welt-schliessen.md` | Umsetzung | autonom | fertig |
| BD9.2 | `BD9.2-lobby-pause-anzeige.md` | Umsetzung | autonom | fertig |
| BD9.3 | `BD9.3-review.md` | Review | autonom | in Arbeit |
| BD9.4 | `BD9.4-abnahme-pc.md` | Workshop | Mensch | offen |

- BD9.1 (SRV) Server: Welt in `rooms[]`, `POST api/room/close`, Protokoll (AC-02, AC-03).
- BD9.2 (CLI) Pausenmenü 40 px, Welt je Lobby-Eintrag, Raum schließen mit Rückfrage (AC-01, AC-02, AC-03).
- BD9.3 Review: alle Kriterien prüfen, PR.
- BD9.4 (Mensch) Abnahme am PC (AC-04).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

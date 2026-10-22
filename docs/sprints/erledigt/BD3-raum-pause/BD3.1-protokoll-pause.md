# BD3.1 · Protokoll: Nachricht pause/resume, Version 8, Beispiel

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** sprint/bd3
- **Abhängig von:** –
- **Tickets:** B-214
- **Kriterien:** AC-03

## Ziel

Das Protokoll kennt die Geräte-Nachrichten `pause` und `resume` (Version 8, Beispiele unter `testdata/protocol/`); der Server nimmt sie an und merkt sich die Pause je Gerät, beide Enden sprechen Version 8.

## Kontext

Protokoll v7: `engine/net/protocol.go` (`ProtocolVersion`, `inMsg`), Verteilung `engine/net/dispatch.go` (`roomMessage`), Doku `docs/protocol.md` (Änderungsliste oben, Tabelle *Nachrichten*). Client-Konstante `PROTOCOL_VERSION` in `src/online/clientProtocol.ts` muss mitziehen, sonst lehnt der Server jeden Client ab (Protokoll-Session passt beide Enden an, `docs/arbeitsweise.md` › Domänen). Beschluss 🧑 2026-10-13: eigene Nachricht, kein Feld in `input`. Die Wirkung der Pause (Stillstand, Schutz) folgt in BD3.3; hier nur Annahme und Zustand `device.paused` im Raum (`Room.Pause`).

## Erlaubte Dateien

- `engine/net/protocol.go`, `engine/net/dispatch.go`, `engine/net/*_test.go`
- `engine/room/pause.go` (neu), `engine/room/pause_test.go` (neu), `engine/room/room.go` (Feld am Gerät)
- `docs/protocol.md`, `testdata/protocol/`
- `src/online/clientProtocol.ts` (nur die Versionskonstante), `src/online/clientSkills.test.ts` (Erwartung `v`)

## Nicht-Ziele

Wirkung der Pause auf Simulation und Schutz (BD3.3); Client sendet `pause`/`resume` noch nicht (späterer CLI-Sprint).

## Schritte

1. Test zuerst: `pause`/`resume` im Raum werden angenommen, ohne Raum `bad_request`; Beispiele `c2s-pause.json`, `c2s-resume.json` laufen roh durch.
2. `ProtocolVersion` 8, `hello`/`welcome`-Beispiele, Client-Konstante.
3. `Room.Pause(id, peer, on)` setzt `device.paused` (zweites `pause` und `resume` ohne Pause ohne Wirkung).
4. `docs/protocol.md`: Änderung gegenüber v7, Tabelle der Nachrichten.

## Fertig, wenn

- [ ] AC-03: `docs/protocol.md` beschreibt `pause`/`resume` mit Version 8, Beispiele `testdata/protocol/c2s-pause.json` und `c2s-resume.json`, Go-Test schickt sie roh.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-03 umgesetzt: `docs/protocol.md` v8 (Änderungen gegenüber v7, Tabelle), Beispiele `c2s-pause.json`, `c2s-resume.json`; geprüft: `TestPauseResumeBeispiele` (roh gesendet), `TestPauseSetztGeraet`, `TestHandschlagFalsch` (v7-Client abgelehnt).
- Client-Konstante `PROTOCOL_VERSION` 8 mitgezogen (sonst lehnt der Server jeden Client ab). Entscheidungen zur Form und Rückmeldung: B-529. Review (Sonnet): keine schweren Befunde; Hinweis „Pause endet bei Trennung“ geht in BD3.3.

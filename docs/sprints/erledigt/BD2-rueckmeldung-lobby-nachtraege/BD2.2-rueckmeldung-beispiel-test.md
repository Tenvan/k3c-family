# BD2.2 · Server-Test prüft das Beispiel der Rückmeldungs-Ereignisse gegen die Simulation

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** bd2/2-rueckmeldung-beispiel-test
- **Abhängig von:** BD2.1
- **Tickets:** B-377
- **Kriterien:** AC-02

## Ziel

Ein Go-Test in `engine/net` erzeugt `strike` (Monarch) und `castFailed` über die Simulation und vergleicht ihre Schlüssel mit `testdata/protocol/s2c-snapshot-rueckmeldung.json`; ändert die Simulation Felder, wird der Test rot (B-377).

## Kontext

- Beispiel: `testdata/protocol/s2c-snapshot-rueckmeldung.json` (Ereignisse `strike` mit `hit`, `castFailed`), angelegt in S9.2; der Client liest es in `src/scenes/effects/effects.test.ts`. Auf Server-Seite prüft `engine/net/ws_test.go` nur die bestehenden Beispiele.
- Quellen der Ereignisse: `engine/sim/monarch.go` (`strike`, Feld `hit`), `engine/sim/skills.go` (`castFailed {from, slot, x}`), Kommentar `engine/sim/events.go`. Gegner-`strike` ohne `hit` zählt nicht als Abweichung.
- Vorgehen: Welt über die öffentliche API der Sim aufbauen (Spieler, ein Schlag auf einen Gegner, ein Skill ohne Ziel), Ereignisse aus dem Snapshot/`events` lesen, Schlüsselmengen vergleichen (nicht Werte). Hilfen aus `ws_test.go` und `engine/sim/feedback_test.go` (Aufbau) wiederverwenden, nichts in `engine/sim` ändern.
- Protokoll-Grenzfall (`docs/arbeitsweise.md` › Domänen): nur Test, Beispieldatei bleibt unverändert. Deterministisch, `engine/rng`.
- Abhängigkeit BD2.1: `castFailed` darf bei gehaltener Taste nur einmal kommen; der Test löst es mit einem einzelnen Tick aus.

## Erlaubte Dateien

- `engine/net/` (neue Testdatei, z. B. `feedback_example_test.go`, oder Erweiterung von `ws_test.go`)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Neue Ereignisse, Änderungen am Protokoll, an `testdata/protocol/` oder an `engine/sim/`, Client-Tests.

## Schritte

1. Branch `bd2/2-rueckmeldung-beispiel-test`, `Status: in Arbeit`. Beispieldatei, `ws_test.go` und die Emit-Stellen lesen.
2. Test schreiben: beide Ereignisse aus der Sim erzeugen, Schlüssel je Ereignis mit dem Beispiel vergleichen (Gegner-`strike` ausnehmen).
3. Gegenprobe von Hand: Feld `hit` testweise umbenennen → Test rot, danach zurücknehmen (im Ergebnis nennen).
4. `task check:diff` (enthält `check:go`). Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-02: `task check:go` enthält einen Test, der die Schlüssel von `strike` (Monarch) und `castFailed` aus der Simulation mit `s2c-snapshot-rueckmeldung.json` vergleicht, und ist grün.
- [ ] Gegenprobe (umbenanntes Feld → rot) im Ergebnis belegt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; währenddessen nur der gezielte Go-Test (`check_run` mit Filter). Keine manuellen Prüfungen.

## Ergebnis

- AC (B-377) geprüft: `TestRueckmeldungEreignisseWieBeispiel` (engine/net/feedback_example_test.go) baut strike ohne Treffer, strike mit Treffer und castFailed aus der Simulation und vergleicht die JSON-Schlüssel mit dem Beispiel `s2c-snapshot-rueckmeldung.json`. Gegenprobe: `hit` testweise umbenannt → Test rot, zurückgenommen → grün.
- Keine Abweichung, keine neuen Tickets.

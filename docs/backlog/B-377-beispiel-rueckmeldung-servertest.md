# B-377 · Ein Server-Test prüft das Beispiel der Rückmeldungs-Ereignisse gegen die Simulation

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

S9.2 hat `testdata/protocol/s2c-snapshot-rueckmeldung.json` (Ereignisse `strike` mit `hit`, `castFailed`) angelegt. Der Client liest es in `src/scenes/effects.test.ts`; auf der Server-Seite vergleicht kein Test die Schlüssel des Beispiels mit den Ereignissen, die `engine/sim` (`monarch.go`, `skills.go`) wirklich sendet (`ws_test.go` prüft nur die bestehenden Beispiele). Gefunden im Review S9.4.

## Ziel

Ändert die Simulation Felder von `strike` oder `castFailed`, wird ein Test rot, bevor Client und Beispiel auseinanderlaufen.

## Beteiligte und Zielgruppen

Entwickler (Agent); kein Spieler-Effekt.

## Anforderungen

- Ein Go-Test in `engine/net` erzeugt beide Ereignisse über die Simulation und vergleicht ihre Schlüssel mit dem Beispiel.

## Nicht-Ziele

Neue Ereignisse; Änderungen am Protokoll oder an `engine/sim/`.

## Regeln und Einschränkungen

Protokoll-Grenzfall (`docs/arbeitsweise.md` › Domänen): nur Test, Beispiel bleibt unverändert; deterministisch.

## Beispiele

Feld `hit` an `strike` umbenannt → Test rot.

## Ausnahme- und Fehlerfälle

Gegner-`strike` ohne `hit` zählt nicht als Abweichung.

## Akzeptanzkriterien

- **AC-01** `task check:go` enthält einen Test, der die Schlüssel von `strike` (Monarch) und `castFailed` aus der Simulation mit `s2c-snapshot-rueckmeldung.json` vergleicht, und ist grün.

## Offene Fragen

keine

## Notizen

–

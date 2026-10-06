# SK1.3 · Schlag und Skill ohne Ziel erzeugen ein Ereignis

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** sk1/3-ereignis-ohne-ziel
- **Abhängig von:** SK1.1 (gleiche Datei `monarch.go`)
- **Tickets:** B-321
- **Kriterien:** AC-04, AC-03

## Ziel

Auf `develop` sendet die Sim `strike` auch ohne Treffer (`hit: false`, mit Abklingzeit) und `castFailed` für einen bereiten Skill ohne Ziel; das ist die Voraussetzung für S9.2 und S9.3 (B-318).

## Kontext

- `engine/sim/monarch.go` › `stepAttack`: Heute sucht es mit `nearest(...)` den nächsten lebenden Gegner in `monarch.Attack.Range`; ohne Gegner `return` ohne Ereignis und ohne Abklingzeit. Mit Gegner: `p.AttackCooldown = a.Cooldown`, `emit(w, "strike", Event{"from": p.ID, "x": unitX(p.X)})`, dann `applyDamage`.
- `engine/sim/skills.go` › `castSkill`: prüft Slot belegt, Skill gelernt, Abklingzeit 0; dann `cast(w, p, e)` aus `skillEffects`. Liefert `cast` `false` (kein Ziel), endet es still ohne Abklingzeit.
- `engine/sim/enemies.go:264` sendet `strike` für Gegner (`from` = Gegner-ID); das bleibt unverändert, `hit` gibt es nur beim Monarchen.
- Ereignis-Liste als Kommentar in `engine/sim/events.go`.
- Beschluss 🧑 2026-10-06 zu B-318: Rückmeldung als Server-Ereignis, der Client rechnet nichts.
- Golden-Läufe ohne Monarch-Eingaben ändern sich nicht; Läufe mit Schlag ins Leere bekommen neue `strike`-Ereignisse.

## Erlaubte Dateien

- `engine/sim/monarch.go` (nur `stepAttack`), `engine/sim/skills.go` (nur `castSkill`), `engine/sim/events.go` (nur Kommentar)
- `engine/sim/feedback_test.go` (neu; Name legt die Session fest)
- `testdata/golden/` (nur über `task golden:update`, mit Begründung)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Protokoll (`docs/protocol.md`, `engine/net/`) und Client (S9.2, S9.3); Abklingzeit für fehlgeschlagene Skills; Gegner-`strike`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Test zuerst (`feedback_test.go`): Schlag ohne Gegner → genau ein `strike` mit `hit: false`, während der Abklingzeit kein zweites; mit Gegner `hit: true`.
3. `stepAttack`: Abklingzeit und `strike` auch ohne Gegner, Feld `hit`; Schaden nur mit Gegner.
4. Test: bereiter Skill ohne Ziel → `castFailed {from, slot, x}`; leerer Slot, nicht gelernter Skill, laufende Abklingzeit → kein Ereignis.
5. `castSkill`: liefert `cast` `false`, `emit(w, "castFailed", Event{"from": p.ID, "slot": slot, "x": unitX(p.X)})`.
6. Kommentar in `events.go` ergänzen, `task check:go`; ändern sich Golden-Daten, Grund im Ergebnis.

## Fertig, wenn

- [ ] AC-04: Tests aus Schritt 2 und 4 grün (B-321/AC-01, B-321/AC-02).
- [ ] AC-03: `task check:go` grün; Golden-Änderungen nur durch neue Ereignisse, begründet (B-321/AC-03).

## Prüfen

```bash
task check:go
```

## Ergebnis

–

# KB5 · SRV, CLI, DEV · Skills am Wirkort, DoTs und Buffs sichtbar, im Spiel und im Studio

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** SRV, CLI, DEV
- **Reife:** bereit
- **Tickets:** B-674, B-675, B-676
- **Start-Commit:** 88ea604b
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude

## Ausgangslage

Flächenzauber werden am Monarchen gezeichnet statt am Wirkort (B-674), Status-Effekte über die Dauer sind weder im Snapshot noch im Client sichtbar (B-675), das Studio zeigt beides nicht (B-676).

## Ziel

Skills erscheinen dort, wo sie wirken; DoTs, Buffs und Kontrolleffekte sind im Spiel und im Studio sichtbar. Am Ende sichtbar: fireball am Gegner, Betäubung, Schild und Sturm als Zeichnung.

## Beteiligte und Zielgruppen

🧑 Ralf und Familie spielen und prüfen; Sessions der Domänen SRV, CLI, DEV setzen um.

## Anforderungen

Siehe `B-674`, `B-675`, `B-676` › Anforderungen.

## Nicht-Ziele

Neue Skill-Looks oder Werte. Abnahmen B-674 AC-04, B-675 AC-04, B-676 AC-03 (🧑, Session KB5.6).

## Regeln und Einschränkungen

Protokoll-Version steigt je Änderung; Golden nur mit `task go:golden` samt Begründung; Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Fireball ohne Gegner in Reichweite → `castFailed`, kein Effekt; mit Gegner → Ring am Gegner mit Skill-Radius.

## Ausnahme- und Fehlerfälle

Unbekannter Skill im Ereignis → Client ignoriert ihn.

## Akzeptanzkriterien

- **AC-01** B-674/AC-01 bis AC-03
- **AC-02** B-675/AC-01 bis AC-03
- **AC-03** B-676/AC-01, AC-02

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KB5.1 | `KB5.1-wirkort-ereignis.md` | Umsetzung | autonom | fertig |
| KB5.2 | `KB5.2-status-snapshot.md` | Umsetzung | autonom | fertig |
| KB5.3 | `KB5.3-status-darstellung.md` | Umsetzung | autonom | fertig |
| KB5.4 | `KB5.4-studio-ziel-und-status.md` | Umsetzung | autonom | fertig |
| KB5.5 | `KB5.5-review-kb5.md` | Review | autonom | offen |
| KB5.6 | `KB5.6-abnahme-kb5.md` | Workshop | Mensch | offen |

## Abnahme

–

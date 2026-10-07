# SK1.2 · Skill-Baum Tank und Zauberer: Nachweis mit zwei Linien gleichzeitig

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** offline
- **Branch:** sk1/2-skill-baum-nachweis
- **Abhängig von:** SK1.1
- **Tickets:** B-007
- **Kriterien:** AC-01, AC-03

## Ziel

Ein Sim-Test zeigt das Beispiel aus B-007 (Spieler 1 Taunt, Spieler 2 Fireball, beide wirken im selben Tick); für B-007/AC-01 bis AC-03 steht der Nachweis im Ergebnis, nichts aus S1 wird nachgebaut.

## Kontext

- Beschluss 🧑 2026-10-06: Skill-Tasten (B-026) und Presets (B-017) sind entschieden und archiviert, Grundlage `docs/rules/monarch.md`, S1 und S3. Geplant wird nur, was nach S1 wirklich fehlt.
- Schon da (S1, `docs/sprints/erledigt/S1-monarch-schlag-skills/`, Abnahme 2026-10-04): Fund-Pool, Verteilung je Spieler, Tier-Gating, Respec (`engine/sim/monarch.go`), aktive Skills Tank (`engine/sim/skills_tank.go`: `castTaunt`, `castStun`, `castShield`, `castLastStand`) und Zauberer (`engine/sim/skills_caster.go`: Fireball/Meteor, Ice Wall, Lightning Storm), Werte in `data/monarch.json`, Tests in `engine/sim/skills_*_test.go` und `engine/sim/monarch_test.go`. Spielstand v3 speichert Pool, Verteilung und Skills (S1 AC-05, B-022).
- Lücke: `TestSkillsZweiSpielerGetrennt` (`engine/sim/skills_test.go`) prüft zwei Spieler, aber beide mit Tank-Skills. Ein Test mit zwei Linien gleichzeitig (Beispiel B-007) fehlt.
- B-007/AC-02 (im Client bedienbar) ist CLI und läuft in S3 (`docs/sprints/erledigt/S3-skill-menue-overlay/`, B-124, B-125): AC-01 bis AC-04 dort nachgewiesen, Abnahme am Gerät (S3.4) offen. Diese Session verweist nur darauf, sie ändert keinen Client-Code.
- Hilfen: `mustLearn` (`engine/sim/monarch_test.go`), `tankPlayer` (`engine/sim/skills_test.go`), `casterPlayer`, `toughEnemy`, `lost` (`engine/sim/skills_caster_test.go`).

## Erlaubte Dateien

- `engine/sim/skills_test.go` oder neu `engine/sim/skills_linien_test.go`
- Planungs-Dateien (`docs/sprints/`, `docs/backlog/B-007-*.md`)

## Nicht-Ziele

Neue Skills, Heiler und Dieb, Änderungen an `data/monarch.json`, Client, Protokoll, Spielstand-Format.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Test: zwei Spieler in einer Welt, Spieler 1 lernt Taunt (Tank), Spieler 2 Fireball (Zauberer); beide drücken im selben Tick ihren Slot. Erwartet: Gegner in Taunt-Reichweite zielen auf Spieler 1, der Fireball-Gegner verliert Leben, jede Abklingzeit läuft nur beim eigenen Spieler.
3. Test: Spieler 2 lernt ohne Punkte bzw. ohne Tier → Fehler, `AvailablePoints` und `p.Skills` unverändert (Fehlerfall B-007).
4. Im Ergebnis je B-007-Kriterium den Nachweis eintragen: AC-01 (S1.2a/b + neuer Test), AC-02 (S3, Abnahme S3.4 offen), AC-03 (S1.4, Spielstand v3).
5. `task check:go` grün.

## Fertig, wenn

- [ ] AC-01: `go test ./engine/sim -run Skills` grün, inklusive des neuen Tests mit Tank und Zauberer gleichzeitig.
- [ ] AC-01: Ergebnis nennt für B-007/AC-01, AC-02, AC-03 je einen Nachweis (Session oder Sprint), AC-02 mit Verweis auf S3.4.
- [ ] AC-03: `task check:go` grün, Golden-Daten unverändert.

## Prüfen

```bash
task check:go
```

## Ergebnis

–

# KR4.1 · Workshop: kritische Fragen zu SND entscheiden (🧑)

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** kr4/1-fragen-snd
- **Abhängig von:** –
- **Tickets:** B-554, B-555, B-556, B-557, B-558
- **Kriterien:** AC-01, AC-02

## Ziel

Ralf hat S1–S5 (B-554 bis B-558) entschieden; bestätigte sind erledigt, geänderte haben Folgearbeit im Projekt SND.

## Kontext

Tabelle in `docs/sprints/aktiv/KR4-fragen-snd/README.md` › Anforderungen, Details je Ticket unter Notizen. S1/S2 mit SO4.1 und SO4.5 hören, S4/S5 mit SO6.3 und SO6.6 (Asset-Studio, `soundtest.html`, TV). Nach einer geänderten Reihenfolge der Effekt-Slots `task gen:soundatlas`.

## Erlaubte Dateien

- `docs/backlog/B-554*` … `B-558*`, `B-168*`, `B-492*`, `B-493*`, `B-494*` (über `plan_*`)
- neue Tickets/Sprints im Projekt SND (über `plan_create`)

## Nicht-Ziele

Umsetzung der Folgearbeit; die Hörproben selbst.

## Schritte

1. Ralf vermerkt je Ticket unter Notizen „bestätigt“ oder die Alternative.
2. Claude setzt bestätigte auf `erledigt`, plant geänderte als Ticket/Sprint im Projekt SND; bei B-554 = bestätigt nicht gewählte Musik-Kandidaten entfernen.
3. KR4.1 auf `fertig`.

## Fertig, wenn

- [x] AC-01: Jedes Ticket trägt Ralfs Entscheidung.
- [x] AC-02: Bestätigte erledigt, geänderte mit Folgearbeit.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

- **AC-01 geprüft:** Alle fünf Tickets (B-554 bis B-558) tragen Ralfs Entscheidung.
- **AC-02 geprüft:** B-556, B-557 bestätigt und `erledigt`; B-555 → B-573, B-558 → B-574, B-575, B-576 mit Verweis `erledigt`; B-554 am 2026-10-24 nach der Hörprobe `erledigt` (Kandidaten entfernt).
- Neue Tickets (Projekt SND): B-573, B-574, B-575, B-576.

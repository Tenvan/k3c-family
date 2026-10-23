# KR4.1 · Workshop: kritische Fragen zu SND entscheiden (🧑)

- **Status:** offen
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

- [ ] AC-01: Jedes Ticket trägt Ralfs Entscheidung.
- [ ] AC-02: Bestätigte erledigt, geänderte mit Folgearbeit.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

Teilergebnis (Workshop 2026-10-23 mit Ralf, Claude moderiert und protokolliert); die Session bleibt `offen`, weil das Aufräumen der Musik-Kandidaten (B-554) erst nach der Hörprobe SO4.5 freigegeben wird.

- **AC-01 geprüft:** Alle fünf Tickets (B-554 bis B-558) tragen Ralfs Entscheidung unter Notizen.
- **AC-02 teilweise:** Bestätigt und `erledigt`: B-556 (A), B-557 (A, Atlas bleibt). Geändert und `erledigt` mit Verweis: B-555 (C → B-573, eigenes Niederlage-Stück), B-558 (Zuordnung A bestätigt; Reiten → B-574, UI-Klicks → B-575, Skill je Klasse → B-576). Offen: B-554 (Auswahl bestätigt, Kandidaten bleiben bis SO4.5, `eingeplant`).
- **Neue Tickets (Projekt SND):** B-573, B-574, B-575, B-576 (alle CLI, Spec Entwurf).
- **Offen für Ralf:** Hörproben SO4.5 und SO6.6 am TV, danach Freigabe zum Entfernen der Kandidaten.

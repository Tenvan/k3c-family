# BS2.2 · Regeln Baumschule, Hafen und Schiff, Sprint abschließen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bs2/2-baumschule-hafen-schiff
- **Abhängig von:** BS2.1
- **Tickets:** B-636, B-637, B-638
- **Kriterien:** AC-02, AC-03, AC-04

## Ziel

Baumschule, Hafen und Schiff stehen in Regeln und Glossar; BS2 ist abgeschlossen.

## Kontext

- Farm heute: Glossar „Farm“, „Farm-Weltplatz“, „Plantage“, „Setzling“; `docs/rules/materialien-gebaeude.md` § 1 und § 3 (Plätze ±52, Q51).
- Inselwechsel heute: `docs/rules/stufen.md` § 1 („Boot oder Portal“), `docs/rules/bosse.md`, `docs/rules/welten.md` § 6.
- Geplante Chunks: `docs/rules/geplante-chunks.md` (Chunk-Arten, Plätze Wald ±1…±7).
- Specs: B-636, B-637, B-638 inkl. Defaults der offenen Fragen. Begriff **Schiff** (Boot als Synonym).

## Erlaubte Dateien

- `docs/rules/materialien-gebaeude.md`, `docs/rules/stufen.md`, `docs/rules/bosse.md`, `docs/rules/welten.md`, `docs/rules/geplante-chunks.md`, `docs/rules/wirtschaft.md`, `docs/rules/zielkorridore.md`, `docs/glossar.md`
- `docs/sprints/**/BS2-*/**`, `docs/backlog/B-636-*.md`, `B-637-*.md`, `B-638-*.md` (über Planungs-Tools)

## Nicht-Ziele

Code und Daten.

## Schritte

1. Glossar: Baumschule, Baumschul-Platz, Roden (falls neu), Hafen, Hafen-Chunk, Schiff, Schiff-Abschnitt; Farm-Einträge ersetzen.
2. `materialien-gebaeude.md`: Baumschule im Wald, Freischaltung durch Roden, Plantage.
3. `stufen.md`, `bosse.md`, `welten.md`: Wechsel am Hafen mit fertigem Schiff und besiegtem Endboss; Portal entfällt (Default).
4. `geplante-chunks.md`: Chunk-Art `harbor`, Platz −7 auf Insel 1 Tiefe 0.
5. Schiffskosten je Abschnitt (Münzen + Material) in `wirtschaft.md`, Korridor „Schiff fertig“ in `zielkorridore.md`.
6. Sprint abschließen (Doku-Sprint: Abnahme, `Status: erledigt`).

## Fertig, wenn

- [ ] AC-02: „Farm“ kommt in `docs/rules` nicht mehr vor (`grep -ri farm docs/rules` leer, Archiv ausgenommen).
- [ ] AC-03: Hafen, Schiff und Wechselbedingung in `stufen.md`, `geplante-chunks.md`, `welten.md`, Glossar.
- [ ] AC-04: Schiffskosten und Korridor im Regelwerk.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

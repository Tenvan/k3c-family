# BS2.2 · Regeln Baumschule, Hafen und Schiff, Sprint abschließen

- **Status:** fertig
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

- AC-02 umgesetzt, geprüft: Baumschule in `materialien-gebaeude.md` (§ 1, § 3, neuer § 3.3 Baumschule und Roden), `gegner.md`, `wirtschaft.md`, `geplante-chunks.md`, `zielkorridore.md`. `grep -i farm docs/rules` findet nur noch die Code-ID „bis BS4 `farm`“ und den aus `data/hub.json` erzeugten Block `rules:hub-levels` („Farm −52, +52“); beide folgen mit BS4 (Daten-Umbau, dann `task gen:rules`).
- AC-03 umgesetzt: `stufen.md` § 1 Inselwechsel am Hafen (Schiff fertig + Endboss, alle am Steg, kein Portal), `geplante-chunks.md` § 10 Hafen-Chunk (`harbor`, Tiefe 0, Platz −7), `bosse.md`, `welten.md`.
- AC-04 umgesetzt: Schiffskosten in `wirtschaft.md` § 8 (Kiel 10 + 80 Holz, Rumpf 20 + 60 Stein + 40 Kupfer, Segel 30 + 40 Eisen + 20 Kristall, Startwerte), Korridor „Schiff fertig“ in `zielkorridore.md` § 6.
- Glossar: Baumschule, Baumschul-Platz, Roden, Hafen, Hafen-Chunk, Schiff, Schiff-Abschnitt; Farm, Bauplatz, Holz, Plantage, Endboss, Inselwechsel angepasst.
- Geprüft: `task check:diff` grün (nach `task gen:rules`), `TestGrenzenWieTabelle` grün. Defaults aus B-636/B-637 (Freigabe 🧑 2026-10-26).

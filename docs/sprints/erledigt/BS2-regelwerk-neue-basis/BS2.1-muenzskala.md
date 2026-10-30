# BS2.1 · Regel Münzskala 2–30 und Zielkorridore

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bs2/1-muenzskala
- **Abhängig von:** –
- **Tickets:** B-635
- **Kriterien:** AC-01

## Ziel

`docs/rules/wirtschaft.md` beschreibt die Münzskala 2–30 mit Stufen und Ausnahmen, `docs/rules/zielkorridore.md` ist auf die Skala umgerechnet.

## Kontext

- Ist: Goldpreise in `data/buildings.json` 5–300, `data/hub.json` bis 200, `data/troops.json` bis 50, Bossbeute bis 500, `data/goals.json` 1000, Einnahmen `data/economy.json`.
- Wunsch 🧑: Mauer beginnt bei 2, größte Bauwerke/Ausbaustufen 30; Ausnahmen (manuelle Chunks, Events …) dürfen abweichen und bekommen später eigenes Balancing.
- Offene Fragen und Defaults: B-635 › Offene Fragen. Neue Begriffe (Münzskala) zuerst in `docs/glossar.md`.

## Erlaubte Dateien

- `docs/rules/wirtschaft.md`, `docs/rules/zielkorridore.md`, `docs/glossar.md`
- `docs/backlog/B-635-*.md` (über Planungs-Tools)

## Nicht-Ziele

Werte in `data/` (BS3).

## Schritte

1. Glossar: Münzskala, Ausnahme-Preis.
2. `wirtschaft.md`: Skala, Stufen, Zuordnung der Kategorien (Mauer, Turm, Grundgebäude, Ausbaustufen, Hub-Stufen, Truppen, Markieren), Ausnahmen-Liste, Einnahmen-Verhältnis.
3. `zielkorridore.md`: Gold-Korridore auf die Skala umrechnen, Bedeutung „Lauf läuft durch“ (Meilenstein 1).
4. Defaults offener Fragen als „angenommen (Default B-635)“ markieren.

## Fertig, wenn

- [ ] AC-01: `wirtschaft.md` und `zielkorridore.md` beschreiben Skala, Ausnahmen und umgerechnete Korridore.
- [ ] `tests/planning.test.ts` grün (`task check:diff`).

## Prüfen

```bash
task check:diff
```

## Ergebnis

- AC-01 umgesetzt: `wirtschaft.md` § 8 Münzskala (Stufen 2–30, Kurve `2·(alt/5)^0,66`, Einnahmen, Kleinbeträge, Ausnahme-Preise, Daten-Test-Pflicht); § 1 und § 3 auf die Skala (Start 14, Beutel 30, Morgen 2, Truhen 3–6, Greed 2).
- AC-01 umgesetzt: `zielkorridore.md` Revision 2: Gold am Morgen 5–20, Gold-Bilanz grün −4…10 / gelb −10…16, BR1-Teil als historisch markiert, neuer § 6 Meilenstein-Läufe („durchlaufen“ = Endboss Insel 1 besiegt).
- Glossar: Ausnahme-Preis, Meilenstein-Lauf, Münzskala.
- Geprüft: `task check:diff` grün. Defaults aus B-635 übernommen (Freigabe 🧑 2026-10-26).

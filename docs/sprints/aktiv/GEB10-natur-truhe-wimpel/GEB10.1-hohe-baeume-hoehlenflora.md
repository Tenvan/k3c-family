# GEB10.1 · Bäume höher und dichter, Höhlen-Flora als JSON und PNG

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb10/1-hohe-baeume-hoehlenflora
- **Abhängig von:** –
- **Tickets:** B-454
- **Kriterien:** AC-01

## Ziel

Alle Waldbäume (`tanne`, `pappel`, `birke` je Form 1–3) sind 11,25 Kacheln hoch und 2–4 Kacheln breit, nur noch im Wald-Look; die Höhlen-Flora (`wurzel`, `wurzel-2`, `wurzel-3`, `hoehlenbusch`) liegt als Daten-JSON und PNG vor; die Vorschau zeigt die dichte Baum-Reihe und die Flora.

## Kontext

- **Spec:** GEB10 AC-01 (B-454/AC-01, B-454/AC-02), Entscheidungen 1–3 und 6 unter „Offene Fragen“ der Sprint-README.
- **Heute:** Bäume 1 × 2,25 Kacheln, Form 2 und 3 mit Stufe 2 = Höhle-Look (`tanne-2-2-32.png` …), Regeldatei `docs/standards/gebaeude-grafik.md` § 1, § 5 „Bäume“, § 6. Szenenhöhe: `src/scenes/layout.ts` (`WORLD_TOP` 420), `src/core/constants.ts` (`GROUND_Y` 900) → 15 Kacheln über dem Boden.
- **Werkstatt:** `task gen:gebaeude` rendert alle `public/grafik/gebaeude/*.json` (16/32/64/128 und `index.json`), zweiter Lauf byte-gleich; Bausteine unter 2 px Master brauchen `"fein": true`. Die Rechtecke entstehen mit einer Zeichenhilfe im Scratchpad (nicht im Repo).
- **Vorschau:** `tools/k3c-studio/frontend/src/graphics/scenesLogic.ts` (`BAUM_ABSTAND`, `baumReihe`, `baumDatei`), `BuildingStrip.tsx` (`BaumFigur`), Test `scenesLogic.test.ts`.
- Das Spiel nutzt bis GEB10.3 weiter die alten Dateien (`tanne-2-1-32.png` …), die erst dort gelöscht werden.

## Erlaubte Dateien

- `public/grafik/gebaeude/` (Bäume `tanne*`, `pappel*`, `birke*`, neu `wurzel*`, `hoehlenbusch` mit PNGs, `index.json`)
- `tools/k3c-studio/frontend/src/graphics/` (`scenesLogic.ts`, `BuildingStrip.tsx`, `scenesLogic.test.ts`)
- `tools/buildings/` (nur nötige Erweiterungen, mit Test)
- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Spiel-Zuordnung und Löschen der alten Höhle-Look-PNGs (GEB10.3); mehr Bäume je Abschnitt im Level-Generator (REG); Setzling und Busch bleiben unverändert.

## Schritte

1. Vorlagen der neun Bäume neu schreiben (Maß, Teile), Bausteine neu zeichnen, `stufen` (Höhle-Look) entfernen.
2. `wurzel`, `wurzel-2`, `wurzel-3` (2–3 × 15 Kacheln, Klasse `flora`) und `hoehlenbusch` (1 × 1) mit Vorlage anlegen; Materialien aus § 3.
3. `task gen:gebaeude` zweimal, PNGs ansehen (64 px).
4. Vorschau: Baum-Schritt 1 Kachel, Breite der Bäume beachten, Reihe „Look Höhle“ durch „Höhlen-Flora“ ersetzen; Vitest anpassen.
5. Regeldatei § 1, § 4, § 5 „Bäume“, § 6 nachziehen.

## Fertig, wenn

- [ ] AC-01: neun Bäume 11,25 Kacheln hoch, ohne `stufen`; vier Flora-JSONs mit PNG 16/32/64/128 im `index.json`; zweiter `task gen:gebaeude` byte-gleich.
- [ ] AC-01: Vorschau-Reihe mit Schritt 1 Kachel und Flora-Reihe, Vitest grün (`bunx vitest run src/graphics/scenesLogic.test.ts`).
- [ ] Regeldatei nachgezogen; `task check:diff` grün (oder gezielte Prüfungen, falls nur das Zeitlimit greift).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–

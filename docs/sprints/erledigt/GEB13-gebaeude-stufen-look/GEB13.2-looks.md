# GEB13.2 · Pappel niedrig, Camp-Look, Heilplatz-Stufen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb13/2-looks
- **Abhängig von:** –
- **Tickets:** B-568, B-570, B-571
- **Kriterien:** AC-02, AC-03

## Ziel

Plantage hat eine niedrige Pappel, Camp und Heilplatz zeigen Stufen-Looks.

## Kontext

B-568, B-570, B-571; Generator `task gen:gebaeude`.

## Erlaubte Dateien

- `public/grafik/gebaeude/`, `data/assets.json`, `src/scenes/render/`, `docs/standards/gebaeude-grafik.md`, `docs/assets/zuordnung-welt.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Pappel. 2. Camp. 3. Heilplatz.

## Fertig, wenn

- [x] AC-02, AC-03.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 umgesetzt: `pappel-niedrig`, Tiefe vor der Farm. AC-03 umgesetzt: Camp-Look je Biom (1/2/3), Heilplatz Stufe 2–5 mit Zeltbahnen und Brunnen ab Stufe 3. Geprüft: `gen:gebaeude` (56 Gebäude), Vitest, `tsc`, oxlint, Kontaktbogen angesehen. Annahmen: Camp-Stufe = Biom-Look; Heilplatz-Look eigene Wahl; Pappelhöhe 5 Kacheln geschätzt.

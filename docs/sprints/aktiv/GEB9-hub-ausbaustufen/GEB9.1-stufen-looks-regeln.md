# GEB9.1 · Stufen-Looks und Übergang in der Regeldatei festschreiben

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb9/1-stufen-looks-regeln
- **Abhängig von:** –
- **Tickets:** B-435
- **Kriterien:** AC-01

## Ziel

`docs/standards/gebaeude-grafik.md` § 6 legt die Stufen-Looks der acht Hub-Gebäude fest (Farbtausch, bleibende Holzteile, Zusatz ab Stufe 3) und beschreibt den Übergang (Grundform-Datei bleibt bis GEB9.5). Damit ist AC-01 erfüllt.

## Kontext

- Entscheidungen stehen in der Sprint-README › Offene Fragen (autonom, 2026-10-22): Stufe = Hub-Stufe, Farbtausch auf Stufe 2–5 plus höchstens ein Zusatz je Gebäude ab Stufe 3, Holzteile bleiben Holz, Camp bleibt Grundform, Stufen vor der Freischaltung bleiben erhalten.
- Material je Hub-Stufe: `docs/rules/materialien-gebaeude.md` § 2 (1 Holz, 2 Stein, 3 Kupfer, 4 Eisen, 5 Kristall). Farbziele wie bei Mauer, Turm, Burg: deren `stufen` in `public/grafik/gebaeude/mauer.json`, `turm.json`, `burg.json` lesen und die Farbpaare übernehmen.
- Die Bausteinarten je Gebäude stehen in § 4 der Regeldatei; die Grundformen liegen als `public/grafik/gebaeude/<id>.json` (`werkstatt`, `farm`, `kaserne`, `lager`, `taverne`, `schmiede`, `heilplatz`, `ruestkammer`). Kaserne, Taverne, Schmiede, Rüstkammer haben schon Stein- bzw. Eisenfarben in der Grundform.
- Mechanik: `tools/buildings/stages.go` (`stufen` mit `farben` alt → neu und `zusaetze`; `pruefeStufen` verlangt, dass die alte Farbe in der Grundform vorkommt).
- Übergang: Mit `stufen` schreibt `tools/buildings` heute nur `<id>-<n>-<dichte>.png`; für die GEB9-Gebäude soll zusätzlich `<id>-<dichte>.png` (= Stufe 1) entstehen, bis GEB9.5 das Spiel umstellt.

## Erlaubte Dateien

- `docs/standards/gebaeude-grafik.md`
- `docs/sprints/` (nur Status dieser Session und der Sprint-README, Offene Fragen bleiben)

## Nicht-Ziele

Keine JSONs, PNGs oder Go-Code ändern; keine Änderung an `data/hub.json`, Regeltabelle oder Protokoll; kein Camp-Look.

## Schritte

1. Branch `geb9/1-stufen-looks-regeln` von `sprint/geb9` (der Sprint-Branch trägt die Aktivierung als ersten Commit), `Status: in Arbeit`. § 3 und § 6 der Regeldatei, `materialien-gebaeude.md` § 2 und die `stufen` von Mauer, Turm, Burg lesen.
2. In § 6 einen Unterabschnitt „Hub-Gebäude (GEB9)“ ergänzen: Tabelle Gebäude × Stufe 2–5 mit Farbtausch (alt → neu je Material), bleibenden Holzteilen und dem Zusatz ab Stufe 3 (Art, Lage grob). Die Farbnamen dürfen nur aus DB32 kommen.
3. Den Übergang dokumentieren: `<id>-<dichte>.png` bleibt für die GEB9-Gebäude zusätzlich bis GEB9.5, danach entfällt sie.
4. Camp und Stufen vor der Freischaltung (Entscheidung 4 und 6) in einem Satz vermerken.
5. `task check:diff`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [ ] AC-01: § 6 enthält die Stufen-Looks aller acht Hub-Gebäude (Farbtausch, bleibende Teile, Zusatz ab Stufe 3), den Übergang und die Camp-Entscheidung; jede Farbe ist ein DB32-Name.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 umgesetzt: `docs/standards/gebaeude-grafik.md` § 6 › Hub-Gebäude legt Farbrollen je Material (Holz bis Kristall, aus dem Tausch von Mauer, Turm, Burg abgeleitet), je Gebäude die tauschenden Arten, den Zusatz, die Stein-Grundform-Regel und den Übergang fest. Neu gegenüber der Spec: Farbschlüssel `"art/farbe"` in `stufen.farben`, damit nur Mauerwerk und Dach wechseln und Holzteile gleicher Farbe bleiben; umzusetzen in GEB9.2 (Session-Schritte angepasst). Alle Entscheidungen autonom (Claude), Ralf kann in der Abnahme widersprechen. `task check:diff` grün.

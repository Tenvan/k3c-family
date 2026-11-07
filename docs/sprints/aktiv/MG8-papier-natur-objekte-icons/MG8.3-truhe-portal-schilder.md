# MG8.3 · Truhe, Portal, Ausgang, Abbau-Schilder und Stern als requisite

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** objekte
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** mg8/3-objekte
- **Abhängig von:** RD1, MC1
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Truhe geschlossen (`truhe`) und offen (`truhe-offen`), Portal (`portal`), Ausgang (`ausgang`), Abbau-Schilder (`schild-axt`, `schild-spitzhacke`) und Fähigkeitspunkt-Stern (`stern`) liegen als `requisite` unter `public/grafik/objekte/`.

## Kontext

`docs/assets/zuordnung-welt.md` › Truhen, Portale, Münzen, Ausgang und Baustellen und Abbau-Schilder. Portal und Stern mit `idle` (Drehen/Pulsieren über Winkel), Rest statisch.

## Erlaubte Dateien

- `public/grafik/objekte/**`
- `docs/sprints/aktiv/MG8-papier-natur-objekte-icons/**`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-MG8-Zeilen)

## Nicht-Ziele

Keine Gerüste (Gebäude), kein `assets_style_set`, keine anderen Gruppen.

## Schritte

1. Je Asset `assets_create {gruppe: objekte}` und `assets_set`.
2. Jede Animation mit `assets_preview` prüfen.
3. `task gen:paper`, `assets_review`, `bunx vitest run tools/paper`.
4. Ergebnis, Commit `feat(pap): Truhe, Portal und Schilder (MG8.3)`.

## Fertig, wenn

- [ ] AC-01: Assets gültig.
- [ ] AC-02: Golden eingetragen, Tests grün.
- [ ] AC-03: alle mit `assets_preview` geprüft; Review ohne roten Eintrag.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: truhe, truhe-offen, portal, ausgang, schild-axt, schild-spitzhacke, stern gültig.
- AC-02 geprüft: `task gen:paper`, Golden ergänzt, fremde Gruppen zurückgesetzt; `tools/paper` 187/187 grün.
- AC-03 geprüft: jede `idle` mit `assets_preview` angesehen (Portal-Wirbel dreht 360°, Schilder wackeln, Stern pendelt). Befund: Symbole auf den Abbau-Schildern klein und dunkel; bleibt bis PS1.
- Abweichung: Wirbel des Portals als Kreis, da eine gedrehte Ellipse kippt. Keine neuen Tickets.

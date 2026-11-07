# WB2.1 · Review: feste Schieber, Liste und Detail getrennt scrollbar

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** wb2/1-review-ruhig
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-01, AC-02, AC-06

## Ziel

Die Review-Seite steht ruhig: fester Frame-Player aus `@webkit/molecules`, Liste und Detail im Template `ListDetail` mit je eigenem Scrollbereich.

## Kontext

Review: `tools/k3c-workbench/frontend/src/assets/review/` (`index.tsx` Seite, `Compare.tsx`, `AnimationSync.tsx` Steuerung, `ReviewList.tsx`). Die Steuerung ist eine umbrechende Flex-Zeile; der Text `Frame n/m` und der Knopf Abspielen/Anhalten wechseln die Breite und schieben die Nachbarn. Seiten mit voller Höhe nutzen `calc(100vh - 104px)` (Kopf 61 px + Rand 2 × 20 px, vgl. `styles/mcp.css`). Bausteine: `tools/webkit/{atoms,molecules,templates}`, Aussehen in `tools/webkit/theme.css` (Präfix `wk-`).

## Erlaubte Dateien

- `tools/webkit/molecules/FramePlayer.tsx`, `FramePlayer.test.tsx`, `index.ts`
- `tools/webkit/templates/ListDetail.tsx`, `ListDetail.test.tsx`, `index.ts`
- `tools/webkit/theme.css`, `tools/webkit/codemap.md`
- `tools/k3c-workbench/frontend/src/assets/review/*`
- `tools/k3c-workbench/frontend/src/styles/assets.css`
- `docs/sprints/…/WB2-*`

## Nicht-Ziele

Katalog (WB2.2); Backend.

## Schritte

1. Test (rot): `FramePlayer` hat feste Breiten (Knopf, Schieber, Zähler mit `tabular-nums`, Breite nach Stellen der Frame-Zahl), Abspielen schaltet um.
2. Test (rot): `ListDetail` rendert Liste und Detail in je eigenem Scrollbereich.
3. Bausteine bauen, `AnimationSync` auf `FramePlayer`, `ReviewPage` auf `ListDetail` umstellen; Review-Tests anpassen.
4. Browser: Vite der Workbench, Review öffnen, abspielen und Positionen messen, Liste scrollen.
5. `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01 Review-Schieber fest (Test, Browser-Messung)
- [ ] AC-02 Review getrennt scrollbar (Test, Browser)
- [ ] AC-06 Bausteine über `@webkit`, `task check:diff` grün

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 geprüft: `FramePlayer` (`@webkit/molecules`) mit festen Breiten (Knopf 6.5rem, Schieber 240 px, Zähler in `ch` nach Stellen, `tabular-nums`); Tests `FramePlayer.test.tsx`, `AnimationSync.test.tsx` › Feste Steuerung. Browser (Vite 5196, Fixture-Daten über eine Wails-Attrappe, 60 Einträge): alle 12 Frames gespielt, Knopf, Schieber und Zähler blieben auf 457/573/825 px.
- AC-02 geprüft: `ListDetail` (`@webkit/templates`, `wk-listdetail` in theme.css); Tests `ListDetail.test.tsx`, `ReviewPage.test.tsx`. Browser: Liste auf scrollTop 2000, Detail blieb bei scrollTop 0 und y 197, Seite scrollt nicht.
- AC-06 (Teil): Bausteine über `@webkit`; `task check:diff` grün bis auf `Catalog.test.tsx` › Auswahl zeigt Animationen (lastabhängig, einzeln 5/5 grün, B-704; Katalog in dieser Session unverändert).
- Abweichung: Die Review-Seite nutzt im Browser nicht den Mock (`?mock=1`), weil sie einen eigenen Transport hat (B-703); die Sichtprüfung lief deshalb mit einer Wails-Attrappe im Browser.
- Neue Tickets: keine.

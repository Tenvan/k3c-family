# WB2.2 · Katalog: Lade-Anzeige, feste Schieber, getrennte Scrollbereiche

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** wb2/2-katalog-ruhig
- **Abhängig von:** WB2.1
- **Tickets:** –
- **Kriterien:** AC-03, AC-04, AC-05

## Ziel

Der Katalog zeigt beim Laden eine Lade-Anzeige, nutzt `FramePlayer` und `ListDetail` aus WB2.1 und hält beim Frame-Laden das alte Bild.

## Kontext

Katalog: `tools/k3c-workbench/frontend/src/assets/catalog/` (`CatalogPage.tsx`, `AssetList.tsx`, `Preview.tsx`, `useCatalog.ts`). `useFrame` liefert bei neuem Schlüssel `frame: null`, bis das Bild da ist; die Bühne fällt zusammen und der Schieber springt. `useCatalog` meldet kein Laden. Bausteine aus WB2.1: `@webkit/molecules` › `FramePlayer`, `@webkit/templates` › `ListDetail`.

## Erlaubte Dateien

- `tools/k3c-workbench/frontend/src/assets/catalog/*`
- `tools/k3c-workbench/frontend/src/styles/assets.css`
- `tools/webkit/molecules/LoadingNote.tsx`, `index.ts`, `tools/webkit/theme.css`
- `docs/sprints/…/WB2-*`

## Nicht-Ziele

Backend-Tempo; Styleguide, Sound, Quellen.

## Schritte

1. Test (rot): Lade-Anzeige beim Laden der Liste, bei Asset-Wahl und beim Frame-Laden; altes Bild bleibt stehen.
2. Test (rot): Katalog in `ListDetail`, Vorschau mit `FramePlayer`.
3. Umsetzen.
4. Browser: Katalog öffnen, Gruppen wechseln, abspielen, Liste scrollen.
5. `task check:diff`, Commit.

## Fertig, wenn

- [ ] AC-03 Lade-Anzeige (Test, Browser)
- [ ] AC-04 Katalog getrennt scrollbar (Test, Browser)
- [ ] AC-05 Katalog-Schieber fest (Test, Browser-Messung)
- [ ] `task check:diff` grün

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

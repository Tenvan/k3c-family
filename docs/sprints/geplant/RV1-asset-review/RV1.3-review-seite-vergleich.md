# RV1.3 · ReviewPage: Liste, Vergleich, Overlay, Teil-Markierung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** review
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** rv1/3-review-seite-vergleich
- **Abhängig von:** PV1.8
- **Tickets:** B-699
- **Kriterien:** AC-03, AC-05

## Ziel

Auf `develop` zeigt die Review-Seite im Bereich Assets (`ReviewPage`) alle Asset-Änderungen gegen eine wählbare Git-Basis: Liste mit Status, Vorher/Nachher mit Überblendregler, Pixel-Diff-Overlay mit Boxen und markierten geänderten Teilen; ungültige Specs stehen rot mit Fehlertext in der Liste.

## Kontext

- **Platzhalter:** `tools/k3c-workbench/frontend/src/assets/review/index.tsx` exportiert `ReviewPage` (PV1.8); WB1 hängt sie in die Navigation, der Export-Name bleibt.
- **RPC:** `backend` aus `frontend/src/api` (Wails-Binding bzw. `POST /api/dev/rpc/{Method}`); in `frontend/src/api/wails.ts` sind die Asset-Methoden als `unknown` getippt. Typen deshalb in `assets/review/types.ts` laut `docs/standards/paper-mcp.md` › 4 (`Review`, `ReviewAsset`, `Frame`, `PixelDiff`, `Vorlage`), dünne getypte Wrapper in `assets/review/api.ts`; `wails.ts` bleibt unverändert.
- **Fixtures** für Vitest: `tools/paper/fixtures/ui/AssetsReview.json`, `AssetsFrame.json`, `AssetsPixelDiff.json`, `AssetsGet.json` (relativer Import aus dem Test, notfalls `readFileSync`). Muster für API-Mocks: `vi.mock('../../api', () => ({ backend: fake }))` wie `src/balancing/useBalancing.test.tsx`.
- **Bausteine:** nur `@webkit/*` (Atomic Design, `tests/atomicDesign.test.ts`); Schieber = `ValueSlider` aus `@webkit/atoms` (PV1.8, `aria-label` Pflicht).
- **Fachliches:** Vorher = `AssetsFrame(id, anim, 0, '', basis)`, Nachher = `AssetsFrame(id, anim, 0, '', '')`; `neu` hat nur Nachher, `geloescht` nur Vorher. Animationen aus `AssetsGet(id).animationen` (Schlüssel in Datei-Reihenfolge), bei `geloescht` leere Animation (= erste, RV1.2). Pixel-Boxen aus `AssetsPixelDiff` (`[x, y, b, h]` in Frame-Pixeln, prozentual über das Bild legen). Teil-Markierung = Ids aus `ReviewAsset.teile`, die in `Frame.teile` eine Box haben. Der Umhang ist kein Teil: Er erscheint über die Pixel-Diff-Box (Fixture `[56, 56, 48, 32]`).
- Basis-Auswahl: `HEAD` (Standard), `HEAD~1`, `origin/develop`, freies Feld für Commit-SHA; Server-Fehlertext wird unverändert angezeigt; bei `basisStand: lokal` Hinweis „Basis-Stand: lokal (ohne Fetch)“.

## Erlaubte Dateien

- `tools/k3c-workbench/frontend/src/assets/review/`

## Nicht-Ziele

- Keine Navigation, kein Katalog, keine Prompt-Buttons am Asset oder Teil (WB1).
- Keine Animation und keine Prompt-Buttons am Review-Eintrag (RV1.4).
- Kein Ändern von Assets aus der Oberfläche (Vertrag: nur MCP-Tools ändern).

## Schritte

1. **Rot** (`ReviewPage.test.tsx`, `backend` gemockt mit den Fixtures): `listet alle Einträge mit Status` – monarch, pferd, fass, banner, mauer mit Status-Badge, mauer mit Badge „Stil“, Kopfzeile nennt Basis `HEAD~1` und `32eb7a7`; `ungültige Spec rot mit Fehlertext` – banner mit Farbe rot und Text „Material „scharlach“ fehlt in der Stil-Datei“, die vier anderen bleiben sichtbar (AC-05); `Basis wählen ruft AssetsReview` – Auswahl `HEAD~1`, `origin/develop`, eingetippte SHA → Aufruf mit genau dem Wert; `Fehler der Basis sichtbar` – Mock wirft den Vertragstext → Text erscheint, Liste leer; `Basis-Stand lokal` – Hinweis sichtbar.
2. **Rot** (`Compare.test.tsx`): `Vorher und Nachher` – Auswahl monarch → `AssetsFrame('monarch', 'idle', 0, '', 'HEAD~1')` und `…, '')`, zwei Bilder mit `src` = Fixture-`bild`; `neu nur Nachher`, `gelöscht nur Vorher`; `Überblendregler` – `ValueSlider` „Überblendung“ auf 25 → Nachher-Bild mit `opacity: 0.25`; `Pixel-Diff-Overlay mit Boxen` – Schalter „Unterschiede“ → Overlay-Bild mit `src` = Fixture-`overlay`, je Eintrag von `boxen` ein Element `data-box` mit `left/top/width/height` in Prozent (56/209, 56/192 …); `markiert geänderte Teile` – banner mit `teile: ['tuch']` und Frame-Mock mit Box für `tuch` → Element `data-teil="tuch"`; monarch ohne Teil-Markierung (AC-03).
3. **Grün:** `types.ts`, `api.ts`, `ReviewPage` (index.tsx) mit `BasisSelect.tsx`, `ReviewList.tsx`, `Compare.tsx`, `Boxes.tsx`; Laden und Fehlerzustände je Aufruf; Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen.
4. Vitest grün, Commit `feat(dev): Review-Seite mit Vergleich und Overlay (RV1.3)`.

## Fertig, wenn

- [ ] AC-03: `ReviewPage.test.tsx` und `Compare.test.tsx` grün (Vorher/Nachher, Überblendregler, Overlay mit Boxen, Teil-Markierung).
- [ ] AC-05: Test `ungültige Spec rot mit Fehlertext` grün.
- [ ] `tests/atomicDesign.test.ts` grün (`task test -- atomicDesign`).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task k3c-workbench:vitest
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf). Code-Review vor dem Commit ab etwa 150 geänderten Zeilen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

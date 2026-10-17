# WZ6.3 · Seite Grafik mit Asset-Vorschau und Grafik-Packs im Asset-Studio

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz6/3-seite-grafik
- **Abhängig von:** WZ6.2
- **Tickets:** B-459
- **Kriterien:** AC-02, AC-06, AC-07

## Ziel

Die Seite Grafik des Asset-Studios kann alles, was die Workbench-Assets-Seite für Grafik kann, zeigt die Grafik-Mini-Szenen von `assetvorschau.html` (Figuren, Gebäude mit Stufenreihen und Gebäude-Vorlagen, Umgebung mit Hintergründen, Objekte, HUD) und die Pack-Übersicht von `grafiken.html` und lädt bei Änderung des Slot-Registers neu, ohne offene Eingaben zu verlieren. Die gemeinsame Seitenstruktur, auf der WZ6.4 den Sound-Teil baut, liegt danach im Studio-Frontend.

## Kontext

Umgebung `live`: Fenster (`task k3c-studio`), Mock im Browser und Screenshot. **Browser-/Fensterprüfung nur mit Freigabe 🧑** für diesen Lauf; ohne Freigabe gelten Tests und Build als Nachweis, die Sichtprüfung folgt in WZ6.8.

### Stand nach WZ6.2

- Wails-App `tools/k3c-studio` (Frontend Vite/React/Radix unter `frontend/`) mit leerer Navigation Grafik/Sound; Bindings auf das gemeinsame Go-Modul `tools/assetkit` mit den Namen aus `tools/k3c-dev/app_assets.go` (`AssetsList`, `AssetsSet`, `AssetsCatalog`, `AssetsHits`, `AssetsResearch`, `AssetsHitDrop`, `AssetsImport`, `AssetsDownload`, `AssetsLicenseOK`).
- Der Wails-Asset-Handler liefert Dateien aus `public/` unter der Seitenwurzel (`grafik/…`, `sprites/…`, `audio/…`).
- Das Backend beobachtet `data/assets.json` per Poll und schickt ein Neuladen-Ereignis ans Frontend; `AssetsSet` lehnt Speichern auf veraltetem Stand ab (Schreibregel). Ereignisname und Fehlerkennung stehen in WZ6.2 › Ergebnis.
- Prüfziel des Asset-Studios in `Taskfile.yml` (Name laut WZ6.2 › Ergebnis), `check:all` nimmt es mit.

### Gemeinsame Seitenstruktur (Quelle Workbench, Stand nach #333)

`tools/k3c-dev/frontend/src/assets/`: `AssetsPage.tsx`, `SlotDetail.tsx`, `CatalogPicker.tsx`, `ResearchPanel.tsx`, `CutEditor.tsx`, `MiniScene.tsx`, `assets.ts` (Test `assets.test.ts`), `useAssets.ts`, dazu `styles/assets.css`. WZ6.3 übernimmt diese Struktur vollständig, weil Grafik und Sound sie teilen; WZ6.4 baut nur den Sound-Teil darauf. Im Studio entfallen die Vite-Adresse `preview`, `startVite` und der Knopf „Vite starten (Bilder)“; `fileUrl` liefert den Pfad relativ zur Seitenwurzel.

Abhängigkeiten außerhalb von `assets/` werden **kopiert** (zwei getrennte Wails-Frontends; ein gemeinsames UI-Paket wäre ein eigenes Ticket):

| Quelle (`tools/k3c-dev/frontend/src/`) | Ziel im Studio-Frontend | Umfang |
|---|---|---|
| `tasks/TaskBars.tsx` | `ui/Splitter.tsx` | nur `Splitter`, `useSplit`, `splitShare`, `clampSplit`, `parseSplit` (ohne `TasksHead`, `TasksFoot`) |
| `lib/prefs.ts`, `lib/errors.ts`, `lib/useDebounced.ts` | `lib/` | ganz |
| `ui/parts.tsx` | `ui/parts.tsx` | `ActionButton`, `StatusBadge`, `NoticeCard`, `Tone`; `Tip` nur, wenn genutzt |
| `styles/theme.css`, `styles/assets.css` | `styles/` | ganz, `assets.css` ohne ungenutzte Klassen |
| `api/assetTypes.ts`, `api/mockAssets.ts`, Asset-Teile von `api/types.ts`, `api/wails.ts`, `api/mock.ts` | `api/` aus WZ6.2 | nur Asset-Bindings; was WZ6.2 schon angelegt hat, ergänzen statt doppeln |

### Grafik-Inhalt von `assetvorschau.html`

`src/tools/assetvorschau.ts` (329 Zeilen DOM-Code) und `assetvorschauLogic.ts` (Test `assetvorschauLogic.test.ts`). `SCENES`: Grafik `figuren`, `gebaeude`, `umgebung`, `objekte`, `hud`; Sound `sfx`, `musik`, `ambient` (WZ6.4).

- **Figuren:** `figureCard` → `renderReference` aus `spriteReference.ts`, alle im selben Maßstab.
- **Gebäude:** Streifen „Neue Gebäude (GEB)“ aus `grafik/gebaeude/index.json` vor den Slots: Baum-Reihe (`baumReihe`, `baumDatei`; 32 px und 16 px, dazu Look Höhle Stufe 2), je Gebäude 32 px ×1 neben 16 px ×2 (`gebaeudeKarten`), kachelbare Grafik dreifach (`kachelAnzahl`), Stufenreihe (`stufenKarten` mit Änderungstext), Gebäude-Vorlage aufklappbar (`vorlageZeilen`); alles auf der Bodenlinie des höchsten Gebäudes.
- **Umgebung:** Slots und Streifen „Hintergründe“ (`hintergrundStreifen` mit `BIOMES`, GEB7): je Biom die drei Hintergrund-Schichten fern → nah auf `palette.sky`, zwei Kacheln nebeneinander, 32 px ×1 und 16 px ×2. Heute hängt der Streifen an der Szene `gebaeude`; B-459/AC-06 ordnet ihn der Umgebung zu, im Studio steht er dort.
- **Objekte, HUD:** Bildkarten mit Ausschnitt (`resolveSlot`, `GRAFIK_SCALE`), NC-Kennzeichen (`licenseFlag` mit `CREDITS`), Lücke und fehlende Datei markiert.
- **Vergleich:** Präferenz `wahl=n` eines Slots statt Präferenz 1 zeigen.

Maßstab und Anordnung laut `docs/standards/gebaeude-grafik.md` § 9 Punkte 3–4; das Studio zeigt genau das. Den Verweis dort ändert WZ6.6.

### Inhalt von `grafiken.html`

`src/tools/grafiken.ts`: Pack-Übersicht aus `GRAFIK_PACKS` (`grafikPacks.ts`) und die Bilder aus `grafik/index.json` (`grafik/<pack>/<datei>`). Pad-Scroll und Auswahl (`installPadScroll`, `selection.ts`) sind Xbox-Bedienung und entfallen. `grafiken.html` bleibt für die Xbox unverändert.

### Spiel-Code nur lesend über Alias

Aliase im Studio-Frontend: `@game` → `src/`, `@data` → `data/` (von `tools/k3c-studio/frontend/` aus `../../../src`, `../../../data`), in `vite.config.ts` (`resolve.alias`) und `tsconfig.json` (`paths`).

| Import in `assetvorschau*`/`grafiken` | im Studio |
|---|---|
| `src/tools/spriteReference` (`renderReference`, `Card`) | `@game/tools/spriteReference` |
| `src/model/biome` (`BIOMES`) | `@game/model/biome` |
| `src/tools/credits` (`CREDITS`) | `@game/tools/credits` |
| `src/core/constants` (`UNIT_PX`) | `@game/core/constants` |
| `src/tools/grafikPacks` (`GRAFIK_PACKS`) | `@game/tools/grafikPacks` |
| `data/sprites.json` (Logik) | `@data/sprites.json` |
| `src/audio/audioCore`, `crossfade`, `events` | erst WZ6.4 |
| `data/assets.json` | **nicht** importieren: Slots kommen aus `AssetsList`, sonst greift das Neuladen (AC-07) nicht |
| `src/tools/texts` (`t`, `textOf`, `applyTexts`, Schlüssel `av.*`), `src/core/shell`, `src/core/clientLog` | nicht übernehmen: `av.*` entfallen in WZ6.6; Beschriftungen deutsch im Studio wie in k3c-dev, Logs nach Emoji-Regel im Studio |

Logik-Umzug: `assetvorschauLogic.ts` und `assetvorschauLogic.test.ts` als Kopie ins Studio-Frontend (z. B. `src/grafik/szenenLogic.ts`), Importe auf `@game`/`@data`, `parseQuery` nur, wenn gebraucht. `src/` bleibt unverändert; WZ6.6 löscht die Originale.

### Fallstricke

- Das Root-Vitest (`task test`) sammelt alle `*.test.ts` des Repos, kennt die Aliase aber nicht. Darum `tools/k3c-studio/**` in `vite.config.ts` der Wurzel aus `test.exclude` nehmen (wie `tools/cockpit/**`) und die Studio-Tests im Prüfziel des Studios mit dessen Vite-Konfiguration fahren.
- Der Vite-Dev-Server liefert keine Dateien außerhalb seiner Wurzel: `server.fs.allow` auf die Repo-Wurzel setzen.
- `paths` zieht die importierten `src/`-Dateien in den Typecheck des Studios: `resolveJsonModule`, `vite/client` (für `?raw` in `credits.ts`).
- `spriteReference.ts` → `texts.ts` → `core/texts.ts` → `clientLog` (POST `/api/clientlog`): Im Studio gibt es keinen Spielserver, Fehler werden geschluckt (ungeprüft, im Mock beobachten).
- Bilder liegen relativ zur Seitenwurzel. In `wails dev` und im Mock liefert sie der Studio-Vite aus `public/` nur im Dev-Modus (z. B. `publicDir` nur bei `serve`, Vorschlag); `public/` darf nicht nach `dist/` und damit in die EXE, die EXE nutzt den Asset-Handler aus WZ6.2.
- Neuladen (AC-07): Ereignis → `AssetsList` neu, nur `view` ersetzen. Die Keys bleiben stabil (`SlotDetail` per Slot-ID, `CutEditor` per aktiver Präferenz und Datei), damit Auswahl (Slot, aktive Präferenz), Filter (`lib/prefs`), Ausschnitt-Entwurf, Such- und Recherche-Eingaben erhalten bleiben; fehlt der gewählte Slot danach, Hinweis statt leerer Seite; Mini-Szenen zeichnen neu. Abgelehntes Speichern: neu laden, Meldung „Register geändert, bitte erneut speichern“, Eingaben bleiben.
- Komplexitäts-Budget: `assetvorschau.ts` in Komponenten je Szene aufteilen (Datei ≤ 400, Funktion ≤ 60 Zeilen).

## Erlaubte Dateien

- `tools/k3c-studio/frontend/**` (Seite Grafik, gemeinsame Seitenstruktur, kopierte Teile, umgezogene Logik, Tests, Codemaps, `vite.config.ts`, `tsconfig.json`; `package.json` ohne neue Abhängigkeit)
- `tools/assetkit/…` nur Ergänzungen, falls ein Binding fehlt (mit Go-Test), und dafür `tools/k3c-studio/*.go`
- `vite.config.ts` (Wurzel): nur `test.exclude` um `tools/k3c-studio/**` (Grenzfall wie die Lint-Konfiguration, B-459 › Regeln)
- `.oxlintrc.json`: nur `ignorePatterns` um `tools/k3c-studio/frontend/dist` und `tools/k3c-studio/frontend/wailsjs`, falls WZ6.2 sie nicht gesetzt hat
- `Taskfile.yml`: nur das Prüfziel des Asset-Studios (Vitest mit der Studio-Konfiguration)

Nur lesen: `src/**`, `data/**`, `public/**`, `tools/k3c-dev/frontend/src/**`, `assetvorschau.html`, `grafiken.html`.

## Nicht-Ziele

Sound-Teil: Hörprobe, Szenen `sfx`, `musik`, `ambient`, Inhalt von `soundtest.html` (WZ6.4); Assets-Seite der Workbench entfernen (WZ6.5); `assetvorschau.html`, `src/tools/assetvorschau*.ts`, Kachel und Texte `av.*` entfernen (WZ6.6); Änderungen an `src/`, `data/`, `public/` und `grafiken.html`; Pad-Bedienung; neue Asset-Funktionen (B-460–B-464); Format von `data/assets.json`; gemeinsames UI-Paket für k3c-dev und Asset-Studio.

## Schritte

1. Einbindung: Aliase `@game`/`@data` in `vite.config.ts` und `tsconfig.json` des Studio-Frontends, `server.fs.allow`, Vitest-Konfiguration des Studios; Wurzel-`vite.config.ts` nimmt `tools/k3c-studio/**` aus, das Prüfziel des Studios fährt Vitest mit der Studio-Konfiguration; Oxlint-Ignore für `dist`/`wailsjs`. Ein Test, der `@game/core/constants` importiert, läuft im Prüfziel grün, `task check` bleibt grün.
2. Gemeinsame Seitenstruktur aus `tools/k3c-dev/frontend/src/assets/` samt `assets.test.ts` und die Abhängigkeiten laut Tabelle kopieren; `preview`/`startVite` entfernen, `fileUrl` relativ; Bindings auf das Studio-`api/` (Wails und Mock).
3. Seite Grafik in der Navigation (Art fest `grafik`, kein Umschalter): Slots mit Filtern (Kategorie, Typ, Status, Suche), Präferenzen, Mini-Szene, Katalog, Recherche, Ausschnitt-Editor, Import.
4. Neuladen (AC-07): Ereignis aus WZ6.2 abonnieren, `view` ersetzen ohne Verlust von Auswahl, Filter und Entwurf; abgelehntes Speichern → neu laden und Meldung; die Entscheidungen dafür als reine Funktionen in `assets.ts` mit Tests (z. B. Auswahl halten, Ablehnung erkennen).
5. `assetvorschauLogic.ts` samt Test ins Studio-Frontend kopieren, Importe auf `@game`/`@data`.
6. Reiter „Szenen“: Mini-Szenen Figuren, Gebäude, Umgebung, Objekte, HUD als React-Komponenten mit dem Inhalt aus dem Kontext (Stufenreihen, Gebäude-Vorlagen, Hintergründe, Vergleich `wahl=n`); Abgleich mit `docs/standards/gebaeude-grafik.md` § 9 Punkte 3–4.
7. Reiter „Packs“: Pack-Übersicht aus `GRAFIK_PACKS` und Bilder aus `grafik/index.json`.
8. Codemaps des Studio-Frontends nachziehen; Prüfziel, `task check`, `task k3c-studio:build`; mit Freigabe 🧑 Mock im Browser oder Fenster und Screenshot ins Ergebnis.

## Fertig, wenn

- [x] AC-02: Seite Grafik mit Slots, Filtern (Kategorie, Typ, Status, Suche), Präferenzen (umordnen, hinzufügen, entfernen), Mini-Szene, Katalog, Recherche, Ausschnitt-Editor und Import; kopierter `assets.test.ts` im Prüfziel des Studios grün.
- [x] AC-06: Reiter „Szenen“ zeigt Figuren (`renderReference`), Gebäude (Baum-Reihe, 32 px ×1 neben 16 px ×2, Stufenreihen, Gebäude-Vorlagen), Umgebung mit Hintergründen (je Biom fern → nah, 32 px ×1 und 16 px ×2), Objekte und HUD; Reiter „Packs“ zeigt `GRAFIK_PACKS` und die Bilder; kopierter Logik-Test grün; Screenshot im Ergebnis, falls 🧑 die Sichtprüfung freigegeben hat (sonst Nachweis in WZ6.8).
- [x] AC-07: Test: Neuladen hält Auswahl, Filter und Ausschnitt-Entwurf; Test: abgelehntes Speichern führt zu „Register geändert, bitte erneut speichern“, die Eingaben bleiben.
- [x] `git diff --name-only` der Session zeigt nichts unter `src/`, `data/`, `public/` und weder `grafiken.html` noch `assetvorschau.html`.
- [ ] Prüfziel des Asset-Studios, `task check` und `task k3c-studio:build` grün (Oxlint: Datei ≤ 400, Funktion ≤ 60 Zeilen): `check:studio` und `k3c-studio:build` grün, Oxlint ohne Befund im Studio; `task check` rot nur durch den vorbestehenden Lint-Fehler auf `develop` (B-465), Typecheck und Tests grün.

## Prüfen

```bash
task check
task check:studio      # Prüfziel des Asset-Studios, Name laut WZ6.2 › Ergebnis
task k3c-studio:build
```

Browser-/Fensterprüfung nur mit Freigabe 🧑 für diesen Lauf: Mock des Studio-Frontends im Browser-Pane oder `task k3c-studio`, Screenshot der Reiter Slots, Szenen (Gebäude, Umgebung) und Packs. Xbox und TV entfallen.

## Ergebnis

- **AC-02 umgesetzt, geprüft:** Seitenstruktur aus `tools/k3c-dev/frontend/src/assets/` nach `tools/k3c-studio/frontend/src/assets/` kopiert (Art fest je Seite, ohne Vite-Adresse und „Vite starten“, `fileUrl` relativ); Abhängigkeiten laut Tabelle kopiert (`lib/`, `ui/parts.tsx` ohne `Tip`, `ui/Splitter.tsx`, `styles/`), `api/` mit Wails-Bindings und Mock. `assets.test.ts` grün in `task check:studio`. Fenster (EXE, echte Daten): 119 Slots mit Filtern, Slot `farm` mit Präferenz, Ausschnitt-Editor und Mini-Szene.
- **AC-06 umgesetzt, geprüft:** Reiter Szenen (Figuren über `renderReference`, Gebäude mit Baum-Reihe, 32/16 px, Stufenreihen, Vorlagen; Umgebung mit Hintergründen je Biom; Objekte; HUD; Vergleich Präferenz n) und Packs (`GRAFIK_PACKS`, `grafik/index.json`, Zoom). `szenenLogic.ts` (Kopie von `assetvorschauLogic.ts`, ohne `parseQuery`) mit Test grün. Sichtprüfung im Fenster `bin/k3c-studio.exe` mit Freigabe 🧑 (2026-10-17, Chat): Screenshots Slots, Slot farm, Figuren, Gebäude, Umgebung, Hintergründe, Packs im Notizordner der Session-Gruppe (`side-session-notes/…/wz6.3-screenshots/`). Bilder kommen über den Asset-Handler.
- **AC-07 umgesetzt, geprüft:** `useAssets` lädt bei `assets:changed` neu und ersetzt nur `view`; ein kurz ungültiges Register lässt die letzten gültigen Daten stehen (Meldung statt leerer Seite); Speichern mit Stand, Ablehnung → neu laden, Meldung „Register geändert, bitte erneut speichern“. `reload.test.tsx` (happy-dom, Mock als Fremdschreiber): Auswahl, Suchfilter und Ausschnitt-Entwurf bleiben beim Neuladen; abgelehntes Speichern meldet, hält die Eingaben, das zweite Speichern gelingt. Unit-Tests für `applySaved`, `selectionOf` (verschwundener Slot → Hinweis) und `isStale` (Text von Wails, Error vom Mock).
- **Prüfung:** `task check:studio` grün (42 Tests, Go-Tests, Lint), `task k3c-studio:build` grün (`public/` nicht in `dist/`), `check_run task:test` und `task:typecheck` grün; `task:lint` rot nur durch B-465 (vorbestehend, deshalb letztes Kästchen offen). Die EXE lauscht auf keinem Port.
- **Review (getrennter Reviewer):** keine schweren Befunde; behoben: fehlende Tests der reinen Funktionen, Seite bleibt bei ungültigem Zwischenstand, Meldungen werden beim Neuladen geräumt, Rückweg im STALE-Test, Import `./paths.ts`. Offen gelassen: Reihenfolgeschutz paralleler Neuladen (heilt sich selbst), Filter Kategorie/Typ/Status gehen beim Reiterwechsel verloren (Neuladen hält sie), `studio:frontend`-`sources` ohne Alias-Eingaben (lokaler Cache; CI baut immer), Abbruch älterer `draw`-Aufrufe in `pixel.tsx`.
- **Abweichungen:** Gebäude-Bodenlinie ohne Hintergrund-Schichten (in `assetvorschau.html` schoben die 15 Kacheln hohen Schichten seit GEB7 alles nach unten). CI-Job `k3c-studio` um den Studio-Vitest ergänzt (`.github/workflows/ci.yml`, nicht in den erlaubten Dateien; sonst liefe der Komponententest nach dem Ausschluss aus dem Wurzel-Vitest nicht mehr in der CI, AC-04). Studio-`tsconfig` mit Typen `node`; `vitest`, `happy-dom` und `@types/node` kommen aus den `node_modules` der Wurzel (`task install`). `PacksView` nutzt `textOf` aus `@game/tools/texts` für die Gruppennamen `grafik.group.*` (bleiben für `grafiken.html`).
- **Geprüft von:** Claude (autonom, Fenster mit Freigabe 🧑). **Neue Tickets:** B-471 (Abmeldung der Animationsschleife in `renderReference`).

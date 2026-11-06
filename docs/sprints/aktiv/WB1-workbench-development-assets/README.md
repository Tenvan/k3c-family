# WB1 · DEV, INF · Workbench mit Bereichen Development und Assets, Studio-Programm abgebaut

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** DEV, INF
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** 8b0e2f18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: „alles bis zur Abnahme abarbeiten“; Defaults in B-701)

## Ausgangslage

Welle 1, Lane **WB1** (DEV, Lane `ui`, parallel zu RD1, MC1, RV1). Die Workbench (`tools/k3c-workbench/frontend/src`) hat heute die Seiten Dienste & Logs, Tasks, Planung, Git, MCP, Monitoring, Balancing (`Header.tsx` › `PAGES`) und in der Kopfzeile den Knopf „Asset-Studio öffnen“ (`StudioLink.tsx` → Binding `StudioOpen` in `app_studio.go`, startet `bin/k3c-studio.exe`). Das Studio (`tools/k3c-studio/`, rund 30 000 Zeilen Go und TypeScript) ist ein eigenes Wails-Programm mit Backend auf Port 8091, Chat, Welten-Editor, Gebäude-Planer, Asset-Zuordnung, eingebundener Engine und Testspielständen. Die RPC-Methoden der Assets (`AssetsList`, `AssetsGet`, `AssetsStyle`, `AssetsPrompts`, `AssetsReview`, `AssetsFrame`, `AssetsPixelDiff`) und der Platzhalter `ReviewPage` liegen seit PV1.8 als Stubs vor (`internal/assetsui/`, Fixtures `tools/paper/fixtures/ui/`, im Frontend noch als `unknown` typisiert); RV1 füllt sie.

## Ziel

Die Workbench hat die Bereiche **Development** (alle heutigen Seiten) und **Assets** (Review, Katalog, Styleguide, Sound, Quellen) ohne schreibende Controls, mit Prompt-Buttons; das Studio-Programm samt Chat ist abgebaut.

Am Ende sichtbar: `task k3c-workbench:web` zeigt beide Bereiche; `bin/k3c-studio*.exe` entstehen nicht mehr.

## Beteiligte und Zielgruppen

- **Ralf (🧑):** sieht und prüft Assets in der Workbench statt im Studio, kopiert Prompts für Claude; nimmt den Sprint per PR ab.
- **Claude (Agenten):** ändern Assets nur über MCP `assets_*` (MC1); die Workbench liefert ihnen die Prompts.
- **RV1:** baut die Review-Seite im Bereich Assets und nutzt `assets/api.ts` und `assets/prompts/fill.ts` aus WB1 (D-02).

## Anforderungen

Entscheidung 005 › 5; `docs/arbeitsweise.md` › Oberflächen der Wails-Programme (Atomic Design über `@webkit`).

## Nicht-Ziele

Review-Seite selbst (RV1, eigener Ordner `frontend/src/assets/review/`), MCP (MC1).

## Regeln und Einschränkungen

DEV-Sessions nur `tools/k3c-workbench/frontend/` (außer `src/assets/review/`), `tools/k3c-workbench/internal/studioview/` (neu, lesende Studio-Teile inkl. „Im Spiel ansehen“), `tools/k3c-workbench/app_studio.go`, `tools/k3c-studio/` (löschen); dazu in WB1.4 `tools/k3c-workbench/uiapi.go` (nur `uiMethods`), `tools/k3c-workbench/go.mod`/`go.sum` und `tools/assetkit/writepaths_test.go` (D-01). Die INF-Session WB1.5 ändert nur `.github/workflows/ci.yml`, `Taskfile.yml`, `tools/k3c-studio/Taskfile.yml`, `vite.config.ts`, `tests/atomicDesign.test.ts`, `tests/offlineTools.test.ts`, `tests/projectRules.test.ts`, `requirements.md`, `docs/glossar.md` (Eintrag Asset-Studio), `docs/arbeitsweise.md` (DEV-Zeile), `CLAUDE.md`, dazu `.claude/launch.json`, `.oxlintrc.json`, `tools/k3c-workbench/services.json`, `tools/k3c-workbench/internal/services/config_test.go`, `tools/k3c-workbench/task-freigaben.json` (D-07). **Schreibendes Control** = Control, das Dateien, Specs oder Zustände ändert; Filter, Auswahl, Schieberegler zum Ansehen sind erlaubt. „In Claude öffnen“ bleibt (B-701 D-12). **Reihenfolge:** WB1.5 vor WB1.4 (sonst bricht `task` am fehlenden Include `tools/k3c-studio/Taskfile.yml`). Tests zuerst (Vitest, Go), Oberfläche nur über `@webkit` (Atomic Design).

## Beispiele

- Ralf wählt Assets › Katalog › `monarch` → Vorschau der Animation `idle` läuft, Abzeichen „im Diff“, Buttons „Material ändern“ … kopieren einen Prompt, der mit „Rufe zuerst `assets_guide {rig: figur}` auf“ beginnt und `public/grafik/figuren/monarch.paper.json` nennt.
- Klick auf den Kopf in der Vorschau → Teil `kopf` umrandet, Button „Teil-Material“ kopiert einen Prompt mit `kopf`.
- Assets › Katalog › „Im Spiel ansehen“ → Browser öffnet das Spiel mit voll ausgebautem Testspielstand auf `127.0.0.1:<freier Port>`.

## Ausnahme- und Fehlerfälle

- Ungültige Spec (`banner`, Material „scharlach“ fehlt) → roter Eintrag mit `feld · regel · text`, die übrigen Assets zeigen weiter Vorschauen (005 › 7).
- `AssetsFrame` für ein Asset ohne Daten (Stub kennt nur `monarch`) → Fehlertext in der Vorschau, Liste bleibt bedienbar.
- Vorlage mit fehlendem Pflichtwert (`{teil}` ohne gewähltes Teil) → kein Button bzw. Fehler, nie ein halb gefüllter Prompt.
- „Im Spiel ansehen“ ohne gebauten Client (`dist/index.html` fehlt) → Hinweis `task build`.

## Akzeptanzkriterien

- **AC-01** Kopfleiste mit Development und Assets; alle bisherigen Seiten unter Development erreichbar (Vitest) (005/AC-07).
- **AC-02** Assets: Katalog (Animations-Vorschau, Abzeichen „im Diff“, **roter Eintrag** für ungültige Specs mit Fehlertext), Styleguide (Materialtafel), Sound, Quellen; kein schreibendes Control (Test) (005/AC-07, §7).
- **AC-03** Prompt-Buttons am Asset und am angeklickten Teil kopieren den gefüllten Prompt aus `AssetsPrompts` (Test der Platzhalter-Ersetzung) (005/AC-09).
- **AC-04** `tools/k3c-studio/` samt Chat, EXEs, Ports 8091/8182 und Studio-Tasks entfernt; `task buildall` grün (005/AC-10).

## Offene Fragen

Keine offenen. Entschieden mit Default (Planung WB1, 2026-11-06, Claude; Einspruch durch 🧑 jederzeit):

- **D-01** WB1.4 braucht über die Lane-Liste hinaus `uiapi.go` (nur `uiMethods`: `StudioOpen` raus, Studio-Lesemethoden rein), `go.mod`/`go.sum` der Workbench und `tools/assetkit/writepaths_test.go` (liest heute `../k3c-studio`).
- **D-02** Typen und Zugriff der sieben Asset-Methoden legt WB1.1 in `frontend/src/assets/api.ts` an (Mock = Fixtures), die Platzhalter-Ersetzung WB1.3 in `frontend/src/assets/prompts/fill.ts` (alle sieben Platzhalter, auch `{basis}`, `{eingabe}`); RV1 importiert beide nur, Änderungen per Ticket an WB1. `wails.ts` gehört WB1 (Ordner `frontend/` außer `review/`).
- **D-03** Sound und Quellen sind reine Listen: Sound = Sound-Slots aus `data/assets.json` mit Status und Hörprobe (Datei als Data-URL über `StudioSoundData`, nur unter `public/`); Quellen = Katalog (`assetkit.Catalog`, Lizenz-Marke) und Recherche-Treffer je Slot. Recherche, Download, Import und Zuordnen bleiben MCP (`assets_research`, `assets_hits`, `assets_import`).
- **D-04** „Im Spiel ansehen“: `internal/studioview` startet die eingebundene Engine beim ersten Aufruf auf einem eigenen Loopback-Listener `127.0.0.1:0` (Spiel-API, `public/`, `dist/`; kein fester Port, keine Firewall-Abfrage); Testspielstände unter `os.UserCacheDir()/k3c/studioview/<Hash des Checkouts>/saves`, nicht im Repo. Gilt als Ansehen (kein schreibendes Control).
- **D-05** Teil-Auswahl per Klick: kleinste Box gewinnt, bei Gleichstand das spätere Teil in `Spec.teile`; Klick daneben hebt die Auswahl auf.
- **D-06** Entfallen ersatzlos: Chat, Welten-Editor, Gebäude-Planer, Kandidaten-Zuordnung, Testspielstand löschen, Spiel-Adresse wählen (`WorldGameServers`).
- **D-07** Funktional an die Studio-Tasks gebunden und daher in WB1.5: `.claude/launch.json`, `.oxlintrc.json`, `services.json` + `config_test.go`, `task-freigaben.json`. Reine Kommentare (`engine/devrun`, `engine/Taskfile.yml`, `assetkit/windowstate`, `webkit/lib/prefs.ts`, `hubGrowth.ts`) und Codemaps bleiben für US1/CU1.
- **D-08** Bereichsumschalter: `ChipSwitch` aus `@webkit/molecules` in der Kopfzeile; gemerkt werden Bereich und Seite je Bereich (`app.area`, `app.page`, `app.assetsPage`); Development-Seiten bleiben eingehängt, auch wenn Assets aktiv ist.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WB1.1 | `WB1.1-navigation-bereiche.md` | Umsetzung | autonom | offen |
| WB1.2 | `WB1.2-katalog-styleguide.md` | Umsetzung | autonom | offen |
| WB1.3 | `WB1.3-prompt-buttons.md` | Umsetzung | autonom | offen |
| WB1.4 | `WB1.4-studioview-studio-loeschen.md` | Umsetzung | autonom | offen |
| WB1.5 | `WB1.5-studio-aus-ci-taskfile-doku.md` | Umsetzung | autonom | offen |
| WB1.6 | `WB1.6-review.md` | Review | autonom | offen |

Lane `ui` (WB1.5: INF). Reihenfolge WB1.1 → WB1.2 → WB1.3; WB1.5 nach WB1.1 (parallel zu WB1.2/WB1.3 möglich, andere Domäne); WB1.4 nach WB1.2 und WB1.5; WB1.6 zuletzt.

- WB1.1 Sonnet: Navigation Development/Assets, Routen inkl. `ReviewPage`, `assets/api.ts`, StudioLink weg (AC-01)
- WB1.2 Sonnet: Katalog und Styleguide gegen die Stubs, Test „kein schreibendes Control“ (AC-02)
- WB1.3 Sonnet: Prompt-Buttons, Teil-Auswahl per Klick, `fill.ts` (AC-03)
- WB1.4 Sonnet: lesende Studio-Teile nach `internal/studioview` (Sound, Quellen, „Im Spiel ansehen“), `tools/k3c-studio` löschen (AC-02, AC-04)
- WB1.5 INF · Sonnet: Studio aus CI, Taskfile, Repo-Regeln und Doku (AC-04)
- WB1.6 Review · Opus

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.

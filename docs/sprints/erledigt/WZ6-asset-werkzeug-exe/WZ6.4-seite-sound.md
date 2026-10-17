# WZ6.4 · Seite Sound mit Hörprobe im Asset-Studio

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz6/4-seite-sound
- **Abhängig von:** WZ6.3
- **Tickets:** B-459
- **Kriterien:** AC-02, AC-06

## Ziel

Die Seite Sound des Asset-Studios kann alles, was die Workbench-Assets-Seite für Sound kann, spielt jede Zuordnung direkt am Slot ab und enthält die Sound-Mini-Szenen von `assetvorschau.html` (`sfx`, `musik` mit Wechsel per Crossfade, `ambient`) sowie die Hörprobe von `soundtest.html`.

## Kontext

Umgebung `live`: Fenster (`task k3c-studio`), Mock im Browser. **Browser-/Fensterprüfung nur mit Freigabe 🧑** für diesen Lauf; das Hören (u. a. `sfx:coin`, Musikwechsel, Abnahmepunkte aus B-421) nimmt 🧑 in WZ6.8 ab.

### Stand nach WZ6.3

Im Studio-Frontend liegen die gemeinsame Seitenstruktur (`AssetsPage`, `SlotDetail`, `CatalogPicker`, `ResearchPanel`, `CutEditor`, `MiniScene`, `assets.ts`, `useAssets`), die Seite Grafik, das Neuladen bei Änderung von `data/assets.json` (AC-07), die Aliase `@game` → `src/` und `@data` → `data/`, die Studio-Tests im Prüfziel und die umgezogene Logik aus `assetvorschauLogic.ts` (`SCENES`, `resolveSlot`, `slotsOf`, `licenseFlag`). Ordner und Dateinamen laut WZ6.3 › Ergebnis. Dateien aus `public/` liefert der Asset-Handler aus WZ6.2 (im Dev-Modus der Studio-Vite, WZ6.3).

### Sound-Teil der Workbench

`tools/k3c-dev/frontend/src/assets/`: Umschalter Grafik/Sound in `AssetsPage.tsx` (Art `sound`), `SoundProbe` in `SlotDetail.tsx` spielt ganze Dateien per `<audio>` über die Vite-Adresse und zeigt für Atlas-Töne (`sprite`) nur den Hinweis „in der Vorschau-Seite abspielen“. Sound-Slots haben einen Bus statt Zielmaß und keinen Ausschnitt (`CutEditor` nur Grafik).

### Sound-Szenen von `assetvorschau.html`

`src/tools/assetvorschau.ts`, Abschnitt Sound (`playSound`, `renderSounds`):
- Atlas-Ton (`sprite`): `audioCore().play(sprite, 'sfx', gain)`; Knopf „fern“ nur dafür mit `FAR_GAIN = attenuation(100, [{ center: 20, span: 20 }], false)` (Quelle in der anderen Bildhälfte des Split-Screens).
- Musik (`bus: music`): `playFile(datei, { fade: CROSSFADE_SECONDS })`, damit blendet der Wechsel `music:tag` → `music:nacht` über (B-459 › Beispiele).
- Umgebungsgeräusch (`bus: ambient`): `playFile` mit `bus: 'ambient'`, `loop: true`; sonst SFX.
- „Stopp“ stoppt `music`, `sfx`, `ambient`; Audio erst nach Nutzergeste (`core.onInput()` bei `pointerdown`/`keydown`); Statuszeile „Beschreibung · Datei#Ton“, fehlende Datei gemeldet.

### Hörprobe von `soundtest.html`

`src/tools/soundtest.ts` und `soundtestLogic.ts` (Test `soundtestLogic.test.ts`): `candidatesFromSlots` (nur ganze Dateien mit Bus `music`/`sfx`; Atlas-Töne und Umgebungsgeräusche fehlen dort), `groupCandidates` (erst Musik-Zustände `ZUSTAENDE`, dann Ereignisse `EREIGNISSE`), `fileFor` (Format nach `canPlayType`, `FORMAT_ORDER` aus `src/audio/atlas`), Abspielen hart oder mit Crossfade, Stopp, Musik-Lautstärke in 10-%-Schritten (`stepVolume`), Auswahl (`moveSelection`), Tastatur (`keyAction`). Die Pad-Steuerung (`padActions`) ist Xbox-Bedienung. `soundtest.html` bleibt für Hörproben am TV (SO4, SO2, HW1.9) unverändert. `audioProbe.ts` gehört zu `gamepad-test.html` (Format-Bericht der Xbox), nicht zur Hörprobe, und wird nicht übernommen.

### Spiel-Code nur lesend über Alias

| Import | im Studio |
|---|---|
| `src/audio/audioCore` (`audioCore`) | `@game/audio/audioCore` |
| `src/audio/crossfade` (`CROSSFADE_SECONDS`) | `@game/audio/crossfade` |
| `src/audio/events` (`attenuation`) | `@game/audio/events` |
| `src/tools/soundtestLogic` (samt `src/audio/atlas`) | `@game/tools/soundtestLogic`: importieren, nicht kopieren, weil `soundtest.html` sie weiter nutzt |
| `src/tools/credits` (NC) | `@game/tools/credits` wie in WZ6.3 |
| `data/assets.json` | nicht importieren, Slots aus `AssetsList` (Neuladen, AC-07) |
| `src/tools/texts`, `src/core/shell`, `src/core/clientLog` | nicht übernehmen, Beschriftungen deutsch im Studio |

### Fallstricke

- `audioCore` lädt `audio/atlas.json` und `audio/<datei>` relativ zur Seitenwurzel; fehlt eine Datei (z. B. im Mock), Meldung statt Absturz.
- WebView2 verlangt wie Edge eine Nutzergeste vor dem ersten Ton.
- Ein `audioCore` für das ganze Fenster, damit ein Musikwechsel überblendet statt doppelt zu spielen; beim Wechsel auf die Seite Grafik alle Busse stoppen.
- `audioCore` → `clientLog` (POST `/api/clientlog`) läuft ohne Spielserver ins Leere (wie in WZ6.3).

## Erlaubte Dateien

- `tools/k3c-studio/frontend/**` (Seite Sound, Erweiterungen der gemeinsamen Seitenstruktur aus WZ6.3, Tests, Codemaps; `package.json` ohne neue Abhängigkeit)
- `tools/assetkit/…` nur Ergänzungen, falls ein Binding fehlt (mit Go-Test), und dafür `tools/k3c-studio/*.go`

Nur lesen: `src/**`, `data/**`, `public/**`, `tools/k3c-dev/frontend/src/**`, `assetvorschau.html`, `soundtest.html`.

## Nicht-Ziele

`soundtest.html`, `src/tools/soundtest*.ts` und `src/audio/**` ändern oder entfernen; Pad-Steuerung der Hörprobe (bleibt am TV); `audioProbe.ts` und Format-Bericht; Grafik (WZ6.3); Aliase, Vitest- und Lint-Einbindung ändern (WZ6.3, nur nutzen); Assets-Seite der Workbench entfernen (WZ6.5); `assetvorschau.html` entfernen (WZ6.6); neue Sound-Funktionen; Format von `data/assets.json`.

## Schritte

1. Seite Sound in der Navigation auf der gemeinsamen Seitenstruktur (Art fest `sound`): Slots mit Filtern (Kategorie `sfx`/`musik`/`ambient`, Status, Suche), Präferenzen, Katalog, Recherche, Import; Neuladen aus WZ6.3 gilt mit.
2. Audio-Baustein des Studios auf `@game/audio/audioCore`: ein Kern je Fenster, Nutzergeste, Stopp aller Busse, Stopp beim Verlassen der Seite.
3. Die Abspiel-Zuordnung aus `playSound` (Bus, Gain, Schleife, Überblendung je Zuordnung) als reine Funktion mit Test.
4. Hörprobe am Slot statt des `SoundProbe`-Hinweises: jede Präferenz abspielbar, Atlas-Töne eingeschlossen, „fern“ für Atlas-Töne, Musik mit Crossfade, Umgebungsgeräusch als Schleife.
5. Reiter „Szenen“: Mini-Szenen `sfx`, `musik`, `ambient` wie in `assetvorschau.ts` (Slots mit Präferenz 1 oder `wahl=n`, NC, Lücke, Stopp, Statuszeile); `music:tag` → `music:nacht` blendet über.
6. Reiter „Hörprobe“: Liste aus `candidatesFromSlots`/`groupCandidates` über die Slots aus `AssetsList`, Format über `fileFor`, Abspielen hart oder mit Crossfade, Stopp, Lautstärke ±10 %, Tastatur über `keyAction`.
7. Codemaps nachziehen; Prüfziel, `task check`, `task k3c-studio:build`; mit Freigabe 🧑 Mock im Browser oder Fenster ohne Konsolenfehler.

## Fertig, wenn

- [x] AC-02: Seite Sound mit Slots, Filtern, Präferenzen, Katalog, Recherche, Import und Hörprobe je Zuordnung (auch Atlas-Töne); Tests im Prüfziel des Studios grün.
- [x] AC-06: Reiter „Szenen“ mit `sfx`, `musik` (Wechsel mit `CROSSFADE_SECONDS`) und `ambient` (Schleife) sowie Reiter „Hörprobe“ mit den Gruppen aus `soundtest.html` (Musik-Zustände, Ereignisse), Lautstärke und Crossfade; Test der Abspiel-Zuordnung (Bus, Gain, Schleife, Überblendung je Slot).
- [x] `git diff --name-only` der Session zeigt nichts unter `src/`, `data/`, `public/` und weder `soundtest.html` noch `assetvorschau.html`.
- [ ] Prüfziel des Asset-Studios, `task check` und `task k3c-studio:build` grün (Oxlint: Datei ≤ 400, Funktion ≤ 60 Zeilen): `check:studio` und `k3c-studio:build` grün, Oxlint ohne Befund im Studio; `task check` rot nur durch den vorbestehenden Lint-Fehler auf `develop` (B-465), Typecheck und Tests grün.

## Prüfen

```bash
task check
task check:studio      # Prüfziel des Asset-Studios, Name laut WZ6.2 › Ergebnis
task k3c-studio:build
```

Browser-/Fensterprüfung nur mit Freigabe 🧑 für diesen Lauf: Mock des Studio-Frontends im Browser-Pane oder `task k3c-studio`, Seite Sound ohne Konsolenfehler. Hören am PC prüft 🧑 in WZ6.8, Hörproben am TV laufen weiter über `soundtest.html`.

## Ergebnis

- **AC-02 umgesetzt, geprüft:** Seite Sound auf der gemeinsamen Seitenstruktur (`AssetsPage art="sound"`): Slots mit Filtern, Präferenzen, Katalog, Recherche, Import; `SoundProbe` am Slot spielt jede Präferenz wie im Spiel (Atlas-Töne mit „fern“, Musik mit Crossfade, Umgebung als Schleife). DOM-Test `SoundPage.test.tsx` (Atlas-Ton `sfx:coin` mit „fern“, keine Ausschnitt-Felder).
- **AC-06 umgesetzt, geprüft:** `playback.ts` (Abspiel-Zuordnung, umgezogen aus `playSound`) mit Test; `audio.ts` mit einem `audioCore` des Spiels je Fenster, Stopp aller Busse (beendet auch noch wartende Atlas-Wiederholungen), Stopp beim Verlassen der Seite. Reiter Szenen (`sfx`, `musik`, `ambient`, Vergleich Präferenz n, NC, Lücke, Stopp, Statuszeile) und Hörprobe (`candidatesFromSlots`/`groupCandidates`/`fileFor`/`keyAction`/`stepVolume` aus `@game/tools/soundtestLogic`, hart oder Crossfade, Lautstärke ±10 % aus dem Mixer, Tastatur).
- **Fenster (Freigabe 🧑, 2026-10-17):** `bin/k3c-studio.exe` mit echten Daten: 19 Sound-Slots; `sfx:coin` spielt schon beim ersten Klick (Status „Münze · audio/atlas.ogg#coin“); Szene Musik `music:tag` → `music:nacht` startet beide (Status ok, die Überblendung hört 🧑 in WZ6.8); Hörprobe mit Gruppen Tag, Nacht, Münze, „spielt: probe-a“. Screenshots im Notizordner (`side-session-notes/…/wz6.4-screenshots/`). Konsole im Fenster nicht einsehbar (kein Devtools-Zugriff), daher nicht geprüft. Der Stand nach dem Review (siehe unten) lief nicht mehr im Fenster, nur in Tests und Build.
- **Prüfung:** `task check:studio` grün (49 Tests, Go-Tests, Lint), `task k3c-studio:build` grün, `check_run task:test` und `task:typecheck` grün; `task:lint` rot nur durch B-465.
- **Review (getrennter Reviewer):** keine schweren Befunde. Behoben: Stopp der Hörprobe stoppt alle Busse (auch nach Reiterwechsel), Lautstärke aus dem Mixer und nur bei laufendem Kern, Abbruch wartender Atlas-Töne bei Stopp, eigene Meldung für unbekannte Atlas-Töne, Enter/Leertaste auf Knöpfen, `playing` im Ref, Testisolation, Kommentar zu `FAR_GAIN`. Gelassen: „lädt nicht“ erscheint erst beim Klick (soundtest.html lädt alle vor).
- **Abweichungen:** `FAR_GAIN` mit x = 40 (Vorschau-Wert, Faktor 0,25): der Wert aus `assetvorschau.ts` (x = 100) ergibt mit der Formel aus SO1 den Faktor 0, „fern“ war dort stumm. Atlas-Töne warten nach dem ersten Entsperren bis zu 3 s auf den Atlas (sonst schlug der erste Klick fehl, im Fenster beobachtet). Der Sound-DOM-Test ersetzt `@game/tools/credits` durch eine leere Liste (das Studio-Vitest liefert `public/**/CREDITS.md?raw` außerhalb seiner Wurzel nicht aus; die Einbindung blieb laut Nicht-Zielen unverändert).
- **Geprüft von:** Claude (autonom, Fenster mit Freigabe 🧑). **Neue Tickets:** keine.

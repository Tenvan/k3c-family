# SO4.6 · Audio-Kern spielt ganze Dateien mit Crossfade, Ambient-Regler (aus SO5.1)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** so4/6-audiokern-dateien
- **Abhängig von:** –
- **Tickets:** B-250, B-218
- **Kriterien:** AC-08, AC-09

## Ziel

Der Audio-Kern spielt ganze Dateien in Schleife mit Crossfade auf einem Bus, die Hörprobe nutzt ihn ohne eigenen `AudioContext`, und die Optionen-Szene regelt die Ambient-Lautstärke.

## Kontext

Aus SO5.1 (PJ3, 2026-10-08); SO5 war als „vor SO4“ geplant, deshalb hängen SO4.2 und das Review von dieser Session ab. Spec: B-250, B-218.
- `src/audio/audioCore.ts` spielt nur Sprites des Atlas (`play(name, bus)`), `ctx` und `mixer` sind privat. Die Hörprobe (`src/tools/soundtest.ts`, SO3.2) legt einen eigenen `AudioContext` mit eigenem `Mixer` an; Formatwahl (`pickFile`) und Mixer sind wiederverwendet.
- Kurve `crossfadeCurves` aus `src/tools/soundtestLogic.ts` in den Kern verschieben; Datei laden und auf Bus `music` in Schleife spielen, Crossfade zweier Stücke ohne Sprung.
- Mixer `src/audio/mixer.ts` hat drei Busse, `Settings.ambientVolume` (Standard 100 %) in `src/core/settings.ts`; die Optionen-Szene (`src/scenes/optionsLogic.ts`) zeigt nur Musik und Effekte. Neuer Eintrag „Ambient“ mit ◀ ▶ in 10er-Schritten, Texte in `texts.de.ts` und `texts.en.ts`.
- Grenzfall Domäne: `src/tools/soundtest.ts` ist PLAT; die Umstellung der Hörprobe auf den Kern ist die zwingende Folge von B-250 und bleibt auf diese Datei beschränkt (im Ergebnis nennen).

## Erlaubte Dateien

- `src/audio/` (Code und Tests), `src/scenes/optionsLogic.ts` und Tests, Textdateien `texts.de.ts`/`texts.en.ts`
- `src/tools/soundtest.ts`, `src/tools/soundtestLogic.ts` (nur Umstellung auf den Kern)
- `docs/sprints/geplant/SO4-musik/` bzw. `aktiv/` (Status, Ergebnis), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Zustands-Automat der Musik (SO4.2), Ducking und Musik-Dateien (SO4.3), Ambient-Klänge selbst.

## Schritte

1. Datei-Wiedergabe mit Schleife und Crossfade im Kern, Kurve dorthin verschieben, Tests.
2. Hörprobe auf den Kern umstellen (kein zweiter `AudioContext`).
3. Optionen-Eintrag „Ambient“ mit Texten und Test.
4. `task check` grün, Ergebnis je Kriterium, committen.

## Fertig, wenn

- [x] AC-08: Test spielt eine Datei in Schleife und blendet zwei Stücke über; `soundtest.ts` erzeugt keinen eigenen `AudioContext`.
- [x] AC-09: Test der Optionen-Logik: „Ambient“ ändert `ambientVolume` in 10er-Schritten.
- [x] `task check` grün (siehe Ergebnis: Lint-Teil von `task check` scheitert an `tools/cockpit`, nicht an dieser Session).

## Prüfen

```bash
task check
```

Hören im Browser nur, wenn 🧑 es für den Lauf freigibt.

## Ergebnis

- **AC-08 geprüft:** `src/audio/audioFiles.test.ts` (7 Tests): `AudioCore.playFile(url, {bus, loop, fade, onEnded})` spielt eine ganze Datei (Standard auf `music` in Schleife, sonst einmal), je Bus ein Stück; Crossfade mit `crossfadeCurves().in` für das neue, vom aktuellen Pegel ausgehender Kurve (`fadeOutFrom`) für das alte, Umlenken mitten im Übergang ohne Sprung, gleiches Stück läuft weiter, fehlende Datei → `false` ohne Absturz und das laufende Stück bleibt, jede Datei wird einmal geladen. `src/tools/soundtest.ts` nutzt `audioCore()` und hat weder `AudioContext` noch `Mixer` mehr. Die Kurve liegt jetzt in `src/audio/crossfade.ts` (Test dort, aus `soundtestLogic` entfernt).
- **AC-09 geprüft:** `src/scenes/optionsLogic.test.ts`: Eintrag „Ambient“ (`Umgebung`/`Ambience`) in 10er-Schritten, begrenzt 0–100, berührt Musik und Effekte nicht; Zeilenabstand der Optionen-Szene 78 → 72 px, damit 11 Zeilen über den Hinweis passen.
- **Zusatz AC-05:** `getVolume`/`setVolume` je Bus im Kern, Test: Musik ändert Effekte nicht.
- **Prüfung:** `npx vitest run` für `src/` und `tests/` grün (1367 + 577 Tests), `tsc --noEmit` ohne Fehler. `task check` insgesamt rot ohne Bezug zu dieser Session: Lint meldet 22 Fehler in `tools/cockpit/` (aus #278/#280) und die Tests in `tools/cockpit/hooks` und `tools/k3c-dev/frontend` finden `claude-code/testing`, `react` und `@radix-ui/themes` im Worktree nicht (Pakete nicht installiert). Nicht von mir verändert.
- **Grenzfall Domäne:** `src/tools/soundtest.ts` und `soundtestLogic.ts` (PLAT) nur für die Umstellung auf den Kern angefasst, wie in der Session vorgesehen.
- **Nicht gehört:** Hören im Browser nicht freigegeben, kein Browser-Lauf. Offen: Die Optionen-Szene speichert die Lautstärke, der Mixer liest sie nur beim Anlegen; ein Live-Übernehmen (OptionsScene → `AudioCore.setVolume`) gibt es noch nicht, es gehört in SO4.2 (Anbindung an die Szenen).

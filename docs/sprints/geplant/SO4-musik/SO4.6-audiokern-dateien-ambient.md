# SO4.6 · Audio-Kern spielt ganze Dateien mit Crossfade, Ambient-Regler (aus SO5.1)

- **Status:** offen
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

- [ ] AC-08: Test spielt eine Datei in Schleife und blendet zwei Stücke über; `soundtest.ts` erzeugt keinen eigenen `AudioContext`.
- [ ] AC-09: Test der Optionen-Logik: „Ambient“ ändert `ambientVolume` in 10er-Schritten.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

Hören im Browser nur, wenn 🧑 es für den Lauf freigibt.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

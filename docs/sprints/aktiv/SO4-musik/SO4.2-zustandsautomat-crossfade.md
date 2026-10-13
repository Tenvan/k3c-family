# SO4.2 · Zustands-Automat, Crossfade, Bus „Musik“

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** so4/2-zustandsautomat-crossfade
- **Abhängig von:** SO4.1, SO1.4, SO4.6
- **Tickets:** B-168
- **Kriterien:** AC-01, AC-02, AC-05

## Ziel

Ein Zustands-Automat leitet aus Snapshot und Ereignissen den Musikzustand ab, bildet jeden der acht Zustände auf genau ein Stück ab und blendet per Crossfade ohne Pause und Knacken über; die Musik läuft über den Bus „Musik“, dessen Lautstärke getrennt von den Effekten einstellbar ist.

## Kontext

- **Audio-Kern (SO1, Voraussetzung):** `src/audio/` mit Mixer (Busse Musik/Effekte/Ambient mit eigener Lautstärke, Speicher im `localStorage`), Entsperren per Geste, Format mit Fallback. Aufbau aus dem Stand auf `develop` lesen; den Kern hier **nicht** ändern (Änderungswunsch → Ticket CLI). Fehlt er: `Status: blockiert`. Ein Stück spielt als `AudioBufferSourceNode` oder `HTMLAudioElement` über den Bus; Wahl nach dem, was der Kern anbietet, Begründung im Ergebnis.
- **Zustände (Beschluss Q16):** Tag, Abend, Nacht, Kampf, Tiefe/Höhle, Boss, Lobby, Niederlage/Sieg; Stück je Zustand aus SO4.1 (`docs/assets/sounds.md`, Abschnitt Musik). Quellen im Snapshot: `world.cycle.phase` (`day` | `dusk` | `night`), `world.wave` und `world.enemies` (Kampf), `world.biome.depth` (Tiefe), Ereignis `castleFallen` (Niederlage); Lobby = Szene `src/scenes/LobbyScene.ts`. **Boss und Sieg** liefert die Simulation erst mit K2/K4: der Automat kennt die Zustände und das Mapping schon, die Auslöser sind bis dahin ohne Quelle (im Ergebnis nennen; Test mit künstlichem Zustand). Ein Client rechnet keinen Spielzustand: der Automat ist eine **reine Funktion** (Snapshot-Auszug, Ereignisse, bisheriger Zustand → Zustand) mit festgelegter Priorität (z. B. Niederlage/Sieg vor Boss vor Kampf vor Tiefe vor Tageszeit); die Reihenfolge ist hier festzulegen, im Test zu belegen und im Ergebnis zu nennen.
- **Mapping:** Zustand → genau ein Stück (AC-01); Tabelle als Daten (z. B. `public/audio/…`-Beschreibung oder Konstante in `src/audio/`), nicht in `src/scenes`. Test: jeder der acht Zustände hat ein Stück oder ist als Lücke mit Ersatzverhalten markiert; kein Zustand hat zwei Stücke.
- **Crossfade:** Dauer als Wert in der Konfiguration des Audio-Kerns (B-168 erlaubt auch `data/`, das gehört aber nicht zur Domäne CLI: dann Ticket SIM statt eigener Änderung), Beispiel in der Spec: 2 s von „Abend“ nach „Nacht“. Lautstärke über Rampen (`linearRampToValueAtTime` o. ä.), nie ein Sprung; **Wechsel schneller als der Crossfade** → laufenden Übergang ohne Knacken auf das neue Ziel umlenken (aktuellen Pegel als Start nehmen). Test der reinen Rampen-Funktion (Pegelverlauf stetig, Summe der Pegel ≤ 1). Vorbild: Crossfade-Probe in `src/tools/soundtest.ts` (SO3.2), dessen reine Funktion mitnutzen, falls sie im Kern liegt, sonst eigene und Ticket „zusammenführen“.
- **Ausnahme:** Stück fehlt oder lädt nicht → vorheriger Zustand läuft weiter oder Stille, kein Absturz, höchstens ein `clientLog`-Eintrag je Stück (`src/core/clientLog.ts`).
- **Eine Musik je Gerät** (nicht je Viertel im Split-Screen); Zustand aus der Stufe des Geräts (`GameScene`: eine Welt je Gerät, `world_`).
- **Bus „Musik“ (AC-05):** Lautstärke getrennt von Effekten; Regler in der Optionen-Szene kommt mit S5 (`src/core/settings.ts`, S5.1), hier genügt der Nachweis, dass der Bus-Gain Musik unabhängig von Effekten verändert (Test des Mixers über seinen Einstieg).
- **Regeln:** `src/scenes` rechnet nichts (`src/scenes/noSim.test.ts`); B-Taste nicht belegen; Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen.
- **Hören am TV** (AC-02 nach Gehör) macht 🧑 in SO4.5; keine Abhängigkeit.

## Erlaubte Dateien

- `src/audio/` (Automat, Rampen, Mapping, Tests)
- `src/scenes/GameScene.ts`, `src/scenes/LobbyScene.ts` (nur: Zustand bzw. Szenenwechsel an den Automaten melden)
- `public/audio/` (nur Beschreibung des Mappings, falls als Datei)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Ducking, Musik-Dateien einchecken, Credits (SO4.3), Hörprobenseite (SO3), Optionen-Szene (S5), dynamische Layer, Boss- und Sieg-Auslöser in der Simulation (K2).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Auswahl aus SO4.1 und SO1-Ergebnisse lesen.
2. Reiner Zustands-Automat mit Test (Tabelle: Tag → Abend → Nacht, Kampf, Tiefe, Niederlage, Boss/Sieg künstlich; Prioritäten).
3. Mapping Zustand → Stück mit Test (genau ein Stück je Zustand).
4. Crossfade über den Bus „Musik“ mit Rampen und Umlenken; Test der reinen Funktion.
5. Anbindung: `GameScene` meldet Zustand und Ereignisse, `LobbyScene` meldet Lobby; im Browser-Pane Tag → Abend → Nacht mit `?fast=1` durchlaufen und Konsole prüfen (`read_console_messages`: keine Fehler).
6. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-01: Test: Der Automat bildet jeden der acht Zustände auf genau ein Stück ab (Lücke mit Ersatzverhalten ausgewiesen).
- [x] AC-02: Test der Rampen (stetiger Pegelverlauf, Umlenken ohne Sprung) und Durchlauf Tag → Abend → Nacht im Browser-Pane ohne Fehler; Hören am TV ist SO4.5.
- [x] AC-05: Test: Bus „Musik“ ändert sich unabhängig vom Bus „Effekte“.
- [x] `src/scenes/noSim.test.ts` grün; fehlendes Stück lässt das Spiel weiterlaufen (Test).

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Neu: `src/audio/music.ts` (+ `music.test.ts`), angebunden in `GameScene` (reset, Ereignisse, Auszug je Frame) und `LobbyScene` (`lobby()` je Frame). Audio-Kern unverändert.

**Abweichung von der Reihenfolge:** SO4.1 (Workshop, 🧑) ist noch `offen`, obwohl diese Session von ihr abhängt; auf ausdrücklichen Wunsch von 🧑 trotzdem umgesetzt. Folge: Es gibt noch keine gewählten Stücke, `MUSIC_TRACKS` führt alle acht Zustände als **Lücke** (`null`). Ersatzverhalten bei Lücke: Stille (das laufende Stück blendet in 2 s aus). Sobald SO4.1/SO4.3 Stücke liefern, wird nur diese Tabelle gefüllt.

- **AC-01:** `nextMusicState` ist eine reine Funktion (bisheriger Zustand, Auszug, Ereignisse → Zustand). Priorität: Lobby > Niederlage/Sieg > Boss > Kampf (`enemies > 0`) > Tiefe (`biome.depth > 0`) > Tageszeit; „Ende“ bleibt nach `castleFallen`, bis die Lobby läuft. Test belegt die Reihenfolge und das Mapping: acht Zustände, je genau ein Eintrag (Stück oder Lücke), kein Stück doppelt (`music.test.ts`).
- **Boss und Sieg:** Auslöser (`boss`, `victory` im Auszug) haben bis K2/K4 keine Quelle; Test mit künstlichem Zustand.
- **AC-02:** Rampen und Umlenken ohne Sprung testen schon `crossfade.test.ts` und `audioFiles.test.ts` (SO4.6): das Umlenken nimmt den aktuellen Pegel als Start. Hinweis: Die Kurven haben **gleiche Leistung** (Summe der Quadrate = 1, belegt im Test), nicht „Summe der Pegel ≤ 1“ (Spitze √2 in der Mitte); bei Bedarf eigenes Ticket. Dauer `CROSSFADE_SECONDS` = 2 s im Kern. Browser-Pane (`?autostart=1&mock=1`, Dev-Aktion `phase` per `/api/dev`): Lobby → Tag → Abend → Nacht → Kampf (Nachtwelle) wie erwartet, **keine Konsolenfehler**. Hören am TV: SO4.5. Hinweis: Da alle Stücke Lücken sind, war im Pane nur der Zustandswechsel prüfbar, kein hörbarer Crossfade.
- **AC-05:** Bus „Musik“ unabhängig von „Effekte“: Test `audioFiles.test.ts` (SO4.6, „Lautstärke je Bus getrennt“) und `mixer.test.ts`.
- **Fehlendes Stück:** `music.test.ts`: Datei lädt nicht → kein Absturz, kein erneuter Versuch je Frame (der Kern loggt einmal je Datei); gesperrter Kontext → kein Request, Start nach der Entsperrung; Wechsel während des Ladens → altes Stück startet nicht mehr. `noSim.test.ts` grün.
- **Wahl Wiedergabe:** `AudioBufferSourceNode` über `AudioCore.playFile` (Kern aus SO4.6), `MusicDirector` ruft nur `preload`, `playFile`, `stop`.
- **Eine Musik je Gerät:** `musicDirector()` ist ein Singleton, gespeist aus der Welt der eigenen Stufe (`world_`).

**Prüfen:** `task:typecheck` grün, `vitest` für `src/audio`, `src/scenes`, `tests/planning` grün (1022 Tests), `task:build` grün. `task check` ist in diesem Worktree **nicht** grün, aber nicht durch diese Session: `oxlint` meldet 22 Fehler nur in `tools/cockpit/hooks/*`, und 11 Testdateien in `tools/cockpit` und `tools/k3c-dev/frontend` finden Pakete nicht (`react`, `@radix-ui/themes`, `claude-code/testing`: Abhängigkeiten dort nicht installiert). Hinweis zum Lauf: Hook-Ordner `.omc/` im Sprint-Ordner (durch `cd`) hatte `tests/planning` kurz gebrochen, gelöscht.

**Offen / Tickets:** keine neuen. Folgearbeit: Stücke eintragen (SO4.1, SO4.3), Hysterese gegen Kampf-Flackern (`enemies` 0 ↔ 1) nur falls SO4.5 es hörbar findet.

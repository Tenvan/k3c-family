# Sound-Katalog

Katalog Ereignis → Sound → Quelle → Lizenz (B-167, SO2). Die Zuordnung steht im Slot-Register `data/assets.json` (Reihenfolge = Präferenz, der erste Sound ist aktiv), Urheber und Lizenz in `public/audio/README.md` › Importe (k3c-workbench). Stand 2026-10-18, Auswahl durch 🧑 im Asset-Studio. Lizenz-Bestätigung in k3c-workbench durch 🧑 steht aus. Einbau ins Spiel: SO2.3.

## Effekte

| Ereignis | Slot | Sound (aktiv) | Quelle | Urheber | Lizenz | Status |
|---|---|---|---|---|---|---|
| Treffer | `sfx:hit` | `audio/kandidaten/kenney-impact-sounds-impactpunch-medium.ogg` (+2) | https://kenney.nl/assets/impact-sounds | Kenney | CC0 | zugeordnet |
| Gegner-Tod | `sfx:kill` | `audio/kandidaten/kenney-digital-audio-spacetrash2.ogg` (+1) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Münze aufheben | `sfx:coin` | `audio/kandidaten/kenney-rpg-audio-handlecoins.ogg` (+2) | https://kenney.nl/assets/rpg-audio | Kenney | CC0 | zugeordnet |
| Münze geben | `sfx:coinGive` | `audio/kandidaten/kenney-rpg-audio-handlecoins2.ogg` | https://kenney.nl/assets/rpg-audio | Kenney | CC0 | gewählt (Default Claude 2026-10-22, B-558), eingebaut SO6.4 (`coinGive`) |
| Pfeil | `sfx:arrow` | `audio/kandidaten/kenney-digital-audio-phaserup2.ogg` (+2) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Schlag | `sfx:strike` | `audio/kandidaten/kenney-rpg-audio-chop.ogg` (+2) | https://kenney.nl/assets/rpg-audio | Kenney | CC0 | zugeordnet |
| Bau-Fortschritt | `sfx:buildProgress` | `audio/kandidaten/kenney-impact-sounds-impactmining-000.ogg` | https://kenney.nl/assets/impact-sounds | Kenney | CC0 | gewählt (Default Claude 2026-10-22, B-558), eingebaut SO6.4 (`buildProgress`) |
| Bau fertig | `sfx:build` | `audio/kandidaten/kenney-rpg-audio-metallatch.ogg` (+1) | https://kenney.nl/assets/rpg-audio | Kenney | CC0 | zugeordnet |
| Tod | `sfx:death` | `audio/kandidaten/kenney-digital-audio-lowdown.ogg` (+1) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Skill | `sfx:skill` | `audio/kandidaten/kenney-digital-audio-powerup11.ogg` (+2) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Nacht naht | `sfx:warn` | `audio/kandidaten/kenney-impact-sounds-impactbell-heavy-00.ogg` (+2) | https://kenney.nl/assets/impact-sounds | Kenney | CC0 | zugeordnet |
| Portal öffnet | `sfx:portal` | `audio/kandidaten/kenney-digital-audio-phasejump3.ogg` (+2) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Respawn (`revive`) | `sfx:revive` | `audio/kandidaten/kenney-digital-audio-powerup3.ogg` (+2) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Wiederbeleben durch Mitspieler (`revived`) | `sfx:revived` | `audio/kandidaten/kenney-digital-audio-highup.ogg` | https://kenney.nl/assets/digital-audio | Kenney | CC0 | gewählt (Default Claude 2026-10-22, B-558), eingebaut SO6.4 (`revived`) |
| Hub-Ausbau | `sfx:hub` | `audio/kandidaten/kenney-digital-audio-threetone2.ogg` | https://kenney.nl/assets/digital-audio | Kenney | CC0 | gewählt (Default Claude 2026-10-22, B-558), eingebaut SO6.4 (Anstieg `hubLevel`, global) |
| Skill je Klasse | – | – | – | – | – | **Lücke** (B-576): gemeinsamer Ton `sfx:skill` (Rückfall); der Snapshot nennt weder Klasse noch Skill-Auslösung eines Spielers (nur Preset-Punkte, `types.ts`), ein Ereignis je gewirktem Skill fehlt |
| Boss-Auftritt | `sfx:boss` | `audio/kandidaten/kenney-digital-audio-lowthreetone.ogg` | https://kenney.nl/assets/digital-audio | Kenney | CC0 | gewählt (Default Claude 2026-10-22, B-558), eingebaut SO6.4 (`bossSpawned`, global) |
| UI-Klicks | `sfx:uiSelect` | `audio/kandidaten/selbst-erzeugt-ui-select.ogg` | selbst erzeugt (Rezept `public/audio/README.md`) | Claude (selbst erzeugt) | CC0 | eingebaut (B-575): Auswahl wechseln in Lobby und Optionen, kein Server-Ereignis (Q08) |
| UI-Klick Bestätigen | `sfx:uiConfirm` | `audio/kandidaten/selbst-erzeugt-ui-confirm.ogg` | selbst erzeugt (Rezept `public/audio/README.md`) | Claude (selbst erzeugt) | CC0 | eingebaut (B-575): Bestätigen (Enter, A, Tippen) |
| Reiten (Hufschlag, Schleife) | `sfx:ride` | `audio/kandidaten/horse-gallop-loop-01.ogg` (+2) | https://opengameart.org/content/horse-gallop-loop | AntumDeluge | CC-BY 3.0 | eingebaut (B-574): Schleife je Spieler, solange er lebt und sich bewegt (`vx`); Absteigen gibt es im Snapshot nicht (Monarch ist immer beritten) |

Die Lücken Münze geben, Bau-Fortschritt, `revived`, Hub-Ausbau und Boss-Auftritt sind mit B-492 geschlossen, Reiten und UI-Klicks mit SO7 (B-574, B-575); offen bleibt nur Skill je Klasse (B-576). Selbst erzeugte Töne (UI-Klicks, Niederlage-Stück) stehen mit Rezept in `public/audio/README.md`.

## Musik (Auswahl SO4.1 vorentschieden, Einbau SO4.3)

| Zustand | Slot | Sound (aktiv) | Quelle | Urheber | Lizenz | Status |
|---|---|---|---|---|---|---|
| Tag | `music:tag` | `audio/kandidaten/happy-adventure-loop.ogg` (+3) | https://opengameart.org/content/happy-adventure-loop | TinyWorlds | CC0 | gewählt (Default Claude 2026-10-22, B-554), spielt im Spiel |
| Abend | `music:abend` | `audio/kandidaten/chiptune-medieval-the-old-tower-inn.ogg` | https://opengameart.org/content/chiptune-medieval-the-old-tower-inn | RandomMind | CC0 | gewählt (Default Claude 2026-10-22, B-554), spielt im Spiel |
| Nacht | `music:nacht` | `audio/kandidaten/haunting-chiptune-loop-void-estate.ogg` (+1) | https://opengameart.org/content/haunting-chiptune-loop-void-estate | Zane Little Music | CC0 | gewählt (Default Claude 2026-10-22, B-554), spielt im Spiel |
| Kampf | `music:kampf` | `audio/kandidaten/pixel-sprinter.ogg` (+2) | https://opengameart.org/content/pixel-sprinter | Zane Little Music | CC0 | gewählt (Default Claude 2026-10-22, B-554), spielt im Spiel |
| Tiefe/Höhle | `music:tiefe` | `audio/kandidaten/8-bit-cave-loop.ogg` | https://opengameart.org/content/8-bit-cave-loop | Wolfgang_ | CC0 | gewählt (Default Claude 2026-10-22, B-554), spielt im Spiel |
| Boss | `music:boss` | `audio/kandidaten/nes-shooter-music-boss.ogg` (+1) | https://opengameart.org/content/nes-shooter-music-5-tracks-3-jingles | SketchyLogic | CC0 | gewählt (Default Claude 2026-10-22, B-554), spielt im Spiel |
| Lobby | `music:lobby` | `audio/kandidaten/nes-shooter-music-map.ogg` (+1) | https://opengameart.org/content/nes-shooter-music-5-tracks-3-jingles | SketchyLogic | CC0 | gewählt (Default Claude 2026-10-22, B-554), spielt im Spiel |
| Niederlage | `music:niederlage` | `audio/kandidaten/selbst-erzeugt-niederlage.ogg` | selbst erzeugt (Rezept `public/audio/README.md`) | Claude (selbst erzeugt) | CC0 | eingebaut (B-573): spielt bei `castleFallen`, sanftes a-Moll, bis die Lobby läuft; Ersatz durch ein Stück aus der Kandidatensuche möglich (Auswahl 🧑) |
| Sieg | `music:ende` | `audio/kandidaten/8-bit-victory-loop.ogg` | https://opengameart.org/content/8-bit-victory-loop | Wolfgang_ | CC0 | zugeordnet für Sieg (Zustand `ending`, noch kein Auslöser); läuft nicht bei Niederlage (B-573) |

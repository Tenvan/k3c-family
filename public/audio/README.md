# Sound-Atlas (SO6.2, B-494)

`atlas.ogg`, `atlas.mp3` und `atlas.json` erzeugt `task gen:soundatlas` (`tools/soundatlas`): die erste Präferenz jedes
Effekt-Slots in `data/assets.json` (Bus `sfx`, ohne die Schleife `sfx:ride`), mono 44,1 kHz, 0,15 s Stille dazwischen
(fängt die MP3-Encoder-Verzögerung ab), Ogg Vorbis q3 und MP3 64 kbit/s. `atlas.json` nennt Start und Dauer je Slot. Der Atlas
setzt nur fremde Dateien zusammen, die unten mit Credit stehen; er selbst braucht keinen eigenen Eintrag. **Nach jeder Änderung
der Effekt-Slots `task gen:soundatlas` laufen lassen**; ein Slot ohne Atlas-Eintrag lädt als Einzeldatei (Rückfall).
Bis SO6.2 war der Atlas ein selbst erzeugter Platzhalter aus drei Sinustönen (SO1.2).

## Probe-Stücke und Kandidaten (SO3.2)

`probe-a` (C-Dur-Akkord, 2 Hz Tremolo) und `probe-b` (a-Moll-Akkord, 3 Hz Tremolo): je 8 s, mono, `.ogg` und `.mp3`, für die Crossfade-Probe
der Hörprobenseite. **Selbst erzeugt, keine fremden Inhalte** (kein Credit nötig). Die Hörprobe zeigt die Sound-Slots des
Slot-Registers `data/assets.json` (B-298, M10; früher `kandidaten.json`); Zuordnungen setzt k3c-workbench. Fremde Dateien nur mit Quelle und Lizenz (B-165).

```bash
ffmpeg -f lavfi -i "aevalsrc='(sin(2*PI*262*t)+sin(2*PI*330*t)+sin(2*PI*392*t))/3*0.4*(0.7+0.3*sin(2*PI*2*t))':d=8:s=44100" -ac 1 -map_metadata -1 -c:a libvorbis -q:a 2 probe-a.ogg
# mp3: -c:a libmp3lame -b:a 48k; probe-b: 220/277/330 Hz, Tremolo 3 Hz
```

## Selbst erzeugte Töne (SO7)

Ohne Netz und ohne fremde Dateien erzeugt (Claude, 2026-10-23), Lizenz CC0: Synthese in Python (nur Standardbibliothek, Sinus mit Obertönen und exponentiellem Ausklingen, 22,05 kHz mono), danach `ffmpeg -map_metadata -1 -c:a libvorbis -q:a 3`. Sie liegen unter `kandidaten/` und stehen wie die fremden Dateien in der Credit-Tabelle unten.

- `selbst-erzeugt-ui-select.ogg` (B-575): 90 ms, 880 Hz, Obertöne 1 und 0,2, Abklingen 45/s.
- `selbst-erzeugt-ui-confirm.ogg` (B-575): 220 ms, zwei Töne 660 Hz (ab 0 s) und 990 Hz (ab 70 ms), Abklingen 18/s und 16/s.
- `selbst-erzeugt-niederlage.ogg` (B-573): 16 s nahtlose Schleife, 60 bpm, Akkordfolge a-Moll, F-Dur, d-Moll, E-Dur (Bass A2, F2, D2, E2 plus Achtel-Arpeggio), darüber eine absteigende Melodie in Glockenklang; Ausklänge laufen über das Schleifenende hinweg.

## Importe (k3c-workbench)

Mit k3c-workbench importierte Kandidaten (B-298): je Datei eine Zeile. „Lizenz prüfen“ gilt, bis 🧑 die Lizenz in k3c-workbench bestätigt.

Kenney-Dateien (Packs Digital Audio, RPG Audio, Impact Sounds, Interface Sounds, alle CC0) liegen unverändert vor. Die OpenGameArt-Stücke (Musik, alle CC0) sind zum Hören aus den Originalen neu auf Ogg Vorbis 96 kbit/s kodiert (`ffmpeg -c:a libvorbis -b:a 96k -ac 2`, Metadaten entfernt), damit die Kandidaten klein bleiben (zusammen etwa 9 MB statt 28 MB); die Wahl entscheidet später, ob das Original nachgeladen wird. Urheber bei OpenGameArt sind die dortigen Benutzernamen.

| Ordner | Pack | Urheber | Lizenz | Quelle | Status |
|---|---|---|---|---|---|
| kandidaten/kenney-digital-audio-spacetrash2.ogg | Kenney Digital Audio: spaceTrash2 | Kenney | CC0 | https://kenney.nl/assets/digital-audio | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-zapthreetonedown.ogg | Kenney Digital Audio: zapThreeToneDown | Kenney | CC0 | https://kenney.nl/assets/digital-audio#zapThreeToneDown | Lizenz erkannt (Kenney) |
| kandidaten/kenney-rpg-audio-drawknife1.ogg | Kenney RPG Audio: drawKnife1 | Kenney | CC0 | https://kenney.nl/assets/rpg-audio#drawKnife1 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-phaserup2.ogg | Kenney Digital Audio: phaserUp2 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#phaserUp2 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-laser5.ogg | Kenney Digital Audio: laser5 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#laser5 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-rpg-audio-knifeslice.ogg | Kenney RPG Audio: knifeSlice | Kenney | CC0 | https://kenney.nl/assets/rpg-audio#knifeSlice | Lizenz erkannt (Kenney) |
| kandidaten/kenney-rpg-audio-knifeslice2.ogg | Kenney RPG Audio: knifeSlice2 | Kenney | CC0 | https://kenney.nl/assets/rpg-audio#knifeSlice2 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-rpg-audio-chop.ogg | Kenney RPG Audio: chop | Kenney | CC0 | https://kenney.nl/assets/rpg-audio#chop | Lizenz erkannt (Kenney) |
| kandidaten/kenney-rpg-audio-metallatch.ogg | Kenney RPG Audio: metalLatch | Kenney | CC0 | https://kenney.nl/assets/rpg-audio#metalLatch | Lizenz erkannt (Kenney) |
| kandidaten/kenney-impact-sounds-impactmining-000.ogg | Kenney Impact Sounds: impactMining_000 | Kenney | CC0 | https://kenney.nl/assets/impact-sounds#impactMining_000 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-lowdown.ogg | Kenney Digital Audio: lowDown | Kenney | CC0 | https://kenney.nl/assets/digital-audio#lowDown | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-lowthreetone.ogg | Kenney Digital Audio: lowThreeTone | Kenney | CC0 | https://kenney.nl/assets/digital-audio#lowThreeTone | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-powerup3.ogg | Kenney Digital Audio: powerUp3 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#powerUp3 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-highup.ogg | Kenney Digital Audio: highUp | Kenney | CC0 | https://kenney.nl/assets/digital-audio#highUp | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-phaserup5.ogg | Kenney Digital Audio: phaserUp5 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#phaserUp5 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-powerup7.ogg | Kenney Digital Audio: powerUp7 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#powerUp7 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-powerup11.ogg | Kenney Digital Audio: powerUp11 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#powerUp11 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-twotone1.ogg | Kenney Digital Audio: twoTone1 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#twoTone1 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-zapthreetoneup.ogg | Kenney Digital Audio: zapThreeToneUp | Kenney | CC0 | https://kenney.nl/assets/digital-audio#zapThreeToneUp | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-phasejump3.ogg | Kenney Digital Audio: phaseJump3 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#phaseJump3 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-phaserdown2.ogg | Kenney Digital Audio: phaserDown2 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#phaserDown2 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-rpg-audio-handlecoins.ogg | Kenney RPG Audio: handleCoins | Kenney | CC0 | https://kenney.nl/assets/rpg-audio#handleCoins | Lizenz erkannt (Kenney) |
| kandidaten/kenney-rpg-audio-handlecoins2.ogg | Kenney RPG Audio: handleCoins2 | Kenney | CC0 | https://kenney.nl/assets/rpg-audio#handleCoins2 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-pepsound3.ogg | Kenney Digital Audio: pepSound3 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#pepSound3 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-impact-sounds-impactpunch-medium.ogg | Kenney Impact Sounds: impactPunch_medium_000 | Kenney | CC0 | https://kenney.nl/assets/impact-sounds#impactPunch_medium_000 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-impact-sounds-impactsoft-medium-0.ogg | Kenney Impact Sounds: impactSoft_medium_000 | Kenney | CC0 | https://kenney.nl/assets/impact-sounds#impactSoft_medium_000 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-impact-sounds-impactmetal-light-0.ogg | Kenney Impact Sounds: impactMetal_light_000 | Kenney | CC0 | https://kenney.nl/assets/impact-sounds#impactMetal_light_000 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-impact-sounds-impactbell-heavy-00.ogg | Kenney Impact Sounds: impactBell_heavy_000 | Kenney | CC0 | https://kenney.nl/assets/impact-sounds#impactBell_heavy_000 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-interface-sounds-bong-001.ogg | Kenney Interface Sounds: bong_001 | Kenney | CC0 | https://kenney.nl/assets/interface-sounds#bong_001 | Lizenz erkannt (Kenney) |
| kandidaten/kenney-digital-audio-threetone2.ogg | Kenney Digital Audio: threeTone2 | Kenney | CC0 | https://kenney.nl/assets/digital-audio#threeTone2 | Lizenz erkannt (Kenney) |
| kandidaten/happy-adventure-loop.ogg | Happy Adventure (Loop) | TinyWorlds | CC0 | https://opengameart.org/content/happy-adventure-loop | Lizenz geprüft (Claude, 2026-10-22, CC0-Link auf der Seite) |
| kandidaten/flowerbed-fields-loop.ogg | Flowerbed Fields [Loop] | Dylan_B | CC0 | https://opengameart.org/content/flowerbed-fields-loop | Lizenz erkannt (OpenGameArt) |
| kandidaten/lasso-lady-seamless-loop.ogg | Lasso Lady (seamless loop) | looneybits | CC0 | https://opengameart.org/content/lasso-lady-seamless-loop | Lizenz erkannt (OpenGameArt) |
| kandidaten/chiptune-medieval-the-old-tower-inn.ogg | [Chiptune] Medieval: The Old Tower Inn | RandomMind | CC0 | https://opengameart.org/content/chiptune-medieval-the-old-tower-inn | Lizenz geprüft (Claude, 2026-10-22, CC0-Link auf der Seite) |
| kandidaten/haunting-chiptune-loop-void-estate.ogg | Haunting Chiptune Loop [Void Estate] | Zane Little Music | CC0 | https://opengameart.org/content/haunting-chiptune-loop-void-estate | Lizenz geprüft (Claude, 2026-10-22, CC0-Link auf der Seite) |
| kandidaten/delayed-chiptune.ogg | Delayed Chiptune | DraymaDev | CC0 | https://opengameart.org/content/delayed-chiptune | Lizenz erkannt (OpenGameArt) |
| kandidaten/8-bit-battle-loop.ogg | 8-Bit Battle Loop | AntumDeluge | CC0 | https://opengameart.org/content/8-bit-battle-loop | Lizenz erkannt (OpenGameArt) |
| kandidaten/confrontation.ogg | Confrontation | DraymaDev | CC0 | https://opengameart.org/content/confrontation-1 | Lizenz erkannt (OpenGameArt) |
| kandidaten/pixel-sprinter.ogg | Pixel Sprinter | Zane Little Music | CC0 | https://opengameart.org/content/pixel-sprinter | Lizenz geprüft (Claude, 2026-10-22, CC0-Link auf der Seite) |
| kandidaten/8-bit-cave-loop.ogg | 8-Bit Cave Loop | Wolfgang_ | CC0 | https://opengameart.org/content/8-bit-cave-loop | Lizenz geprüft (Claude, 2026-10-22, CC0-Link auf der Seite) |
| kandidaten/nes-shooter-music-boss.ogg | NES Shooter Music: Boss | SketchyLogic | CC0 | https://opengameart.org/content/nes-shooter-music-5-tracks-3-jingles#Boss | Lizenz geprüft (Claude, 2026-10-22, CC0-Link auf der Seite) |
| kandidaten/gods-forbid.ogg | Gods Forbid | Mjorlaja | CC0 | https://opengameart.org/content/gods-forbid | Lizenz erkannt (OpenGameArt) |
| kandidaten/nes-shooter-music-map.ogg | NES Shooter Music: Map | SketchyLogic | CC0 | https://opengameart.org/content/nes-shooter-music-5-tracks-3-jingles#Map | Lizenz geprüft (Claude, 2026-10-22, CC0-Link auf der Seite) |
| kandidaten/chiptune-medieval-the-bard-s-tale.ogg | [Chiptune] Medieval: The Bard's Tale | Snabisch | CC0 | https://opengameart.org/content/chiptune-medieval-the-bards-tale | Lizenz erkannt (OpenGameArt) |
| kandidaten/8-bit-victory-loop.ogg | 8-Bit Victory Loop | Wolfgang_ | CC0 | https://opengameart.org/content/8-bit-victory-loop | Lizenz geprüft (Claude, 2026-10-22, CC0-Link auf der Seite) |
| kandidaten/a-big-adventure.ogg | A Big Adventure | Alexandr Zhelanov | CC-BY 4.0 | https://opengameart.org/content/a-big-adventure | bestätigt 🧑 2026-10-18 |
| kandidaten/the-adventure-begins.wav | The Adventure Begins. | mismatchedmisfitz | CC0 | https://opengameart.org/content/the-adventure-begins | bestätigt 🧑 2026-10-18 |
| kandidaten/summer-adventure-background-music.mp3 | Summer Adventure background music | dionelou | CC-BY 4.0 | https://opengameart.org/content/summer-adventure-background-music | bestätigt 🧑 2026-10-18 |
| kandidaten/horse-gallop-loop-01.ogg | Horse Gallop Loop 01 | AntumDeluge | CC-BY 3.0 | https://opengameart.org/content/horse-gallop-loop#gallop_01 | Lizenz erkannt (OpenGameArt) |
| kandidaten/horse-gallop-loop-02.ogg | Horse Gallop Loop 02 | AntumDeluge | CC-BY 3.0 | https://opengameart.org/content/horse-gallop-loop#gallop_02 | Lizenz erkannt (OpenGameArt) |
| kandidaten/horse-gallop-on-different-surfaces.mp3 | Horse gallop on different surfaces | medicinestorm | CC-BY 4.0 | https://opengameart.org/content/horse-gallop-on-different-surfaces | Lizenz erkannt (OpenGameArt) |
| kandidaten/selbst-erzeugt-niederlage.ogg | Niederlage (Schleife) | Claude (selbst erzeugt) | CC0 | selbst erzeugt, Rezept im Abschnitt „Selbst erzeugte Töne“ | erzeugt 2026-10-23 (B-573) |
| kandidaten/selbst-erzeugt-ui-select.ogg | UI-Klick Auswahl | Claude (selbst erzeugt) | CC0 | selbst erzeugt, Rezept im Abschnitt „Selbst erzeugte Töne“ | erzeugt 2026-10-23 (B-575) |
| kandidaten/selbst-erzeugt-ui-confirm.ogg | UI-Klick Bestätigen | Claude (selbst erzeugt) | CC0 | selbst erzeugt, Rezept im Abschnitt „Selbst erzeugte Töne“ | erzeugt 2026-10-23 (B-575) |

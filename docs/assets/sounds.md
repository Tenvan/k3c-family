# Sound-Katalog

Katalog Ereignis → Sound → Quelle → Lizenz (B-167, SO2). Die Zuordnung steht im Slot-Register `data/assets.json` (Reihenfolge = Präferenz, der erste Sound ist aktiv), Urheber und Lizenz in `public/audio/README.md` › Importe (k3c-dev). Stand 2026-10-18, Auswahl durch 🧑 im Asset-Studio. Lizenz-Bestätigung in k3c-dev durch 🧑 steht aus. Einbau ins Spiel: SO2.3.

## Effekte

| Ereignis | Slot | Sound (aktiv) | Quelle | Urheber | Lizenz | Status |
|---|---|---|---|---|---|---|
| Treffer | `sfx:hit` | `audio/kandidaten/kenney-impact-sounds-impactpunch-medium.ogg` (+2) | https://kenney.nl/assets/impact-sounds | Kenney | CC0 | zugeordnet |
| Gegner-Tod | `sfx:kill` | `audio/kandidaten/kenney-digital-audio-spacetrash2.ogg` (+1) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Münze aufheben | `sfx:coin` | `audio/kandidaten/kenney-rpg-audio-handlecoins.ogg` (+2) | https://kenney.nl/assets/rpg-audio | Kenney | CC0 | zugeordnet |
| Münze geben | – | – | – | – | – | **Lücke** (B-492) |
| Pfeil | `sfx:arrow` | `audio/kandidaten/kenney-digital-audio-phaserup2.ogg` (+2) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Schlag | `sfx:strike` | `audio/kandidaten/kenney-rpg-audio-chop.ogg` (+2) | https://kenney.nl/assets/rpg-audio | Kenney | CC0 | zugeordnet |
| Bau-Fortschritt | – | – | – | – | – | **Lücke** (B-492) |
| Bau fertig | `sfx:build` | `audio/kandidaten/kenney-rpg-audio-metallatch.ogg` (+1) | https://kenney.nl/assets/rpg-audio | Kenney | CC0 | zugeordnet |
| Tod | `sfx:death` | `audio/kandidaten/kenney-digital-audio-lowdown.ogg` (+1) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Skill | `sfx:skill` | `audio/kandidaten/kenney-digital-audio-powerup11.ogg` (+2) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Nacht naht | `sfx:warn` | `audio/kandidaten/kenney-impact-sounds-impactbell-heavy-00.ogg` (+2) | https://kenney.nl/assets/impact-sounds | Kenney | CC0 | zugeordnet |
| Portal öffnet | `sfx:portal` | `audio/kandidaten/kenney-digital-audio-phasejump3.ogg` (+2) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Respawn (`revive`) | `sfx:revive` | `audio/kandidaten/kenney-digital-audio-powerup3.ogg` (+2) | https://kenney.nl/assets/digital-audio | Kenney | CC0 | zugeordnet |
| Wiederbeleben durch Mitspieler (`revived`) | – | – | – | – | – | **Lücke** (B-492) |
| Hub-Ausbau | – | – | – | – | – | **Lücke** (B-492) |
| Skill je Klasse | – | – | – | – | – | **Lücke** (B-492) |
| Boss-Auftritt | – | – | – | – | – | **Lücke** (B-492) |
| UI-Klicks | – | – | – | – | – | **Lücke** (B-492) |
| Reiten (Hufschlag, Schleife) | `sfx:ride` | `audio/kandidaten/horse-gallop-loop-01.ogg` (+2) | https://opengameart.org/content/horse-gallop-loop | AntumDeluge | CC-BY 3.0 | zugeordnet, Einbau offen (B-492) |

Lücken (Münze geben, Bau-Fortschritt, `revived`, Hub-Ausbau, Skill je Klasse, Boss-Auftritt, UI-Klicks) führt B-492; für Reiten ist der Slot `sfx:ride` angelegt und mit Kandidaten belegt, der Einbau (Schleife je berittenem Spieler) steht in B-492; „Selbst erzeugt“-Füller waren nicht nötig, weil alle Pflicht-Ereignisse (Q08) einen Sound haben.

## Musik (Stand SO2, Auswahl und Einbau in SO4)

| Zustand | Slot | Sound (aktiv) | Quelle | Urheber | Lizenz | Status |
|---|---|---|---|---|---|---|
| Tag | `music:tag` | `audio/kandidaten/happy-adventure-loop.ogg` (+3) | https://opengameart.org/content/happy-adventure-loop | fluffrabbit | CC0 | zugeordnet |
| Abend | `music:abend` | `audio/kandidaten/chiptune-medieval-the-old-tower-inn.ogg` | https://opengameart.org/content/chiptune-medieval-the-old-tower-inn | Snabisch | CC0 | zugeordnet |
| Nacht | `music:nacht` | `audio/kandidaten/haunting-chiptune-loop-void-estate.ogg` (+1) | https://opengameart.org/content/haunting-chiptune-loop-void-estate | -Ryu- | CC0 | zugeordnet |
| Kampf | `music:kampf` | `audio/kandidaten/pixel-sprinter.ogg` (+2) | https://opengameart.org/content/pixel-sprinter | Zamoar | CC0 | zugeordnet |
| Tiefe/Höhle | `music:tiefe` | `audio/kandidaten/8-bit-cave-loop.ogg` | https://opengameart.org/content/8-bit-cave-loop | haxx | CC0 | zugeordnet |
| Boss | `music:boss` | `audio/kandidaten/nes-shooter-music-boss.ogg` (+1) | https://opengameart.org/content/nes-shooter-music-5-tracks-3-jingles | chrismthegamedude | CC0 | zugeordnet |
| Lobby | `music:lobby` | `audio/kandidaten/nes-shooter-music-map.ogg` (+1) | https://opengameart.org/content/nes-shooter-music-5-tracks-3-jingles | chrismthegamedude | CC0 | zugeordnet |
| Niederlage/Sieg | `music:ende` | `audio/kandidaten/8-bit-victory-loop.ogg` | https://opengameart.org/content/8-bit-victory-loop | josepharaoh99 | CC0 | zugeordnet (nur Sieg, **Lücke** Niederlage, B-168) |

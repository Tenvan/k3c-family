# Sound-Atlas (SO1.2)

`atlas.ogg` und `atlas.mp3`: ein Audio-Sprite mit drei Sinustönen (0,8 s, mono). `atlas.json` nennt die Dateien je Format
und Start/Dauer je Sound (Sekunden). **Selbst erzeugt, keine fremden Inhalte** (kein Credit nötig); Lückenfüller bis SO2.

| Sound | Start | Dauer | Ton |
|---|---|---|---|
| `coin` | 0,0 s | 0,2 s | 880 Hz |
| `hit` | 0,3 s | 0,2 s | 220 Hz |
| `warn` | 0,6 s | 0,2 s | 440 Hz |

Erzeugt mit ffmpeg (`aevalsrc` mit Stille dazwischen, mp3 hat ~25 ms Encoder-Verzögerung, die Pausen fangen das ab):

```bash
E="if(lt(t,0.2),sin(2*PI*880*t),if(lt(t,0.3),0,if(lt(t,0.5),sin(2*PI*220*t),if(lt(t,0.6),0,sin(2*PI*440*t)))))*0.5"
ffmpeg -f lavfi -i "aevalsrc='$E':d=0.8:s=44100" -ac 1 -map_metadata -1 -c:a libvorbis -q:a 2 atlas.ogg
ffmpeg -f lavfi -i "aevalsrc='$E':d=0.8:s=44100" -ac 1 -map_metadata -1 -c:a libmp3lame -b:a 48k atlas.mp3
```

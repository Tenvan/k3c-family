# Audio-Testdateien

Testtöne für den Audio-Abschnitt der Gamepad-Testseite (`gamepad-test.html`, B-166): je 0,3 s Sinuston 440 Hz, mono.
**Selbst erzeugt, keine fremden Inhalte** (kein Credit nötig).

Erzeugt mit dem ffmpeg aus dem Python-Paket `imageio-ffmpeg` (temporär, außerhalb des Repos):

```bash
FF=$(uvx --from imageio-ffmpeg python -c "import imageio_ffmpeg; print(imageio_ffmpeg.get_ffmpeg_exe())")
S="-f lavfi -i sine=frequency=440:duration=0.3 -ac 1 -map_metadata -1 -y"
"$FF" $S -c:a pcm_s16le -ar 22050 test.wav
"$FF" $S -c:a libvorbis -q:a 0 test.ogg
"$FF" $S -c:a aac -b:a 64k test.m4a
"$FF" $S -c:a libmp3lame -b:a 64k test.mp3
```

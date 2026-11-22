package playtest

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
)

// ErrNoVideo: ohne video.mkv gibt es keine Bilder.
var ErrNoVideo = errors.New("video.mkv fehlt (gelöscht?), kein Bild möglich")

// FrameAt zieht das Bild bei Zeitpunkt tsMs (Unix-ms) aus video.mkv nach frames/t-<tsMs>.jpg und liefert den
// absoluten Pfad; ein vorhandenes Bild wird wiederverwendet. Zeitpunkte vor dem Start der Aufnahme gelten als 0 s.
func (t *Transcriber) FrameAt(id string, tsMs int64) (string, error) {
	ses, err := t.Store.Get(id)
	if err != nil {
		return "", err
	}
	if !ses.HasVideo {
		return "", ErrNoVideo
	}
	ff, err := t.LookPath("ffmpeg")
	if err != nil {
		return "", ErrNoFFmpeg
	}
	dir := filepath.Join(t.Store.Root, ses.ID)
	rel := dirFrames + "/t-" + strconv.FormatInt(tsMs, 10) + ".jpg"
	out := filepath.Join(dir, filepath.FromSlash(rel))
	if st, err := os.Stat(out); err == nil && !st.IsDir() {
		return out, nil
	}
	if err := os.MkdirAll(filepath.Dir(out), 0o755); err != nil {
		return "", err
	}
	sek := float64(max(tsMs-CaptureStart(readMeta(dir)), 0)) / 1000
	proc, err := t.Spawn(FrameArgs(ff, sek, filepath.FromSlash(rel)), dir)
	if err != nil {
		return "", fmt.Errorf("Bild nicht erzeugt: %w", err)
	}
	if err := proc.Wait(); err != nil {
		return "", fmt.Errorf("Bild nicht erzeugt: %v: %s", err, proc.Tail())
	}
	if _, err := os.Stat(out); err != nil {
		return "", fmt.Errorf("Bild nicht erzeugt: %w", err)
	}
	return out, nil
}

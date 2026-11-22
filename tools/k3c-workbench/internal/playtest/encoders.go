package playtest

import (
	"strings"
	"sync"
)

// Weitere Encoder neben EncoderQSV und EncoderX264 (record.go). Die Bezeichnung steht in session.json und in der Anzeige.
const (
	EncoderNVENC = "h264_nvenc 1080p30"
	EncoderAMF   = "h264_amf 1080p30"
)

// encoderOrder: Hardware-Encoder zuerst (schont die CPU beim Spielen), libx264 als sicherer Rückfall.
var encoderOrder = []string{EncoderNVENC, EncoderAMF, EncoderQSV, EncoderX264}

// encoderSpec liefert die Skalierung (läuft nach hwdownload) und die Codec-Argumente des Encoders.
func encoderSpec(enc string) (scale string, codec []string) {
	const hd = "scale=1920:1080:flags=fast_bilinear,format=nv12"
	switch enc {
	case EncoderX264:
		return "scale=1280:720:flags=fast_bilinear,format=yuv420p", []string{"-c:v", "libx264", "-preset", "ultrafast", "-crf", "28"}
	case EncoderNVENC:
		return hd, []string{"-c:v", "h264_nvenc", "-preset", "p1", "-tune", "ll", "-rc", "vbr", "-cq", "28", "-b:v", "0"}
	case EncoderAMF:
		return hd, []string{"-c:v", "h264_amf", "-quality", "speed", "-rc", "cqp", "-qp_i", "26", "-qp_p", "28"}
	default: // EncoderQSV
		return hd, []string{"-c:v", "h264_qsv", "-global_quality", "26", "-look_ahead", "0"}
	}
}

// EncoderProbeArgs ist ein kurzer echter Encode (10 Bilder aus einer Testquelle), der zeigt, ob sich der Encoder
// auf diesem Rechner öffnen lässt – ein Encoder, der im ffmpeg-Build steht, aber keine passende GPU findet, scheitert hier.
func EncoderProbeArgs(ff, enc string) []string {
	scale, codec := encoderSpec(enc)
	a := []string{ff, "-hide_banner", "-f", "lavfi", "-i", "color=c=black:s=1920x1080:r=30", "-frames:v", "10", "-vf", scale}
	a = append(a, codec...)
	return append(a, "-f", "null", "-")
}

// PickEncoder wählt den ersten Encoder, der sich wirklich öffnen lässt; libx264 gilt ohne Probe als Rückfall.
// `failed` nennt die verworfenen Encoder mit dem Grund, fürs Log.
func PickEncoder(run probe, ff string) (enc string, failed []string) {
	for _, e := range encoderOrder {
		if e == EncoderX264 {
			break
		}
		out, err := run(EncoderProbeArgs(ff, e))
		if err == nil {
			return e, failed
		}
		failed = append(failed, strings.Fields(e)[0]+": "+lastError(out))
	}
	return EncoderX264, failed
}

var (
	pickMu    sync.Mutex
	pickCache = map[string]string{} // ffmpeg-Pfad → Encoder; die Probe öffnet GPU-Sitzungen, einmal je Prozess genügt
)

// pickCached liefert den Encoder aus dem Cache, sonst nach der Probe; pickLogged und ListDevices teilen ihn,
// damit die Geräteliste der Seite nie neben einer laufenden Aufnahme erneut probt.
func pickCached(ff string, log func(level, msg string, kv ...any)) string {
	pickMu.Lock()
	defer pickMu.Unlock()
	if enc, ok := pickCache[ff]; ok {
		return enc
	}
	enc := pickLogged(ff, log)
	pickCache[ff] = enc
	return enc
}

// pickLogged wählt den Encoder und schreibt die verworfenen mit Grund ins Log.
func pickLogged(ff string, log func(level, msg string, kv ...any)) string {
	enc, failed := PickEncoder(runProbe, ff)
	if log != nil {
		for _, f := range failed {
			log("info", "🐛 Encoder nicht nutzbar: "+f)
		}
		log("info", "🐛 Encoder gewählt: "+enc)
	}
	return enc
}

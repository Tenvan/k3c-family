package playtest

import (
	"errors"
	"strings"
	"testing"
)

func TestPickEncoderNimmtDenErstenFunktionierenden(t *testing.T) {
	var asked []string
	run := func(a []string) (string, error) {
		asked = append(asked, strings.Join(a, " "))
		if strings.Contains(strings.Join(a, " "), "h264_amf") {
			return "", nil
		}
		return "Error creating a MFX session\nError while opening encoder", errors.New("exit")
	}
	enc, failed := PickEncoder(run, "ff")
	if enc != EncoderAMF {
		t.Fatalf("AMF erwartet: %q", enc)
	}
	if len(failed) != 1 || !strings.HasPrefix(failed[0], "h264_nvenc: ") {
		t.Fatalf("verworfen: %v", failed)
	}
}

func TestPickEncoderFaelltAufX264ZurueckOhneEineProbe(t *testing.T) {
	calls := 0
	run := func([]string) (string, error) { calls++; return "kaputt", errors.New("exit") }
	enc, failed := PickEncoder(run, "ff")
	if enc != EncoderX264 || len(failed) != 3 || calls != 3 {
		t.Fatalf("%q %v %d", enc, failed, calls)
	}
}

func TestFFmpegArgsJederEncoder(t *testing.T) {
	for _, enc := range encoderOrder {
		a := strings.Join(FFmpegArgs("ffmpeg", "D", "Mikro", 0, enc), " ")
		codec := strings.Fields(enc)[0]
		if !strings.Contains(a, "-c:v "+codec) || !strings.Contains(a, "-cluster_time_limit 1000") || !strings.Contains(a, "-flush_packets 1") {
			t.Errorf("%s: %s", enc, a)
		}
	}
}

func TestRecorderNutztGeprueftenEncoder(t *testing.T) {
	g := newRig(t)
	g.rec.Pick = func(string) string { return EncoderAMF }
	st, err := g.rec.Start(Options{Mic: "M"})
	if err != nil || st.Encoder != EncoderAMF {
		t.Fatalf("%v %+v", err, st)
	}
	_ = g.rec.Stop()
	g.rec.Wait()
}

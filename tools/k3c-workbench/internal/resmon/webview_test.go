package resmon

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"
)

// B-895/AC-03: Überschreitet der WebView2-Renderer die Grenze, steht ein 🤒-Eintrag im Log, einmal je Stufe.
func TestWebViewRendererUeberGrenzeWarnt(t *testing.T) {
	var buf bytes.Buffer
	m := New(Config{WebViewWarn: 500 << 20, Log: slog.New(slog.NewTextHandler(&buf, nil))})
	warns := func() int { return strings.Count(buf.String(), "🤒 webview-renderer") }

	m.checkThresholds(Sample{WebView: 400 << 20})
	if warns() != 0 {
		t.Fatalf("unter der Grenze gewarnt: %s", buf.String())
	}
	m.checkThresholds(Sample{WebView: 600 << 20})
	if warns() != 1 || !strings.Contains(buf.String(), formatBytes(600<<20)) {
		t.Fatalf("über der Grenze keine Warnung: %s", buf.String())
	}
	m.checkThresholds(Sample{WebView: 700 << 20}) // nächste Stufe erst bei 900 MiB
	if warns() != 1 {
		t.Fatalf("Warnung wiederholt: %s", buf.String())
	}
	m.checkThresholds(Sample{WebView: 950 << 20})
	if warns() != 2 {
		t.Fatalf("nächste Stufe ohne Warnung: %s", buf.String())
	}
}

func TestWebViewVorgabeUndBericht(t *testing.T) {
	if New(Config{}).cfg.WebViewWarn != DefaultWebViewWarn || DefaultWebViewWarn != 500<<20 {
		t.Fatal("Vorgabe der Renderer-Grenze ist nicht 500 MiB")
	}
	if got := summary(Sample{WebView: 600 << 20}); !strings.Contains(got, "Renderer "+formatBytes(600<<20)) {
		t.Fatalf("Zusammenfassung ohne Renderer: %s", got)
	}
	if got := summary(Sample{}); strings.Contains(got, "Renderer") {
		t.Fatalf("Renderer ohne Fenster genannt: %s", got)
	}
}

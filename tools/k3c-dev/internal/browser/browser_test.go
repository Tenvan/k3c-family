package browser

import (
	"errors"
	"path/filepath"
	"slices"
	"testing"
)

func TestFindBekannteOrte(t *testing.T) {
	env := map[string]string{"ProgramFiles(x86)": `C:\PF86`, "ProgramFiles": `C:\PF`, "LOCALAPPDATA": `C:\LA`}
	chrome := filepath.Join(`C:\LA`, "Google", "Chrome", "Application", "chrome.exe")
	got, err := find("windows", func(k string) string { return env[k] }, func(p string) bool { return p == chrome })
	if err != nil || got != chrome {
		t.Errorf("find = %q, %v; erwartet %q", got, err, chrome)
	}
	edge := filepath.Join(`C:\PF86`, "Microsoft", "Edge", "Application", "msedge.exe")
	got, _ = find("windows", func(k string) string { return env[k] }, func(p string) bool { return p == chrome || p == edge })
	if got != edge {
		t.Errorf("Edge zuerst erwartet, bekam %q", got)
	}
	if _, err := find("linux", func(string) string { return "" }, func(string) bool { return false }); !errors.Is(err, ErrNotFound) {
		t.Errorf("ohne Browser: %v", err)
	}
}

func TestArgsHeadlessMitEigenemProfil(t *testing.T) {
	args := Args("edge.exe", "prof", "http://127.0.0.1:5183/game.html")
	for _, want := range []string{"--headless=new", "--user-data-dir=prof", "http://127.0.0.1:5183/game.html"} {
		if !slices.Contains(args, want) {
			t.Errorf("%v ohne %s", args, want)
		}
	}
}

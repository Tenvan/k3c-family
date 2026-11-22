package playtest

import (
	"errors"
	"path/filepath"
	"strings"
	"testing"
)

func fakeFinder(env map[string]string, onPath string, regDirs []string, files map[string]bool, wing []string) finder {
	return finder{
		env: func(k string) string { return env[k] },
		lookPath: func(string) (string, error) {
			if onPath != "" {
				return onPath, nil
			}
			return "", errors.New("nicht im PATH")
		},
		regPaths: func() []string { return regDirs },
		glob:     func(string) ([]string, error) { return wing, nil },
		isFile:   func(p string) bool { return files[p] },
	}
}

func TestFindFFmpegReihenfolge(t *testing.T) {
	reg := filepath.Join("C:", "ff", "bin")
	regExe := filepath.Join(reg, exeName())
	// 1. Variable hat Vorrang vor PATH
	f := fakeFinder(map[string]string{FFmpegEnv: "X:/mein/ffmpeg.exe"}, "C:/pfad/ffmpeg.exe", nil, map[string]bool{"X:/mein/ffmpeg.exe": true}, nil)
	if p, err := f.find("ffmpeg"); err != nil || p != "X:/mein/ffmpeg.exe" {
		t.Fatalf("Variable: %q %v", p, err)
	}
	// 2. PATH vor Registry
	f = fakeFinder(nil, "C:/pfad/ffmpeg.exe", []string{reg}, map[string]bool{regExe: true}, nil)
	if p, _ := f.find("ffmpeg"); p != "C:/pfad/ffmpeg.exe" {
		t.Fatalf("PATH: %q", p)
	}
	// 3. Registry-PATH, wenn der Prozess-PATH veraltet ist
	f = fakeFinder(nil, "", []string{reg}, map[string]bool{regExe: true}, nil)
	if p, err := f.find("ffmpeg"); err != nil || p != regExe {
		t.Fatalf("Registry: %q %v", p, err)
	}
	// 4. WinGet-Ordner, neueste Version zuletzt
	f = fakeFinder(map[string]string{"LOCALAPPDATA": "C:/L"}, "", nil, nil, []string{"alt/ffmpeg.exe", "neu/ffmpeg.exe"})
	if p, err := f.find("ffmpeg"); err != nil || p != "neu/ffmpeg.exe" {
		t.Fatalf("WinGet: %q %v", p, err)
	}
}

func TestFindFFmpegFehlerNenntOrte(t *testing.T) {
	f := fakeFinder(map[string]string{FFmpegEnv: "X:/weg.exe", "LOCALAPPDATA": "C:/L"}, "", []string{"D:/a"}, nil, nil)
	_, err := f.find("ffmpeg")
	if !errors.Is(err, ErrNoFFmpeg) {
		t.Fatalf("kein ErrNoFFmpeg: %v", err)
	}
	for _, want := range []string{FFmpegEnv + "=X:/weg.exe", "PATH des Prozesses", "PATH aus der Registry", "WinGet-Ordner"} {
		if !strings.Contains(err.Error(), want) {
			t.Errorf("Meldung ohne %q: %v", want, err)
		}
	}
}

func TestParseMonitorSize(t *testing.T) {
	out := "Input #0, lavfi, from 'ddagrab=output_idx=0':\n  Stream #0:0: Video: wrapped_avframe, bgra(tv, bt709), 2560x1440 [SAR 1:1 DAR 16:9], 1 fps\n"
	if w, h := ParseMonitorSize(out); w != 2560 || h != 1440 {
		t.Fatalf("%dx%d", w, h)
	}
	if w, h := ParseMonitorSize("nichts"); w != 0 || h != 0 {
		t.Fatalf("%dx%d", w, h)
	}
}

func TestListMonitorsBisZumErstenFehler(t *testing.T) {
	var calls []string
	run := func(args []string) (string, error) {
		calls = append(calls, args[5])
		if strings.Contains(args[5], "output_idx=2") {
			return "Invalid output index", errors.New("exit 1")
		}
		return "Stream #0:0: Video: wrapped_avframe, 1920x1080 [SAR 1:1]", nil
	}
	mons := listMonitors(run, "ff")
	if len(mons) != 2 || mons[1].Index != 1 || mons[0].Label != "Monitor 1 (1920×1080)" {
		t.Fatalf("%+v", mons)
	}
	if len(calls) != 3 {
		t.Fatalf("Proben: %v", calls)
	}
	if got := listMonitors(func([]string) (string, error) { return "", errors.New("x") }, "ff"); len(got) != 0 {
		t.Fatalf("ohne Desktop: %+v", got)
	}
}

func TestCheckDevices(t *testing.T) {
	ok := func([]string) (string, error) { return "", nil }
	if err := CheckDevices(ok, "ff", "Raummikro", 0); err != nil {
		t.Fatal(err)
	}
	badMic := func(a []string) (string, error) {
		if a[3] == "dshow" {
			return "Could not run graph\nI/O error", errors.New("exit")
		}
		return "", nil
	}
	err := CheckDevices(badMic, "ff", "Raummikro", 0)
	if err == nil || !strings.Contains(err.Error(), "Raummikro") || !strings.Contains(err.Error(), "I/O error") {
		t.Fatalf("Mikro: %v", err)
	}
	badMon := func(a []string) (string, error) {
		if a[3] == "lavfi" {
			return "Invalid output index", errors.New("exit")
		}
		return "", nil
	}
	err = CheckDevices(badMon, "ff", "M", 3)
	if err == nil || !strings.Contains(err.Error(), "Monitor 4") {
		t.Fatalf("Monitor: %v", err)
	}
}

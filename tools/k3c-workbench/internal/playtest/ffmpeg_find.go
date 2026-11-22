package playtest

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
)

// FFmpegEnv nennt die ffmpeg.exe ausdrücklich (hat Vorrang vor allem anderen, B-1230).
const FFmpegEnv = "K3C_FFMPEG"

// finder sucht eine Programmdatei; die Quellen sind austauschbar, damit der Test sie ohne Rechner-Zustand prüft.
type finder struct {
	env      func(string) string
	lookPath func(string) (string, error)
	regPaths func() []string // aktueller User- und Machine-PATH aus der Registry (nur Windows)
	glob     func(string) ([]string, error)
	isFile   func(string) bool
}

func realFinder() finder {
	return finder{env: os.Getenv, lookPath: exec.LookPath, regPaths: registryPathDirs, glob: filepath.Glob, isFile: isRegular}
}

func isRegular(p string) bool {
	st, err := os.Stat(p)
	return err == nil && st.Mode().IsRegular()
}

// FindFFmpeg liefert den Pfad zur ffmpeg: `K3C_FFMPEG`, dann der PATH des Prozesses, dann der aktuelle PATH aus der
// Registry (ein vor der Installation gestarteter Prozess kennt ihn nicht), dann der WinGet-Ordner. Der Fehler
// nennt die geprüften Orte. `name` ist der Programmname (z. B. „ffmpeg“); fremde Namen laufen nur über den PATH.
func FindFFmpeg(name string) (string, error) { return realFinder().find(name) }

func (f finder) find(name string) (string, error) {
	if name != "ffmpeg" {
		return f.lookPath(name)
	}
	tried := []string{}
	if v := strings.TrimSpace(f.env(FFmpegEnv)); v != "" {
		if f.isFile(v) {
			return v, nil
		}
		tried = append(tried, FFmpegEnv+"="+v+" (Datei fehlt)")
	}
	if p, err := f.lookPath("ffmpeg"); err == nil {
		return p, nil
	}
	tried = append(tried, "PATH des Prozesses")
	if runtime.GOOS == "windows" || f.regPaths != nil {
		var dirs []string
		if f.regPaths != nil {
			dirs = f.regPaths()
		}
		for _, d := range dirs {
			if p := filepath.Join(d, exeName()); f.isFile(p) {
				return p, nil
			}
		}
		tried = append(tried, "PATH aus der Registry")
		for _, pat := range f.wingetPatterns() {
			if m, _ := f.glob(pat); len(m) > 0 {
				return m[len(m)-1], nil // neueste Version zuletzt
			}
		}
		tried = append(tried, "WinGet-Ordner")
	}
	return "", &NotFoundError{Tried: tried}
}

func (f finder) wingetPatterns() []string {
	base := f.env("LOCALAPPDATA")
	if base == "" {
		return nil
	}
	return []string{filepath.Join(base, "Microsoft", "WinGet", "Packages", "Gyan.FFmpeg*", "ffmpeg-*", "bin", exeName())}
}

func exeName() string {
	if runtime.GOOS == "windows" {
		return "ffmpeg.exe"
	}
	return "ffmpeg"
}

// NotFoundError ist der Fehler von FindFFmpeg; er ist ErrNoFFmpeg und nennt die geprüften Orte.
type NotFoundError struct{ Tried []string }

func (e *NotFoundError) Error() string {
	return ErrNoFFmpeg.Error() + "; geprüft: " + strings.Join(e.Tried, ", ") + "; oder " + FFmpegEnv + " auf die ffmpeg.exe setzen"
}

// Is lässt `errors.Is(err, ErrNoFFmpeg)` gelten.
func (e *NotFoundError) Is(target error) bool { return target == ErrNoFFmpeg }

// noFFmpeg macht aus dem Fehler der Suche einen ErrNoFFmpeg; hat FindFFmpeg die geprüften Orte genannt, bleibt der Text.
func noFFmpeg(err error) error {
	if errors.Is(err, ErrNoFFmpeg) {
		return err
	}
	return ErrNoFFmpeg
}

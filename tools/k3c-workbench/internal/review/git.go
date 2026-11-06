package review

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"slices"
	"strconv"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/proc"
)

// Ablage der Specs (paper-spec.md › Dateien): public/grafik/<gruppe>/<id>.paper.json; Index und Stil sind keine Specs.
const (
	grafik   = "public/grafik"
	stilPfad = grafik + "/stil.paper.json"
	endung   = ".paper.json"
	gitZeit  = 30 * time.Second
)

// git führt git ohne Shell im Repo aus und liefert stdout; ein Fehler trägt stderr.
func git(ctx context.Context, root string, args ...string) ([]byte, error) {
	ctx, cancel := context.WithTimeout(ctx, gitZeit)
	defer cancel()
	cmd := proc.Command(ctx, append([]string{"git", "-c", "core.autocrlf=false", "-c", "core.quotepath=false"}, args...))
	cmd.Dir = root
	cmd.Env = append(os.Environ(), "GIT_TERMINAL_PROMPT=0", "GCM_INTERACTIVE=never")
	var out, errOut bytes.Buffer
	cmd.Stdout, cmd.Stderr = &out, &errOut
	if err := cmd.Run(); err != nil {
		return nil, fmt.Errorf("git %s: %w: %s", args[0], err, strings.TrimSpace(errOut.String()))
	}
	return out.Bytes(), nil
}

// reSegment: Gruppe bzw. Dateiname ohne Pfadteile (kein .., kein Backslash, kein Laufwerk).
var reSegment = regexp.MustCompile(`^[a-z0-9][a-z0-9._-]*$`)

// istSpec: genau public/grafik/<gruppe>/<id>.paper.json, ohne Gruppen-Index.
func istSpec(pfad string) bool {
	teile := strings.Split(pfad, "/")
	return len(teile) == 4 && teile[0]+"/"+teile[1] == grafik && strings.HasSuffix(teile[3], endung) &&
		teile[3] != "index"+endung && gueltigesSegment(teile[2]) && gueltigesSegment(strings.TrimSuffix(teile[3], endung))
}

func gueltigesSegment(s string) bool { return reSegment.MatchString(s) && !strings.Contains(s, "..") }

// istManifest: public/grafik/<gruppe>/<id>/manifest.json (Ausgabe eines Assets).
func istManifest(pfad string) bool {
	teile := strings.Split(pfad, "/")
	return len(teile) == 5 && teile[0]+"/"+teile[1] == grafik && teile[4] == "manifest.json" &&
		gueltigesSegment(teile[2]) && gueltigesSegment(teile[3])
}

// manifestZu ist das Manifest der Ausgabe neben der Spec.
func manifestZu(spec string) string { return strings.TrimSuffix(spec, endung) + "/manifest.json" }

// BasisDateien liest Specs, Manifeste und Stil-Datei des Commits mit zwei git-Aufrufen (ls-tree, cat-file --batch);
// einzelne `git show` je Datei wären unter Windows zu langsam.
func BasisDateien(ctx context.Context, root, commit string) (map[string][]byte, error) {
	out, err := git(ctx, root, "ls-tree", "-r", "-z", commit, "--", grafik)
	if err != nil {
		return nil, err
	}
	var pfade, shas []string
	for zeile := range strings.SplitSeq(strings.TrimRight(string(out), "\x00"), "\x00") {
		kopf, pfad, ok := strings.Cut(zeile, "\t")
		felder := strings.Fields(kopf)
		if !ok || len(felder) != 3 || felder[1] != "blob" {
			continue
		}
		if istSpec(pfad) || istManifest(pfad) || pfad == stilPfad {
			pfade, shas = append(pfade, pfad), append(shas, felder[2])
		}
	}
	return blobs(ctx, root, pfade, shas)
}

// blobs liest Blobs in einem Aufruf von `git cat-file --batch`.
func blobs(ctx context.Context, root string, pfade, shas []string) (map[string][]byte, error) {
	dateien := make(map[string][]byte, len(pfade))
	if len(shas) == 0 {
		return dateien, nil
	}
	ctx, cancel := context.WithTimeout(ctx, gitZeit)
	defer cancel()
	cmd := proc.Command(ctx, []string{"git", "cat-file", "--batch"})
	cmd.Dir = root
	cmd.Stdin = strings.NewReader(strings.Join(shas, "\n") + "\n")
	var out, errOut bytes.Buffer
	cmd.Stdout, cmd.Stderr = &out, &errOut
	if err := cmd.Run(); err != nil {
		return nil, fmt.Errorf("git cat-file: %w: %s", err, strings.TrimSpace(errOut.String()))
	}
	rest := out.Bytes()
	for i := range shas {
		kopf, nach, ok := bytes.Cut(rest, []byte("\n"))
		f := strings.Fields(string(kopf))
		if !ok || len(f) != 3 {
			return nil, fmt.Errorf("git cat-file: Antwort unlesbar bei %s", pfade[i])
		}
		n, err := strconv.Atoi(f[2])
		if err != nil || n+1 > len(nach) {
			return nil, fmt.Errorf("git cat-file: Größe unlesbar bei %s", pfade[i])
		}
		dateien[pfade[i]], rest = nach[:n], nach[n+1:]
	}
	return dateien, nil
}

// SpecsImArbeitsstand listet die Spec-Pfade im Dateisystem, auch unversionierte.
func SpecsImArbeitsstand(root string) ([]string, error) {
	treffer, err := filepath.Glob(filepath.Join(root, filepath.FromSlash(grafik), "*", "*"+endung))
	if err != nil {
		return nil, err
	}
	var pfade []string
	for _, abs := range treffer {
		rel, err := filepath.Rel(root, abs)
		if err != nil {
			return nil, err
		}
		if p := filepath.ToSlash(rel); istSpec(p) {
			pfade = append(pfade, p)
		}
	}
	slices.Sort(pfade)
	return pfade, nil
}

// Show liest eine Datei aus dem Commit; fehlt sie dort, ist ok false und err nil.
func Show(ctx context.Context, root, commit, pfad string) ([]byte, bool, error) {
	if _, err := git(ctx, root, "rev-parse", "--verify", "--quiet", commit+":"+pfad); err != nil {
		var exit *exec.ExitError
		if errors.As(err, &exit) && exit.ExitCode() == 1 {
			return nil, false, nil // --verify --quiet: Exit 1 = gibt es nicht
		}
		return nil, false, err
	}
	out, err := git(ctx, root, "cat-file", "blob", commit+":"+pfad)
	if err != nil {
		return nil, false, err
	}
	return out, true, nil
}

// leseArbeitsstand liest eine Datei relativ zum Checkout; fehlt sie, ist ok false.
func leseArbeitsstand(root, pfad string) ([]byte, bool, error) {
	raw, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(pfad)))
	if errors.Is(err, os.ErrNotExist) {
		return nil, false, nil
	}
	return raw, err == nil, err
}

package paperassets

import (
	"context"
	"path/filepath"
	"strings"
	"testing"
)

// Die Rig-Vorlagen bestehen die echte Prüfung (check.ts über die Attrappe); die Figur darf nur proportion melden.
func TestRigVorlagen(t *testing.T) {
	t.Parallel()
	_, cli := testRepo(t)
	for _, rig := range RigNamen() {
		t.Run(rig, func(t *testing.T) {
			t.Parallel()
			text, err := Vorlage(rig, "probe", "Probe")
			if err != nil {
				t.Fatal(err)
			}
			spec := filepath.Join(t.TempDir(), "probe.paper.json")
			schreibe(t, filepath.Dir(spec), "probe.paper.json", string(text))
			err = cli.Validate(context.Background(), spec)
			if rig != "figur" {
				if err != nil {
					t.Errorf("%s: %v", rig, err)
				}
				return
			}
			if ab, ok := AlsAbgelehnt(err); err != nil && (!ok || ab.Fehler[0].Regel != "proportion" || len(ab.Fehler) != 1) {
				t.Errorf("figur: %v", err)
			}
		})
	}
	if _, err := Vorlage("drache", "x", "y"); err == nil || !strings.Contains(err.Error(), "figur, vierbeiner, gebaeude, requisite, hintergrund") {
		t.Errorf("unbekannter Rig: %v", err)
	}
	if teile := Rigs["figur"].Pflicht; len(teile) != 10 || teile[0] != "rumpf" {
		t.Errorf("Pflicht figur: %v", teile)
	}
}

// Review MC1.5: die Vorlagen bestehen auch das echte validate (RD1, inkl. Beinpfad des Vierbeiners).
func TestRigVorlagenEchteCLI(t *testing.T) {
	t.Parallel()
	_, cli := testRepo(t)
	cli.Skript = filepath.Join(repoWurzel(t), "tools", "paper", "bin", "paper.ts")
	for _, rig := range RigNamen() {
		text, err := Vorlage(rig, "probe", "Probe")
		if err != nil {
			t.Fatal(err)
		}
		dir := filepath.Join(t.TempDir(), "gruppe")
		schreibe(t, dir, "probe.paper.json", string(text))
		err = cli.Validate(context.Background(), filepath.Join(dir, "probe.paper.json"))
		ab, _ := AlsAbgelehnt(err)
		if rig == "figur" && ab != nil && len(ab.Fehler) == 1 && ab.Fehler[0].Regel == "proportion" {
			continue
		}
		if err != nil {
			t.Errorf("%s: %v", rig, err)
		}
	}
}

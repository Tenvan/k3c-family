package paperassets

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
)

const monarchPfad = "public/grafik/figuren/monarch.paper.json"

func TestSetAendertMaterial(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	r, err := Set(context.Background(), cli, "monarch", []byte(`{"umhang":{"material":"purpur"}}`))
	if err != nil {
		t.Fatal(err)
	}
	if len(r.Felder) != 1 || r.Felder[0].Pfad != "umhang.material" || r.Felder[0].Alt != "koenigsrot" || r.Felder[0].Neu != "purpur" {
		t.Errorf("Felder: %+v", r.Felder)
	}
	if r.PNGs == 0 || strings.Join(r.Animationen, ",") != "idle,lauf,reiten" || len(r.Pixel) != 3 {
		t.Errorf("Render: %+v", r)
	}
	// B-706: Overlays von assets_set heißen anders als die von assets_review (<id>-<anim>-0-diff.png).
	for _, p := range r.Pixel {
		if p.Overlay != "" && p.Overlay != ".cache/paper/mcp/set-monarch-"+p.Animation+"-0-diff.png" {
			t.Errorf("Overlay %s: %s", p.Animation, p.Overlay)
		}
	}
	b, _ := os.ReadFile(filepath.Join(root, monarchPfad))
	if !strings.Contains(string(b), `"material": "purpur"`) || !strings.HasSuffix(string(b), "}\n") {
		t.Errorf("Spec nicht geschrieben:\n%.300s", b)
	}
	leer(t, filepath.Join(root, ".cache", "paper", "tmp"))
}

// 005/AC-06 · MC1/AC-02: ein ungültiger Patch ändert weder Spec noch Ausgabe.
func TestSetUngueltigAendertNichts(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	vorher := hashBaum(t, root, monarchPfad, "public/grafik/figuren/monarch")
	_, err := Set(context.Background(), cli, "monarch", []byte(`{"umhang":{"material":"scharlach"}}`))
	ab, ok := AlsAbgelehnt(err)
	if !ok || len(ab.Fehler) != 1 || ab.Fehler[0].Regel != "material-unbekannt" || ab.Fehler[0].Feld != "umhang.material" {
		t.Fatalf("Ablehnung: %v", err)
	}
	if nachher := hashBaum(t, root, monarchPfad, "public/grafik/figuren/monarch"); nachher != vorher {
		t.Error("Spec oder Ausgabe geändert")
	}
	leer(t, filepath.Join(root, ".cache", "paper", "tmp"))
	if _, err := Set(context.Background(), cli, "monarch", []byte(`{"umhang":`)); err == nil {
		t.Error("kaputter Patch ohne Fehler")
	}
	if _, err := Set(context.Background(), cli, "x", []byte(`{}`)); err == nil || err.Error() != "Asset „x“ fehlt" {
		t.Errorf("unbekannte id: %v", err)
	}
}

// Ein Render-Fehler (Exit 2) nach gültigem validate lässt ebenfalls alles stehen.
func TestSetRenderFehlerAendertNichts(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	stub := filepath.ToSlash(cli.Skript)
	cli.Skript = filepath.Join(t.TempDir(), "kaputt.ts")
	schreibe(t, filepath.Dir(cli.Skript), "kaputt.ts", `import { run } from '`+stub+`';
const a = process.argv.slice(2);
if (a[0] === 'render') { process.stderr.write('Renderer kaputt\n'); process.exit(2); }
const r = run(a);
if (r.stdout) process.stdout.write(r.stdout + '\n');
if (r.stderr) process.stderr.write(r.stderr + '\n');
process.exit(r.code);
`)
	vorher := hashBaum(t, root, monarchPfad, "public/grafik/figuren/monarch")
	_, err := Set(context.Background(), cli, "monarch", []byte(`{"umhang":{"material":"purpur"}}`))
	if err == nil || !strings.Contains(err.Error(), "Renderer kaputt") {
		t.Fatalf("Render-Fehler: %v", err)
	}
	if hashBaum(t, root, monarchPfad, "public/grafik/figuren/monarch") != vorher {
		t.Error("Spec oder Ausgabe geändert")
	}
	leer(t, filepath.Join(root, ".cache", "paper", "tmp"))
}

// Schreibt die CLI eine Ausgabe in den Temp-Ordner, ersetzt sie den alten Ausgabe-Ordner.
func TestSetUebernimmtAusgabe(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	stub := filepath.ToSlash(cli.Skript)
	cli.Skript = filepath.Join(t.TempDir(), "schreibt.ts")
	schreibe(t, filepath.Dir(cli.Skript), "schreibt.ts", `import { run } from '`+stub+`';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join, basename } from 'node:path';
const a = process.argv.slice(2);
if (a[0] === 'render') {
  const aus = join(dirname(a[1]), basename(a[1], '.paper.json'));
  mkdirSync(aus, { recursive: true });
  writeFileSync(join(aus, 'idle.png'), 'neu');
  process.stdout.write(JSON.stringify({ ok: true, manifeste: [], dateien: [join(aus, 'idle.png')] }) + '\n');
  process.exit(0);
}
const r = run(a);
if (r.stdout) process.stdout.write(r.stdout + '\n');
process.exit(r.code);
`)
	r, err := Set(context.Background(), cli, "monarch", []byte(`{"version":2}`))
	if err != nil {
		t.Fatal(err)
	}
	b, err := os.ReadFile(filepath.Join(root, "public/grafik/figuren/monarch/idle.png"))
	if err != nil || string(b) != "neu" || r.PNGs != 1 || r.Dateien[0] != "public/grafik/figuren/monarch/idle.png" {
		t.Errorf("Ausgabe: %q %v %+v", b, err, r)
	}
	leer(t, filepath.Join(root, ".cache", "paper", "tmp"))
}

// Review MC1.1: scheitert das Umbenennen der Ausgabe, kommt die alte Spec zurück.
func TestSetRollbackBeiAusgabeFehler(t *testing.T) {
	root, cli := testRepo(t)
	stub := filepath.ToSlash(cli.Skript)
	cli.Skript = filepath.Join(t.TempDir(), "schreibt.ts")
	schreibe(t, filepath.Dir(cli.Skript), "schreibt.ts", `import { run } from '`+stub+`';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join, basename } from 'node:path';
const a = process.argv.slice(2);
if (a[0] === 'render') {
  const aus = join(dirname(a[1]), basename(a[1], '.paper.json'));
  mkdirSync(aus, { recursive: true });
  writeFileSync(join(aus, 'idle.png'), 'neu');
  process.stdout.write(JSON.stringify({ ok: true, manifeste: [], dateien: [] }) + '\n');
  process.exit(0);
}
const r = run(a);
if (r.stdout) process.stdout.write(r.stdout + '\n');
process.exit(r.code);
`)
	vorher := hashBaum(t, root, monarchPfad, "public/grafik/figuren/monarch")
	ordnerUmbenennen = func(string, string) error { return os.ErrPermission }
	t.Cleanup(func() { ordnerUmbenennen = os.Rename })
	if _, err := Set(context.Background(), cli, "monarch", []byte(`{"version":2}`)); err == nil {
		t.Fatal("kein Fehler")
	}
	if hashBaum(t, root, monarchPfad, "public/grafik/figuren/monarch") != vorher {
		t.Error("Spec oder Ausgabe geändert")
	}
}

// B-706: die alte Ausgabe wird unter dem Temp-Ordner (.cache/) beiseitegelegt, nicht neben der Ausgabe.
func TestErsetzeOrdnerBeiseiteImCache(t *testing.T) {
	root := t.TempDir()
	tmp := filepath.Join(root, ".cache", "paper", "tmp", "abc")
	schreibe(t, tmp, "figuren/monarch/idle.png", "neu")
	schreibe(t, root, "public/grafik/figuren/monarch/idle.png", "alt")
	var ziele []string
	ordnerUmbenennen = func(a, b string) error { ziele = append(ziele, b); return os.Rename(a, b) }
	t.Cleanup(func() { ordnerUmbenennen = os.Rename })
	ziel := filepath.Join(root, "public", "grafik", "figuren", "monarch")
	if err := ersetzeOrdner(filepath.Join(tmp, "figuren", "monarch"), ziel); err != nil {
		t.Fatal(err)
	}
	if len(ziele) != 2 || ziele[0] != filepath.Join(tmp, ".beiseite") {
		t.Errorf("beiseite: %v", ziele)
	}
	if b, _ := os.ReadFile(filepath.Join(ziel, "idle.png")); string(b) != "neu" {
		t.Errorf("Ausgabe: %q", b)
	}
	if e, _ := os.ReadDir(filepath.Dir(ziel)); len(e) != 1 {
		t.Errorf("neben der Ausgabe: %v", e)
	}
}

// Review MC1.1: zwei gleichzeitige Sets auf dasselbe Asset verlieren kein Update.
func TestSetParallelOhneVerlust(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	var wg sync.WaitGroup
	for _, p := range []string{`{"version":2}`, `{"beschreibung":"neu"}`} {
		wg.Go(func() {
			if _, err := Set(context.Background(), cli, "monarch", []byte(p)); err != nil {
				t.Error(err)
			}
		})
	}
	wg.Wait()
	b, _ := os.ReadFile(filepath.Join(root, monarchPfad))
	if !strings.Contains(string(b), `"version": 2`) || !strings.Contains(string(b), `"beschreibung": "neu"`) {
		t.Errorf("Update verloren:\n%.300s", b)
	}
}

// Review MC1.1: Schreibe lehnt id oder Gruppe mit Pfadanteilen ab.
func TestSchreibePrueftNamen(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	for _, a := range []Asset{{ID: "../x", Gruppe: "g"}, {ID: "x", Gruppe: ".."}, {ID: "X", Gruppe: "g"}} {
		if _, err := Schreibe(context.Background(), cli, a, nil, []byte(`{}`)); err == nil {
			t.Errorf("%+v ohne Fehler", a)
		}
	}
	leer(t, filepath.Join(root, ".cache", "paper", "tmp"))
}

// MC1.5: gegen die echte Render-CLI (RD1) findet monarch sein Reittier in der Nachbar-Gruppe auch im Temp-Ordner.
func TestSetMitEchterCLI(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	cli.Skript = filepath.Join(repoWurzel(t), "tools", "paper", "bin", "paper.ts")
	r, err := Set(context.Background(), cli, "monarch", []byte(`{"umhang":{"material":"purpur"}}`))
	if err != nil {
		t.Fatal(err)
	}
	if len(r.Felder) != 1 || r.PNGs == 0 {
		t.Errorf("Ergebnis: %+v", r)
	}
	if _, err := os.Stat(filepath.Join(root, "public/grafik/figuren/monarch/manifest.json")); err != nil {
		t.Errorf("Ausgabe nicht übernommen: %v", err)
	}
	leer(t, filepath.Join(root, ".cache", "paper", "tmp"))
}

// Review MC1.5: in der Gruppe „alt“ geht die Änderung nicht verloren (Temp-Pfad der alten Spec kollidierte).
func TestSetGruppeAlt(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	kopiere(t, root, "tools/paper/fixtures/specs/mauer.paper.json", "public/grafik/alt/mauer.paper.json")
	_ = os.Remove(filepath.Join(root, "public/grafik/gebaeude/mauer.paper.json"))
	r, err := Set(context.Background(), cli, "mauer", []byte(`{"version":2}`))
	if err != nil || len(r.Felder) != 1 {
		t.Fatalf("Set: %+v %v", r.Felder, err)
	}
	b, _ := os.ReadFile(filepath.Join(root, "public/grafik/alt/mauer.paper.json"))
	if !strings.Contains(string(b), `"version": 2`) {
		t.Error("Änderung verloren")
	}
}

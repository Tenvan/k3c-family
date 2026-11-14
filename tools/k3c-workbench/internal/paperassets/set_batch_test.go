package paperassets

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// stubMit ersetzt die CLI durch ein Skript über der Attrappe: `render` liefert, was render (JS-Rumpf mit a =
// Argumente, Ergebnis {code, stdout, stderr} oder undefined = Attrappe) zurückgibt, auch als Schritt von `batch`.
// Jeder bun-Start schreibt eine Zeile in die gelieferte Zähl-Datei (B-856).
func stubMit(t *testing.T, cli CLI, render string) (CLI, string) {
	t.Helper()
	dir := t.TempDir()
	zaehler := filepath.Join(dir, "starts.txt")
	q := func(p string) string { b, _ := json.Marshal(filepath.ToSlash(p)); return string(b) }
	batchTS := filepath.Join(repoWurzel(t), "tools", "paper", "cli", "batch.ts")
	schreibe(t, dir, "stub.ts", `import { run } from `+q(cli.Skript)+`;
import { batch, schritteAus } from `+q(batchTS)+`;
import { appendFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { basename, dirname, join } from 'node:path';
appendFileSync(`+q(zaehler)+`, process.argv[2] + '\n');
type E = { code: 0 | 1 | 2; stdout: string; stderr: string };
const render = (a: string[]): E | undefined => { `+render+` };
const lauf = (a: string[]): E => (a[0] === 'render' ? render(a) : undefined) ?? run(a);
const a = process.argv.slice(2);
const r: E = a[0] === 'batch' ? { code: 0, stdout: JSON.stringify({ ok: true, schritte: batch(schritteAus(a[1]), lauf) }), stderr: '' } : lauf(a);
if (r.stdout) process.stdout.write(r.stdout + '\n');
if (r.stderr) process.stderr.write(r.stderr + '\n');
process.exit(r.code);
`)
	cli.Skript = filepath.Join(dir, "stub.ts")
	return cli, zaehler
}

// starts liest die Befehle der bun-Starts aus der Zähl-Datei.
func starts(t *testing.T, zaehler string) []string {
	t.Helper()
	b, err := os.ReadFile(zaehler)
	if err != nil {
		return nil
	}
	return strings.Fields(string(b))
}

// B-856/AC-01: assets_set startet bun höchstens zweimal (gemessen: einmal, `batch`), Ergebnis wie vorher.
func TestSetStartetBunHoechstensZweimal(t *testing.T) {
	t.Parallel()
	_, cli := testRepo(t)
	cli, zaehler := stubMit(t, cli, "return undefined;")
	r, err := Set(context.Background(), cli, "monarch", []byte(`{"umhang":{"material":"purpur"}}`))
	if err != nil {
		t.Fatal(err)
	}
	if s := starts(t, zaehler); len(s) > 2 || len(s) == 0 {
		t.Fatalf("bun-Starts: %d %v (erlaubt: höchstens 2)", len(s), s)
	} else {
		t.Logf("bun-Starts je assets_set: %d %v", len(s), s)
	}
	if len(r.Felder) != 1 || len(r.Pixel) != 3 || r.PNGs == 0 {
		t.Errorf("Ergebnis: %+v", r)
	}
	for _, p := range r.Pixel {
		if p.Fehler != "" || p.Overlay != ".cache/paper/mcp/set/monarch-"+p.Animation+"-0-diff.png" {
			t.Errorf("Pixel %s: %+v", p.Animation, p)
		}
	}
}

// Eine neue Animation hat keinen alten Frame: Anteil 1, Fehler „neu“, die anderen Animationen bleiben verglichen.
func TestSetNeueAnimationImSammelaufruf(t *testing.T) {
	t.Parallel()
	_, cli := testRepo(t)
	r, err := Set(context.Background(), cli, "monarch", []byte(`{"animationen":{"winken":{"frames":1}}}`))
	if err != nil {
		t.Fatal(err)
	}
	var neu *Pixel
	for i := range r.Pixel {
		if r.Pixel[i].Animation == "winken" {
			neu = &r.Pixel[i]
		} else if r.Pixel[i].Fehler != "" {
			t.Errorf("Pixel %s: %+v", r.Pixel[i].Animation, r.Pixel[i])
		}
	}
	if neu == nil || neu.Anteil != 1 || neu.Fehler != "neu" {
		t.Errorf("neue Animation: %+v", r.Pixel)
	}
}

// B-870/AC-01: Bricht der Sammelaufruf als Ganzes ab (hier Exit 2), nennt Set den Grund und die Felder, die ein
// einzelner diff noch liefert; die Spec bleibt unberührt.
func TestSetAbbruchDesSammelaufrufsNenntFelderUndGrund(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	dir := t.TempDir()
	q := func(p string) string { b, _ := json.Marshal(filepath.ToSlash(p)); return string(b) }
	schreibe(t, dir, "stub.ts", `import { run } from `+q(cli.Skript)+`;
const a = process.argv.slice(2);
if (a[0] === 'batch') { process.stderr.write('bun abgestürzt\n'); process.exit(2); }
const r = run(a);
if (r.stdout) process.stdout.write(r.stdout + '\n');
if (r.stderr) process.stderr.write(r.stderr + '\n');
process.exit(r.code);
`)
	cli.Skript = filepath.Join(dir, "stub.ts")
	spec := filepath.Join(root, "public", "grafik", "figuren", "monarch.paper.json")
	vorher, err := os.ReadFile(spec)
	if err != nil {
		t.Fatal(err)
	}
	r, err := Set(context.Background(), cli, "monarch", []byte(`{"umhang":{"material":"purpur"}}`))
	if err == nil || !strings.Contains(err.Error(), "bun abgestürzt") {
		t.Fatalf("Fehler: %v", err)
	}
	if !strings.Contains(r.Abbruch, "bun abgestürzt") || len(r.Felder) != 1 || r.Felder[0].Pfad != "umhang.material" {
		t.Errorf("Teilergebnis: Abbruch %q, Felder %+v", r.Abbruch, r.Felder)
	}
	if nachher, _ := os.ReadFile(spec); string(nachher) != string(vorher) {
		t.Error("Spec geändert")
	}
}

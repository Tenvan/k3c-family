package main

import (
	"reflect"
	"strings"
	"testing"
)

// testRepo: Module und Paket-Ordner wie im echten Checkout, ohne Dateisystem.
func testRepo() repo {
	goDirs := map[string]bool{
		"data": true, "engine/sim": true, "engine/balance": true, "engine/net": true, "tools/checkdiff": true,
		"tools/k3c-workbench": true, "tools/k3c-workbench/internal/planning": true, "tools/k3c-workbench/internal/simrun": true,
		"tools/assetkit": true, "native/k3c-rig": true, "native/k3c-rig/rig": true, "native/k3c-game/view": true, "engine/balance/report": false,
	}
	return repo{
		modules: []string{".", "native/k3c-game", "native/k3c-rig", "tools/assetkit", "tools/k3c-workbench"},
		hasGo:   func(dir string) bool { return goDirs[dir] },
	}
}

// want: erwartete Auswahl. flags = gesetzte Schalter (D Docs, R Rules, C Client, W WbVitest, S StarterVitest, A go.mod der
// Wurzel), pkgs = geänderte Paket-Ordner als "modul:ordner".
type want struct {
	flags string
	pkgs  []string
}

func summary(p plan) want {
	w := want{}
	for _, f := range []struct {
		on   bool
		name string
	}{{p.Docs, "D"}, {p.Rules, "R"}, {p.Client, "C"}, {p.WbVitest, "W"}, {p.StarterVitest, "S"}, {p.Mods["."] != nil && p.Mods["."].All, "A"}} {
		if f.on {
			w.flags += f.name
		}
	}
	for _, mod := range sortedKeys(p.Mods) {
		for _, d := range sortedKeys(p.Mods[mod].Dirs) {
			w.pkgs = append(w.pkgs, mod+":"+d)
		}
	}
	return w
}

func TestClassify(t *testing.T) {
	cases := []struct {
		name  string
		files []string
		want  want
	}{
		{"nur Doku: nur Planungs-Tests", []string{"docs/backlog/B-001-x.md", "CLAUDE.md"}, want{"D", nil}},
		{"Go-Datei: nur ihr Paket", []string{"engine/sim/world.go"}, want{"R", []string{".:engine/sim"}}},
		{"testdata im Paket zählt zum Paket", []string{"engine/net/testdata/x.json"}, want{"R", []string{".:engine/net"}}},
		{"Golden an der Wurzel: Besitzer-Pakete", []string{"testdata/golden/sim-1.json.gz"}, want{"R", []string{".:engine/level", ".:engine/sim"}}},
		{"data: Paket k3c/data und Client", []string{"data/enemies.json"}, want{"RC", []string{".:data"}}},
		{"Taskfile und CI lösen keine Tests aus", []string{"Taskfile.yml", "engine/Taskfile.yml", ".github/workflows/ci.yml"}, want{"R", nil}},
		{"go.mod: Modul übersetzen, keine Tests", []string{"go.mod"}, want{"RA", nil}},
		{"Workbench: eigenes Modul, Frontend nur Vitest",
			[]string{"tools/k3c-workbench/internal/planning/next.go", "tools/k3c-workbench/frontend/src/App.tsx"},
			want{"RW", []string{"tools/k3c-workbench:internal/planning"}}},
		{"webkit: Vitest beider Oberflächen", []string{"tools/webkit/atoms/Button.tsx"}, want{"RWS", nil}},
		{"natives Modul: Paket und Modul-Wurzel",
			[]string{"native/k3c-rig/rig/matrix.go", "native/k3c-rig/main.go", "native/k3c-rig/export/export.ts"},
			want{"R", []string{"native/k3c-rig:.", "native/k3c-rig:rig"}}},
		{"eingebettete Datei im Unterordner: Paket darüber", []string{"data/biomes/wald.json", "engine/balance/report/chart.json"}, want{"RC", []string{".:data", ".:engine/balance"}}},
		{"Lockfile zählt als Client", []string{"bun.lock"}, want{"RC", nil}},
		{"Client: src und Seiten", []string{"src/scenes/GameScene.ts", "index.html"}, want{"RC", nil}},
		{"gelöschtes Paket: kein Ziel", []string{"engine/alt/weg.go"}, want{"R", nil}},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := summary(classify(testRepo(), c.files)); !reflect.DeepEqual(got, c.want) {
				t.Errorf("Auswahl %+v, erwartet %+v", got, c.want)
			}
		})
	}
}

func names(steps []step) string {
	var n []string
	for _, s := range steps {
		n = append(n, s.Name)
	}
	return strings.Join(n, " ")
}

func TestGoStepsNurGeaendertePaketeTesten(t *testing.T) {
	p := classify(testRepo(), []string{"engine/sim/world.go", "engine/balance/run.go"})
	abnehmer := []string{"k3c/engine/room"}
	got := goSteps([]goTargets{targetsOf(".", p.Mods["."], abnehmer, false)})
	if names(got) != "go-vet go-lint go-test" {
		t.Fatalf("Schritte %q", names(got))
	}
	vet, test := strings.Join(got[0].Cmd, " "), strings.Join(got[2].Cmd, " ")
	if vet != "go vet ./engine/balance ./engine/sim k3c/engine/room" {
		t.Errorf("vet prüft geänderte Pakete und Abnehmer: %q", vet)
	}
	if test != "go test -short ./engine/sim" {
		t.Errorf("Tests nur im geänderten Paket, Sim-Pakete gesperrt: %q", test)
	}
	if got[0].Stage != 1 || got[2].Stage != 2 {
		t.Errorf("vet in Stufe 1, Tests in Stufe 2: %+v", got)
	}
}

func TestSimSperreAufgehoben(t *testing.T) {
	p := classify(testRepo(), []string{"engine/balance/run.go"})
	if tg := targetsOf(".", p.Mods["."], nil, false); len(tg.Tests) != 0 {
		t.Errorf("gesperrt, aber Tests %v", tg.Tests)
	}
	if tg := targetsOf(".", p.Mods["."], nil, true); !reflect.DeepEqual(tg.Tests, []string{"./engine/balance"}) {
		t.Errorf("CHECK_SIM=1: Tests %v", tg.Tests)
	}
	if got := skippedSim(p, false); !reflect.DeepEqual(got, []string{"engine/balance"}) {
		t.Errorf("gemeldet %v", got)
	}
}

func TestAbnehmerInAnderemModulNurVet(t *testing.T) {
	got := goSteps([]goTargets{targetsOf("tools/k3c-workbench", nil, []string{"k3c/tools/k3c-workbench/internal/simrun"}, false)})
	if names(got) != "k3c-workbench-frontend k3c-workbench-vet" {
		t.Errorf("Schritte %q", names(got))
	}
}

func TestJsSteps(t *testing.T) {
	got := jsSteps(plan{Docs: true}, "HEAD")
	if names(got) != "regeln" || strings.Contains(strings.Join(got[0].Cmd, " "), "projectRules") {
		t.Errorf("nur Doku: %+v", got)
	}
	got = jsSteps(plan{Rules: true, Client: true}, "HEAD")
	if names(got) != "regeln lint typecheck vitest" || got[3].Stage != 2 {
		t.Errorf("Client: %q", names(got))
	}
}

// Spiel-Modul (B-794, AC-01): Änderung im Paket löst vet, Lint und Test im Modul aus (Lint seit dem NG1-Review, das
// Modul steht in lintMods). Eine Änderung in engine/ übersetzt das Modul als Abnehmer (nur vet).
func TestSpielModul(t *testing.T) {
	p := classify(testRepo(), []string{"native/k3c-game/view/frame.go"})
	if w := summary(p); !reflect.DeepEqual(w, want{"R", []string{"native/k3c-game:view"}}) {
		t.Fatalf("Auswahl %+v", w)
	}
	got := goSteps([]goTargets{targetsOf("native/k3c-game", p.Mods["native/k3c-game"], nil, false)})
	if names(got) != "k3c-game-vet k3c-game-lint k3c-game-test" || got[0].Dir != "native/k3c-game" {
		t.Fatalf("Schritte %q in %q", names(got), got[0].Dir)
	}
	if strings.Join(got[1].Cmd, " ") != "golangci-lint run ./view" {
		t.Errorf("Lint: %v", got[1].Cmd)
	}
	if strings.Join(got[2].Cmd, " ") != "go test -short ./view" {
		t.Errorf("Test: %v", got[2].Cmd)
	}
	got = goSteps([]goTargets{targetsOf("native/k3c-game", nil, []string{"k3c/native/k3c-game/world"}, false)})
	if names(got) != "k3c-game-vet" {
		t.Errorf("Abnehmer aus engine/: %q", names(got))
	}
}

package balance

import (
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strconv"
	"strings"
	"testing"
)

// RS2.2 (B-726), seit RU41 gegen Rust: Die Namen des Studios folgen dem Crate k3c-balance, das `k3c-game sim balance`
// rechnet. Der Test liest den Rust-Quelltext (bot_names, min_players, Konstanten der Hilfen) und vergleicht.

const rustBalance = "../../../../rust/crates/k3c-balance/src"

func readRust(t *testing.T, file string) string {
	t.Helper()
	b, err := os.ReadFile(filepath.Join(rustBalance, file))
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

// rustBody liefert den Rumpf der Funktion name (bis zur ersten Zeile, die nur `}` enthält).
func rustBody(t *testing.T, src, name string) string {
	t.Helper()
	_, rest, ok := strings.Cut(src, "fn "+name+"(")
	if !ok {
		t.Fatalf("fn %s nicht gefunden", name)
	}
	body, _, _ := strings.Cut(rest, "\n}")
	return body
}

var (
	rustString = regexp.MustCompile(`"([^"]+)"`)
	rustArm    = regexp.MustCompile(`"([^"]+)"\s*=>\s*(\d+)`)
	rustConst  = regexp.MustCompile(`pub const (\w+): &str = "([^"]*)";`)
)

func TestBotNamenWieEngine(t *testing.T) {
	src := readRust(t, "bots.rs")
	var names []string
	for _, m := range rustString.FindAllStringSubmatch(rustBody(t, src, "bot_names"), -1) {
		names = append(names, m[1])
	}
	slices.Sort(names)
	if !slices.Equal(names, BotNames()) {
		t.Errorf("Profile k3c-balance %v, Studio %v", names, BotNames())
	}
	arms := rustArm.FindAllStringSubmatch(rustBody(t, src, "min_players"), -1)
	for _, m := range arms {
		if strconv.Itoa(minPlayers[m[1]]) != m[2] {
			t.Errorf("minPlayers[%s]: k3c-balance %s, Studio %d", m[1], m[2], minPlayers[m[1]])
		}
	}
	if len(arms) != len(minPlayers) {
		t.Errorf("minPlayers: andere Anzahl als k3c-balance (%d)", len(arms))
	}
}

func TestHilfenWieEngine(t *testing.T) {
	want := map[string]string{"HELPS_AIDS": HelpsAids, "HELPS_FULL": HelpsFull, "HELPS_BOSS": HelpsBoss, "MIX_SEP": MixSep}
	got := map[string]string{}
	for _, file := range []string{"helps.rs", "bots.rs"} {
		for _, m := range rustConst.FindAllStringSubmatch(readRust(t, file), -1) {
			got[m[1]] = m[2]
		}
	}
	for k, v := range want {
		if got[k] != v {
			t.Errorf("%s: k3c-balance %q, Studio %q", k, got[k], v)
		}
	}
}

func TestKnownBotUndWelten(t *testing.T) {
	for name, ok := range map[string]bool{"saver": true, "coop2": true, "saver+passive": true, "coop2+saver": false, "": false, "raten": false} {
		if KnownBot(name) != ok {
			t.Errorf("KnownBot(%q) = %v", name, !ok)
		}
	}
	if w, err := loadWorld("standard"); err != nil || len(w.Islands) == 0 || len(w.Islands[0].Depths) == 0 {
		t.Errorf("standard: %+v %v", w, err)
	}
	if _, err := loadWorld("../targets"); err == nil {
		t.Error("Pfad außerhalb von data/worlds angenommen")
	}
}

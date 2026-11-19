package main

import (
	"strconv"
	"strings"
	"unicode"
)

func itoa(n int) string { return strconv.Itoa(n) }

// snake macht aus einem Go-Testnamen den Rust-Namen: `TestFooBar` → `foo_bar`, `TestRU15Foo` → `ru15_foo`,
// `TestHUDLayout` → `hud_layout`, `TestFoo_bar` → `foo_bar`. Das Präfix Test entfällt, Großbuchstabenfolgen bleiben zusammen.
func snake(goName string) string {
	s := strings.TrimPrefix(goName, "Test")
	r := []rune(s)
	var b strings.Builder
	for i, c := range r {
		if c == '_' {
			if b.Len() > 0 && !strings.HasSuffix(b.String(), "_") {
				b.WriteRune('_')
			}
			continue
		}
		if unicode.IsUpper(c) && i > 0 && r[i-1] != '_' {
			prev := r[i-1]
			nextLower := i+1 < len(r) && unicode.IsLower(r[i+1])
			if unicode.IsLower(prev) || unicode.IsDigit(prev) || (unicode.IsUpper(prev) && nextLower) {
				b.WriteRune('_')
			}
		}
		b.WriteRune(unicode.ToLower(c))
	}
	return b.String()
}

// themes ordnet einen fehlenden Test nach Dateiname einem Sprint-Thema zu (RU60.2 bis RU60.4); sonst "allgemein".
var themes = []struct{ key, theme string }{
	{"story", "Geschichte (GS1)"},
	{"crown", "Krone (KS3, KS6)"},
	{"portal", "Portale (KS2)"},
	{"torch", "Fackeln (W15)"},
	{"voyage", "Schiffsfahrt (GS2)"},
	{"seemap", "Seekarte (GS2, WE4)"},
	{"seekarte", "Seekarte (GS2, WE4)"},
	{"harbor", "Seekarte (GS2, WE4)"},
	{"island_travel", "Seekarte (GS2, WE4)"},
	{"ship", "Seekarte (GS2, WE4)"},
	{"handwerker", "Handwerker (WI1)"},
	{"hammer", "Handwerker (WI1)"},
	{"purse", "Börse (KS5)"},
	{"shield", "Schild und Banner (KS4)"},
	{"banner", "Schild und Banner (KS4)"},
	{"endboss", "Endboss (WE5)"},
	{"events", "Events (WE5)"},
	{"skins", "Skins (WE5)"},
	{"hub", "Hub (WE5)"},
}

func theme(file string) string {
	l := strings.ToLower(file[strings.LastIndex(file, "/")+1:])
	for _, t := range themes {
		if strings.Contains(l, t.key) {
			return t.theme
		}
	}
	return "allgemein"
}

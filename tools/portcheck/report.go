package main

import (
	"fmt"
	"sort"
	"strings"
)

// areaResult ist das Ergebnis eines Bereichs.
type areaResult struct {
	area    area
	total   int
	matched int
	missing []goTest
}

// compare gleicht die Go-Tests eines Bereichs mit den Rust-Funktionsnamen ab. Subtests und Tabellenfälle zählen als der
// Elterntest; gleiche Namen in mehreren Go-Dateien zählen je Datei.
func compare(a area, tests []goTest, rust map[string]bool) areaResult {
	r := areaResult{area: a, total: len(tests)}
	for _, t := range tests {
		if rust[snake(t.name)] {
			r.matched++
		} else {
			r.missing = append(r.missing, t)
		}
	}
	return r
}

func render(tag string, rows []areaResult, noTarget []string) string {
	var b strings.Builder
	b.WriteString("# Test-Abgleich Go gegen Rust (portcheck, RU60.1)\n\n")
	fmt.Fprintf(&b, "Erzeugt mit `task gen:portcheck` aus dem Tag `%s` und dem Arbeitsbaum. Reproduzierbar, nur `git` und Dateien.\n\n", tag)
	b.WriteString("**Regel:** Go `TestFooBar` entspricht Rust `fn foo_bar` irgendwo im Zielordner; Subtests und Tabellenfälle zählen als der\n")
	b.WriteString("Elterntest. Ein Rust-Test mit anderem Namen wird nicht erkannt: „fehlt“ heißt „kein Name gleich“ und ist eine Arbeitsliste\n")
	b.WriteString("für RU60.2 bis RU60.4, kein Beweis, dass das Verhalten fehlt.\n\n")
	b.WriteString("| Bereich | Go-Ordner | Rust-Ordner | Go-Tests | mit Gegenstück | fehlen |\n|---|---|---|---|---|---|\n")
	tt, tm := 0, 0
	for _, r := range rows {
		fmt.Fprintf(&b, "| %s | `%s` | `%s` | %d | %d | %d |\n", r.area.name, r.area.goDir, r.area.rustDir, r.total, r.matched, len(r.missing))
		tt += r.total
		tm += r.matched
	}
	fmt.Fprintf(&b, "| **gesamt** | | | **%d** | **%d** | **%d** |\n\n", tt, tm, tt-tm)
	b.WriteString("## Fehlende Tests nach Thema\n\n")
	byTheme := map[string][]string{}
	for _, r := range rows {
		for _, m := range r.missing {
			t := theme(m.file)
			byTheme[t] = append(byTheme[t], fmt.Sprintf("%s/%s (`%s`)", r.area.name, m.name, m.file[strings.LastIndex(m.file, "/")+1:]))
		}
	}
	themes := make([]string, 0, len(byTheme))
	for t := range byTheme {
		themes = append(themes, t)
	}
	sort.Strings(themes)
	for _, t := range themes {
		fmt.Fprintf(&b, "- **%s:** %d\n", t, len(byTheme[t]))
	}
	b.WriteString("\n## Fehlende Tests je Bereich\n")
	for _, r := range rows {
		if len(r.missing) == 0 {
			continue
		}
		fmt.Fprintf(&b, "\n### %s (%d von %d fehlen)\n\n", r.area.name, len(r.missing), r.total)
		sorted := append([]goTest(nil), r.missing...)
		sort.Slice(sorted, func(i, j int) bool { return sorted[i].file+sorted[i].name < sorted[j].file+sorted[j].name })
		for _, m := range sorted {
			fmt.Fprintf(&b, "- `%s` (`%s`, %s)\n", m.name, m.file, theme(m.file))
		}
	}
	b.WriteString("\n## Ohne Ziel (kein Rust-Gegenstück, Entscheidung 013)\n\n")
	b.WriteString("Go-Ordner mit Tests, für die es keinen Rust-Bereich gibt (Werkzeuge, Workbench, Go-Engine-Teile): Ordner, Zahl der Go-Tests.\n\n")
	for _, n := range noTarget {
		p := strings.SplitN(n, "|", 2)
		fmt.Fprintf(&b, "- `%s`: %s\n", p[0], p[1])
	}
	return b.String()
}

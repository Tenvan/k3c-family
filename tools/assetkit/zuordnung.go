package assets

import (
	"os"
	"path/filepath"
	"strings"
)

// AssignmentRow ist eine Tabellenzeile aus docs/assets/zuordnung*.md, die zu einem Slot passt.
type AssignmentRow struct {
	File    string            `json:"datei"`
	Section string            `json:"abschnitt"`
	Cells   map[string]string `json:"zellen"`
}

// mdTable ist eine gelesene Markdown-Tabelle samt der Überschrift davor.
type mdTable struct {
	section string
	head    []string
	rows    [][]string
}

// readTables liest alle Tabellen einer Markdown-Datei tolerant: Kopfzeile = Spaltennamen, Trennzeile entfällt,
// Abschnitt = letzte ##-Überschrift.
func readTables(file string) []mdTable {
	b, err := os.ReadFile(file)
	if err != nil {
		return nil
	}
	var out []mdTable
	section := ""
	open := false
	for _, line := range strings.Split(strings.ReplaceAll(string(b), "\r\n", "\n"), "\n") {
		t := strings.TrimSpace(line)
		switch {
		case strings.HasPrefix(t, "#"):
			open = false
			if strings.HasPrefix(t, "##") {
				section = strings.TrimSpace(strings.TrimLeft(t, "#"))
			}
		case strings.HasPrefix(t, "|"):
			cells := splitRow(t)
			switch {
			case !open:
				out = append(out, mdTable{section: section, head: cells})
				open = true
			case isSeparator(cells):
			default:
				last := &out[len(out)-1]
				last.rows = append(last.rows, cells)
			}
		default:
			open = false
		}
	}
	return out
}

// splitRow zerlegt eine Tabellenzeile in getrimmte Zellen ohne die äußeren Pipes.
func splitRow(line string) []string {
	line = strings.TrimSuffix(strings.TrimPrefix(line, "|"), "|")
	cells := strings.Split(line, "|")
	for i := range cells {
		cells[i] = strings.TrimSpace(cells[i])
	}
	return cells
}

// isSeparator erkennt die Trennzeile |---|---|.
func isSeparator(cells []string) bool {
	for _, c := range cells {
		if strings.Trim(c, ":- ") != "" {
			return false
		}
	}
	return len(cells) > 0
}

// cleanCell nimmt Backticks und Fettdruck weg.
func cleanCell(s string) string {
	return strings.TrimSpace(strings.NewReplacer("`", "", "**", "").Replace(s))
}

// findAssignment liefert alle Zeilen, deren erste Zelle (ohne Backticks/Fettdruck) die Slot-ID ist.
func findAssignment(root, id string) []AssignmentRow {
	rows := []AssignmentRow{}
	files, _ := filepath.Glob(filepath.Join(root, "docs", "assets", "zuordnung*.md"))
	for _, f := range files {
		for _, t := range readTables(f) {
			for _, r := range t.rows {
				if len(r) == 0 || cleanCell(r[0]) != id {
					continue
				}
				rows = append(rows, AssignmentRow{File: filepath.Base(f), Section: t.section, Cells: rowCells(t.head, r)})
			}
		}
	}
	return rows
}

// rowCells ordnet die Zellen einer Zeile den Spaltenköpfen zu; fehlende Zellen bleiben weg.
func rowCells(head, r []string) map[string]string {
	cells := map[string]string{}
	for i, h := range head {
		if i < len(r) {
			cells[h] = r[i]
		}
	}
	return cells
}

// findPack sucht den Pack-Ordner der Datei in der Pack-Tabelle von zuordnung.md (Spalte „Pack (Ordner-ID)“, mehrere
// IDs kommagetrennt); ohne Treffer gibt es nur die ID.
func findPack(root, file string) *SlotPack {
	id := packFolder(file)
	if id == "" {
		return nil
	}
	p := &SlotPack{ID: id}
	for _, t := range readTables(filepath.Join(root, "docs", "assets", "zuordnung.md")) {
		col, src, lic := colIndex(t.head, "Pack"), colIndex(t.head, "Quelle"), colIndex(t.head, "Lizenz")
		if col < 0 || src < 0 || lic < 0 {
			continue
		}
		for _, r := range t.rows {
			if len(r) <= max(col, src, lic) || !hasID(r[col], id) {
				continue
			}
			p.Source, p.License = r[src], r[lic]
			return p
		}
	}
	return p
}

// colIndex ist die Spalte, deren Name mit prefix beginnt (-1 ohne Treffer).
func colIndex(head []string, prefix string) int {
	for i, h := range head {
		if strings.HasPrefix(h, prefix) {
			return i
		}
	}
	return -1
}

// hasID prüft, ob die kommagetrennte Zelle die Pack-ID enthält.
func hasID(cell, id string) bool {
	for _, p := range strings.Split(cell, ",") {
		if cleanCell(p) == id {
			return true
		}
	}
	return false
}

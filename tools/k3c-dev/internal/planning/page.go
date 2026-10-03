package planning

import (
	_ "embed"
	"encoding/json"
	"strings"
)

// pageHTML ist die Seite „Sprints & Backlog“: eigenständiges HTML mit Filter und Detail-Panel. Der Mock der Oberfläche
// liest dieselbe Datei (frontend/src/api/mockPlanning.ts) und setzt eigene Daten ein.
//
//go:embed page.html
var pageHTML string

// DataMarker steht in page.html an der Stelle der Daten.
const DataMarker = "/*DATA*/null"

// Page erzeugt die Seite aus der Planung unter root/docs. json.Marshal maskiert < und >, ein `</script>` im Text bricht
// das Script also nicht ab.
func Page(root string) (string, error) {
	d, err := Load(root)
	if err != nil {
		return "", err
	}
	b, err := json.Marshal(d)
	if err != nil {
		return "", err
	}
	return strings.Replace(pageHTML, DataMarker, string(b), 1), nil
}

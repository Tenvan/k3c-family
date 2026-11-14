package mcpsrv

import (
	"context"
	"fmt"
	"os/exec"
	"regexp"
	"strconv"
	"time"
)

// minTaskVersion ist die kleinste Go-Task-Version, mit der check_run Testmuster an Task-Ziele gibt: Task quotet
// CLI_ARGS nach `--` und setzt GO_TEST_FLAGS als Text ein (B-866), belegt mit 3.54 (B-868). Einzige Stelle der Zahl;
// requirements.md nennt dieselbe.
var minTaskVersion = [3]int{3, 54, 0}

// readTaskVersion liest `task --version` (z. B. „3.54.0“ oder „Task version: v3.54.0 (h1:…)“); Tests ersetzen es
// über Server.taskVersion.
func readTaskVersion(ctx context.Context) (string, error) {
	ctx, cancel := context.WithTimeout(ctx, 10*time.Second)
	defer cancel()
	out, err := exec.CommandContext(ctx, "task", "--version").Output()
	return string(out), err
}

var versionRule = regexp.MustCompile(`v?(\d+)\.(\d+)(?:\.(\d+))?`)

// parseVersion holt die erste Version x.y[.z] aus dem Text.
func parseVersion(s string) ([3]int, bool) {
	m := versionRule.FindStringSubmatch(s)
	if m == nil {
		return [3]int{}, false
	}
	var v [3]int
	for i := range 3 {
		if m[i+1] != "" {
			v[i], _ = strconv.Atoi(m[i+1])
		}
	}
	return v, true
}

func versionLess(a, b [3]int) bool {
	for i := range 3 {
		if a[i] != b[i] {
			return a[i] < b[i]
		}
	}
	return false
}

func versionText(v [3]int) string { return fmt.Sprintf("%d.%d.%d", v[0], v[1], v[2]) }

// checkTaskVersion prüft vor einem Task-Ziel mit Testmuster die Task-Version (B-868): zu alt → Fehler mit der
// Mindestversion; nicht lesbar → Warnung (Text für die Antwort), kein Abbruch. Andere Ziele prüft es nicht.
func checkTaskVersion(ctx context.Context, t checkTarget, pattern string, read func(context.Context) (string, error)) (string, error) {
	if pattern == "" || len(t.command) == 0 || t.command[0] != "task" {
		return "", nil
	}
	out, err := read(ctx)
	v, ok := parseVersion(out)
	if err != nil || !ok {
		return fmt.Sprintf("⚠ Task-Version nicht lesbar (task --version); Testmuster brauchen mindestens %s (requirements.md)",
			versionText(minTaskVersion)), nil
	}
	if versionLess(v, minTaskVersion) {
		return "", fmt.Errorf("%s mit Testmuster braucht Go Task ab %s (gefunden %s), sonst quotet Task das Muster nicht; "+
			"Task aktualisieren (requirements.md) oder ohne Muster laufen lassen", t.name, versionText(minTaskVersion), versionText(v))
	}
	return "", nil
}

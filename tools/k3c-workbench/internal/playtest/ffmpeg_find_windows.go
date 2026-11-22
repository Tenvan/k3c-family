//go:build windows

package playtest

import (
	"os"
	"strings"

	"golang.org/x/sys/windows/registry"
)

// registryPathDirs liest den aktuellen PATH des Benutzers und des Systems aus der Registry (Umgebungsvariablen
// in den Werten werden aufgelöst) – ein Prozess, der vor der Installation von ffmpeg startete, erbt ihn nicht.
func registryPathDirs() []string {
	var out []string
	sources := []struct {
		root registry.Key
		path string
	}{
		{registry.CURRENT_USER, `Environment`},
		{registry.LOCAL_MACHINE, `SYSTEM\CurrentControlSet\Control\Session Manager\Environment`},
	}
	for _, s := range sources {
		k, err := registry.OpenKey(s.root, s.path, registry.QUERY_VALUE)
		if err != nil {
			continue
		}
		v, _, err := k.GetStringValue("Path")
		_ = k.Close()
		if err != nil {
			continue
		}
		for _, d := range strings.Split(v, ";") { // erst trennen, dann auflösen: ein einzelnes % wandert nicht über ein ; hinweg
			if d = strings.TrimSpace(os.Expand(expandPercent(d), expandVar)); d != "" {
				out = append(out, d)
			}
		}
	}
	return out
}

// expandVar löst ${NAME} auf; `$$` (aus einem literalen $ im Pfad, siehe expandPercent) wird wieder zu einem $.
func expandVar(name string) string {
	if name == "$" {
		return "$"
	}
	if v, ok := os.LookupEnv(name); ok {
		return v
	}
	return ""
}

// expandPercent macht aus %NAME% ein ${NAME} und maskiert ein literales $ als $$.
func expandPercent(s string) string {
	s = strings.ReplaceAll(s, "$", "$$")
	var b strings.Builder
	for {
		i := strings.IndexByte(s, '%')
		if i < 0 {
			b.WriteString(s)
			return b.String()
		}
		j := strings.IndexByte(s[i+1:], '%')
		if j < 0 {
			b.WriteString(s)
			return b.String()
		}
		b.WriteString(s[:i])
		b.WriteString("${" + s[i+1:i+1+j] + "}")
		s = s[i+1+j+1:]
	}
}

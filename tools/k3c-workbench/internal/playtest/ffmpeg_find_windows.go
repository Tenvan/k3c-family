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
		for _, d := range strings.Split(os.ExpandEnv(expandPercent(v)), ";") {
			if d = strings.TrimSpace(d); d != "" {
				out = append(out, d)
			}
		}
	}
	return out
}

// expandPercent macht aus %NAME% ein ${NAME}, das os.ExpandEnv kennt.
func expandPercent(s string) string {
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

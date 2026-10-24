package planning

import (
	"fmt"
	"strings"
)

// B-519: Status und Dateien eines aktiven Sprints stehen bis zum Sprint-PR nur auf origin/sprint/<präfix>. plan_next
// überlagert sie aus dem lokalen Ref refs/remotes/origin/… (kein fetch, kein Netz); fehlt der Ref, gilt der lokale Stand.

// originOverlay überlagert die Sessions jedes aktiven Sprints in d mit dem Stand von origin/sprint/<präfix> und
// liefert je Sprint eine Hinweiszeile, ob origin oder der lokale Stand gilt. readmes: Sprint-ID → README von origin.
func originOverlay(root string, d *Data) (notes []string, readmes map[string]string) {
	readmes = map[string]string{}
	for i, sp := range d.Sprints {
		if sp.Status != "aktiv" {
			continue
		}
		prefix := sprintPrefix(sp)
		ref := "refs/remotes/origin/sprint/" + prefix
		if _, err := runGit(root, "rev-parse", "--verify", "--quiet", ref); err != nil {
			notes = append(notes, fmt.Sprintf("%s: kein origin/sprint/%s, lokaler Stand", sp.ID, prefix))
			continue
		}
		od, err := loadAt(root, ref)
		if err != nil {
			notes = append(notes, fmt.Sprintf("%s: origin/sprint/%s nicht lesbar (%v), lokaler Stand", sp.ID, prefix, err))
			continue
		}
		for _, o := range od.Sprints {
			if o.ID != sp.ID {
				continue
			}
			d.Sprints[i].Sessions = o.Sessions
			if txt, err := runGit(root, "show", ref+":docs/"+originReadmePath(root, ref, sp.ID)); err == nil {
				readmes[sp.ID] = string(txt)
			}
			notes = append(notes, fmt.Sprintf("%s: Stand von origin/sprint/%s", sp.ID, prefix))
			break
		}
	}
	return notes, readmes
}

// originReadmePath ist der Pfad der Sprint-README unter docs/ im Stand ref (der Sprint kann dort in einem anderen
// Zustandsordner liegen); leer, wenn nicht gefunden.
func originReadmePath(root, ref, id string) string {
	out, err := runGit(root, "ls-tree", "-r", "--name-only", ref, "docs/sprints")
	if err != nil {
		return ""
	}
	for _, l := range strings.Split(string(out), "\n") {
		l = strings.TrimSpace(l)
		parts := strings.Split(l, "/")
		if len(parts) == 5 && parts[4] == "README.md" && strings.HasPrefix(parts[3], id+"-") {
			return strings.TrimPrefix(l, "docs/")
		}
	}
	return ""
}

// sprintPrefix ist der Branch-Präfix: aus „Branch“ der ersten Session (`<präfix>/…`), sonst die Sprint-ID klein.
func sprintPrefix(sp Sprint) string {
	for _, x := range sp.Sessions {
		if b, _, ok := strings.Cut(sessionHead(x)["Branch"], "/"); ok && b != "" {
			return b
		}
	}
	return strings.ToLower(sp.ID)
}

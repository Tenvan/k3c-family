package mcpsrv

import (
	"fmt"
	"strings"

	"k3c/tools/k3c-workbench/internal/balance"
	"k3c/tools/k3c-workbench/internal/simrun"
)

// sim_test action baseline (B-640, BS7.2): ohne saveBaseline die gesicherten Baselines auflisten, mit id und
// saveBaseline einen durchgelaufenen Offline-Lauf unter einem Namen sichern. Ablage reports/baseline/<name>/.

func (s *Server) simBaseline(root string, in simTestIn) (string, error) {
	if in.SaveBaseline == "" {
		return listBaselines(root)
	}
	if in.ID == "" {
		return "", fmt.Errorf("sim_test baseline: id des durchgelaufenen Laufs fehlt")
	}
	info, err := s.sims.Get(root, in.ID)
	switch {
	case err != nil:
		return "", fmt.Errorf("lauf %q unbekannt in diesem Checkout; sim_test list zeigt die Läufe", in.ID)
	case info.State != simrun.Done:
		return "", fmt.Errorf("lauf %s ist %s; gesichert werden nur durchgelaufene Läufe", in.ID, info.State)
	}
	report, err := simrun.RunReport(root, in.ID)
	if err != nil {
		return "", err
	}
	// derselbe Weg wie „Als Baseline sichern“ der Balancing-Seite (B-662)
	bi, err := s.sims.SaveBaseline(root, report, in.SaveBaseline, in.OhnePass)
	if err != nil {
		return "", err
	}
	return fmt.Sprintf("💾 Baseline %s gesichert aus %s: %d Seeds (%s), Bots %s, Welt %s\nVergleichen: sim_test {action: start, baseline: %q}",
		bi.Name, bi.Run, bi.Seeds, bi.SeedMode, strings.Join(bi.Bots, ","), strings.Join(bi.Worlds, ","), bi.Name), nil
}

func listBaselines(root string) (string, error) {
	list, err := balance.ListBaselines(root)
	if err != nil {
		return "", err
	}
	if len(list) == 0 {
		return "keine gesicherten Baselines; sichern mit sim_test {action: baseline, id, saveBaseline: M1}", nil
	}
	var out []string
	for _, b := range list {
		out = append(out, fmt.Sprintf("%s · %s · %s · %d Seeds · Bots %s", b.Name, b.Saved, b.Run, b.Seeds, strings.Join(b.Bots, ",")))
	}
	return strings.Join(out[:min(len(out), maxStatusLines)], "\n"), nil
}

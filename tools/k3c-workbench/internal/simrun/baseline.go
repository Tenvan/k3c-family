package simrun

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"k3c/tools/k3c-workbench/internal/balance"
)

// Baseline sichern (B-640, B-662): sim_test {action: baseline} und „Als Baseline sichern“ der Balancing-Seite rufen
// beide SaveBaseline, ein Weg für dieselben Dateien unter reports/baseline/<name>/.

// SaveBaseline sichert den Bericht rel (reports/<lauf>/balance-*.json) unter name. Kennt das Register den Lauf
// (simtest-<id>), muss er durchgelaufen sein; der Auftrag kommt vom Lauf, sonst aus dem Bericht (Stamp). open sichert
// auch einen gültigen Lauf ohne Pass (offener Meilenstein, balance.SaveBaseline).
func (g *Registry) SaveBaseline(root, rel, name string, open bool) (balance.BaselineInfo, error) {
	path, err := ReportPath(root, rel)
	if err != nil {
		return balance.BaselineInfo{}, err
	}
	dir := filepath.Dir(path)
	if !strings.HasPrefix(filepath.Base(path), "balance-") || filepath.Dir(dir) != filepath.Join(root, "reports") {
		return balance.BaselineInfo{}, fmt.Errorf("bericht %q: nur balance-*.json im Ordner eines Laufs (reports/<lauf>/)", rel)
	}
	if st, err := os.Lstat(path); err != nil || !st.Mode().IsRegular() {
		return balance.BaselineInfo{}, fmt.Errorf("bericht %q gibt es nicht", rel)
	}
	var spec json.RawMessage
	if id, ok := strings.CutPrefix(filepath.Base(dir), "simtest-"); ok {
		if info, err := g.Get(root, id); err == nil {
			if info.State != Done {
				return balance.BaselineInfo{}, fmt.Errorf("lauf %s ist %s; gesichert werden nur durchgelaufene Läufe", id, info.State)
			}
			if info.Spec != nil {
				spec, _ = json.Marshal(info.Spec)
			}
		}
	}
	if spec == nil {
		spec = reportSpec(path)
	}
	bi, err := balance.SaveBaseline(root, name, path, spec, open)
	if err == nil {
		g.mu.Lock()
		g.reportsChanged(root)
		g.mu.Unlock()
	}
	return bi, err
}

// RunReport ist der neueste Balancing-Bericht des Laufs id (reports/simtest-<id>/balance-*.json, relativ zum Checkout).
func RunReport(root, id string) (string, error) {
	files, _ := filepath.Glob(filepath.Join(root, "reports", "simtest-"+id, "balance-*.json"))
	if len(files) == 0 {
		return "", fmt.Errorf("lauf %s hat keinen Balancing-Bericht (nur offline-Läufe mit focus balance)", id)
	}
	rel, err := filepath.Rel(root, files[len(files)-1])
	return filepath.ToSlash(rel), err
}

// reportSpec ist der gestempelte Auftrag eines Berichts als JSON; nil ohne Auftrag (ältere Berichte).
func reportSpec(path string) json.RawMessage {
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil
	}
	var st stampRaw
	if json.Unmarshal(raw, &st) != nil || st.spec() == nil {
		return nil
	}
	return st.Spec
}

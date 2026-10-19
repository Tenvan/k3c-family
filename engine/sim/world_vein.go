package sim

// Adern (B-114, materialien-gebaeude.md § 1): unendliche Ressourcen unter Tage. Eine Ader wird einmal markiert und
// bleibt es, verschwindet nie und nimmt höchstens MaxWorkers Bauern gleichzeitig. Zielrate mit 2 Bauern =
// 2 × Amount ÷ WorkSeconds (data/economy.json › veins); der Trageweg zum Lager zählt nicht dazu.
// Adern stehen nicht in economy.Gatherables: Der endliche Startvorrat („Alles abbauen“) kennt sie nicht.

// VeinData ist eine Ader-Art aus data/economy.json › veins.
type VeinData struct {
	Gatherable
	MaxWorkers int
}

// gatherOf sind die Abbau-Werte einer Knoten-Art (Ader oder endliche Ressource); false, wenn sie keine hat.
func gatherOf(kind string) (Gatherable, bool) {
	if v, ok := economy.Veins[kind]; ok {
		return v.Gatherable, true
	}
	g, ok := economy.Gatherables[kind]
	return g, ok
}

func isVein(n *ResourceNode) bool {
	_, ok := economy.Veins[n.Kind]
	return ok
}

// veinWorkers zählt die Bauern mit einem Sammel-Auftrag an der Ader.
func veinWorkers(w *World, n *ResourceNode) int {
	count := 0
	for _, t := range w.Troops {
		if t.Job != nil && t.Job.Type == "gather" && t.Job.NodeID == n.ID {
			count++
		}
	}
	return count
}

// nodeFree: Nimmt der Knoten noch einen Bauern? Endliche Ressourcen einen, Adern bis MaxWorkers.
func nodeFree(w *World, n *ResourceNode) bool {
	if isVein(n) {
		return veinWorkers(w, n) < economy.Veins[n.Kind].MaxWorkers
	}
	return !isWorker(w, n.WorkerID)
}

// gatherVein baut an der Ader ab; jeder Bauer hat seinen eigenen Fortschritt im Auftrag, die Ader bleibt.
func gatherVein(t *Troop, g Gatherable, dt float64) {
	t.Job.progress += dt / g.WorkSeconds
	if t.Job.progress >= 1 {
		t.Job = &Job{Type: "carry", Resource: g.Resource, Amount: g.Amount}
	}
}

package backend

import (
	"time"

	assets "k3c/tools/assetkit"
)

// Kandidaten je Asset-Slot (B-677, WZ33): Zuordnungen, Vorrat, Netz und Katalog in einer Liste, „Zuordnen“ lädt und
// übernimmt in einem Aufruf. Die Logik liegt in tools/assetkit/candidates*.go, Felder wie api/candidateTypes.ts.

// AssetCandidatesView ist die Antwort von AssetCandidates; Stand ist der Stand des Registers für AssetCandidateAssign
// (Schreibregel wie AssetsSet), offline = Netz-Kandidaten sind nicht zuordenbar.
type AssetCandidatesView struct {
	Offline    bool               `json:"offline"`
	Stand      string             `json:"stand"`
	Candidates []assets.Candidate `json:"kandidaten"`
}

// AssetCandidates liefert die Kandidaten des Slots (Binding); query leer = Beschreibung des Slots, limit begrenzt die
// Katalog-Treffer (≤ 0 = 50).
func (a *App) AssetCandidates(slot, query string, limit int) (AssetCandidatesView, error) {
	stand, err := assets.Stamp(a.root) // vor dem Lesen: ein späterer Schreiber macht den Stand veraltet, nie die Liste unbemerkt alt
	if err != nil {
		return AssetCandidatesView{}, err
	}
	view := AssetCandidatesView{Stand: stand, Offline: a.sourceState().offline(a.context())}
	view.Candidates, err = assets.Candidates(a.root, slot, assets.CandidateOptions{Query: query, Limit: limit,
		Offline: view.Offline, PublicURL: "/", VorratURL: vorratURL})
	if err != nil {
		return AssetCandidatesView{}, err
	}
	return view, nil
}

// AssetCandidateAssign ordnet den Kandidaten id dem Slot zu und lädt ihn bei Bedarf (Binding): Netz → Vorrat → Spiel in
// einem Aufruf, nur auf dem Stand stand aus AssetCandidates. confirmedBytes muss bei Vorschlägen deren Größe sein.
// Der Kandidat wird neu aufgelöst (Slot, Lizenz, Größe aus Liste und Katalog, nie aus der Anfrage); Netz-Kandidaten laden
// nicht offline und nicht, während ein Download der Seite Quellen läuft (das Übernehmen eines schon geladenen Eintrags schon). Scheitert ein Schritt, ist nichts zugeordnet.
func (a *App) AssetCandidateAssign(slot, id, stand string, confirmedBytes int64) (AssetSaved, error) {
	st := a.sourceState()
	s, next, err := assets.AssignCandidate(a.context(), a.root, slot, id, stand, confirmedBytes, assets.AssignOptions{
		Client:  st.client,
		Offline: func() bool { return st.offline(a.context()) },
		Reserve: st.reserve, // nur um einen Download, nicht um das Übernehmen aus dem Vorrat
		Release: st.release,
		Now:     time.Now(),
	})
	if err != nil {
		return AssetSaved{}, err
	}
	a.publish(evAssetsChanged)
	return AssetSaved{Slot: AssetSlotView{Slot: *s, Status: assets.Status(a.root, *s)}, Stand: next}, nil
}

// reserve belegt den Download-Platz für einen Aufruf von AssetCandidateAssign; false, wenn schon einer läuft.
func (st *sourceState) reserve() bool {
	st.mu.Lock()
	defer st.mu.Unlock()
	if st.busy {
		return false
	}
	st.busy = true
	return true
}

func (st *sourceState) release() {
	st.mu.Lock()
	st.busy = false
	st.mu.Unlock()
}

package room

import "slices"

// Beobachter (B-352, TR4.2): ein Gerät ohne Platz, z. B. das Gerät von `sim_test` mit Clients. Es bekommt Level und
// Zustand jeder Stufe, in der ein Monarch steht (ohne Monarchen die Startstufe), belegt keinen Monarchen und nimmt
// keine Eingabe an (`input`, `addSlot`, `pause` → bad_request). Es zählt nicht als verbundenes Gerät: Ein Raum nur mit
// Beobachtern rechnet nicht (wie ein leerer), wird aber nicht aufgeräumt, solange ein Beobachter verbunden ist.

// Observe tritt dem Raum per Code als Beobachter bei; hatte das Gerät dort Plätze, sind sie danach frei.
func (m *Manager) Observe(id string, peer Peer, code string) (*Room, error) {
	err := ErrRoomNotFound
	defer func() {
		m.notify(err == nil)
		m.logRejected("observe", id, code, err)
	}()
	m.mu.Lock()
	defer m.mu.Unlock()
	r := m.rooms[code]
	if m.shut {
		err = ErrClosed
	} else if r != nil {
		err = r.lockedObserve(id, peer)
	}
	return r, err
}

// CreateObserved ist Create ohne Platz: Das Gerät legt den Raum an (oder tritt dem offenen Stand bei) und beobachtet.
func (m *Manager) CreateObserved(id string, peer Peer, name string, fresh bool, depth int, opts Options) (*Room, error) {
	return m.createWith(id, name, fresh, depth, opts, nil, func(r *Room) error { return r.lockedObserve(id, peer) })
}

func (r *Room) lockedObserve(id string, peer Peer) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.closed {
		return ErrRoomNotFound
	}
	r.observe(id, peer)
	return nil
}

// observe nimmt das Gerät als Beobachter auf; ein früherer Eintrag desselben Geräts gibt seine Monarchen frei.
func (r *Room) observe(id string, peer Peer) {
	old := r.devices[id]
	if old != nil && old.connected && old.peer != peer {
		old.peer.Replaced()
	}
	d := &device{peer: peer, slots: map[int]int{}, connected: true, stages: map[int]bool{}, observer: true}
	r.devices[id] = d
	r.log().Info("🔌 Beobachter im Raum", "device", short(id), "geraete", r.connected(), "beobachter", r.watching())
	peer.Joined(r.Code, r.Name, r.isl.WorldID(), []Seat{})
	r.pushState(d, nil)
	if old != nil && len(old.slots) > 0 {
		for _, idx := range old.slots {
			r.release(idx)
		}
		r.afterDisconnect() // Plätze melden und speichern wie beim Verlassen
		return
	}
	r.broadcastSeats()
}

// unobserve: Der Beobachter geht (leave oder Abbruch). Ist danach kein Gerät mehr da, läuft die Frist EmptyFor.
func (r *Room) unobserve(id string) {
	delete(r.devices, id)
	r.log().Info("👋 Beobachter hat den Raum verlassen", "device", short(id), "beobachter", r.watching())
	if r.connected() == 0 && r.watching() == 0 { // Frist EmptyFor ab jetzt, auch nach langem Beobachten eines leeren Raums
		r.emptySince = r.m.now()
	}
}

// watching zählt die verbundenen Beobachter.
func (r *Room) watching() int {
	n := 0
	for _, d := range r.devices {
		if d.connected && d.observer {
			n++
		}
	}
	return n
}

// observerStages sind die Stufen aller Monarchen, aufsteigend; ohne Monarchen die Startstufe.
func (r *Room) observerStages() []int {
	if len(r.monarchs) == 0 {
		return []int{r.start}
	}
	out := make([]int, 0, len(r.monarchs))
	for i := range r.monarchs {
		out = append(out, r.isl.StageOf(i))
	}
	slices.Sort(out)
	return slices.Compact(out)
}

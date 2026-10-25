package net

import "slices"

// Warteschlange einer Verbindung (Kopf von ws.go): Zustände ersetzen einander, andere Nachrichten bleiben.

// push stellt o hinten an; ein Zustand nimmt einen wartenden derselben Stufe (waiting) heraus und übernimmt dessen
// Ereignisse. So staut ein hängendes Gerät nur andere Nachrichten, nie veraltete Zustände (B-280).
func (c *conn) push(o out) {
	c.qmu.Lock()
	n, lost, i := len(c.queue), 0, c.waiting(o)
	switch {
	case i >= 0:
		o.events, lost = mergeEvents(c.queue[i], o.state)
		o.dropped = c.queue[i].dropped + droppedOf(c.queue[i].state) + lost
		c.queue = append(slices.Delete(c.queue, i, i+1), o)
		c.drops++
		if lost > 0 && c.warned {
			lost = 0
		}
		c.warned = c.warned || lost > 0
	case n >= sendBuffer:
		c.qmu.Unlock()
		c.s.log.Warn("🐢 Warteschlange voll, Verbindung zu", "ns", "ws", "device", short(c.device))
		c.cancel()
		return
	default:
		c.queue = append(c.queue, o)
	}
	c.qmu.Unlock()
	if lost > 0 {
		c.s.log.Warn("🐢 Ereignisse verworfen", "ns", "ws", "device", short(c.device), "anzahl", lost)
	}
	select {
	case c.wake <- struct{}{}:
	default:
	}
}

// waiting ist der Platz des wartenden Zustands derselben Stufe, den der Zustand o ersetzt: gesucht bis zum letzten
// Level dieser Stufe (danach kommt ein snap), andere Nachrichten dazwischen bleiben an ihrem Platz; -1: keiner.
func (c *conn) waiting(o out) int {
	for i := len(c.queue) - 1; o.state != nil && i >= 0; i-- {
		q := c.queue[i]
		switch {
		case q.stage != o.stage || (q.state == nil && !q.level):
		case q.level:
			return -1
		default:
			return i
		}
	}
	return -1
}

// pop nimmt die vorderste Nachricht; false: Warteschlange leer.
func (c *conn) pop() (out, bool) {
	c.qmu.Lock()
	defer c.qmu.Unlock()
	if len(c.queue) == 0 {
		return out{}, false
	}
	o := c.queue[0]
	c.queue[0] = out{}
	c.queue = c.queue[1:]
	return o, true
}

// mergeEvents sind die Ereignisse des verworfenen Zustands old und des neuen cur, höchstens maxEvents (die neuesten),
// und die Zahl der dabei verworfenen; nil, wenn old keine hatte.
func mergeEvents(old out, cur map[string]any) ([]any, int) {
	prev := old.events
	if prev == nil {
		prev, _ = old.state["events"].([]any)
	}
	if len(prev) == 0 {
		return nil, 0
	}
	now, _ := cur["events"].([]any)
	all := append(append(make([]any, 0, len(prev)+len(now)), prev...), now...)
	lost := max(0, len(all)-maxEvents)
	return all[lost:], lost
}

// droppedOf ist eventsDropped eines Zustands aus stateOf (fehlt bei 0).
func droppedOf(state map[string]any) int {
	n, _ := state["eventsDropped"].(float64)
	return int(n)
}

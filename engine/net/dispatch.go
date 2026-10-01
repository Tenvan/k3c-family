package net

import (
	"encoding/json"

	"k3c/engine/room"
	"k3c/engine/sim"
)

// handle verarbeitet eine Nachricht nach dem Handschlag. Fehler gehen nur an dieses Gerät, die Verbindung bleibt.
func (c *conn) handle(data []byte) {
	var m inMsg
	if err := json.Unmarshal(data, &m); err != nil {
		c.fail(codeBadRequest)
		return
	}
	r := c.current()
	var err error
	switch {
	case (m.T == "create" || m.T == "join") && r != nil:
		err = room.ErrBadRequest
	case m.T == "create":
		err = c.create(m)
	case m.T == "join":
		err = c.enter(c.s.cfg.Rooms.Join(c.device, c, m.Room, m.Slots))
	case r == nil:
		err = room.ErrBadRequest // addSlot, removeSlot, input, leave, hello und Unbekanntes ohne Raum
	default:
		err = c.roomMessage(r, m)
	}
	if err != nil {
		c.fail(codeOf(err))
	}
}

func (c *conn) fail(code string) { c.enqueue(errMsg(code)) }

func (c *conn) create(m inMsg) error {
	if m.Fresh == nil || m.Slots == nil {
		return room.ErrBadRequest
	}
	return c.enter(c.s.cfg.Rooms.Create(c.device, c, m.Save, *m.Fresh, m.Depth, m.Slots))
}

func (c *conn) enter(r *room.Room, err error) error {
	if err != nil {
		return err
	}
	c.mu.Lock()
	c.room = r
	c.mu.Unlock()
	return nil
}

// roomMessage: Nachrichten, die nur im Raum gelten.
func (c *conn) roomMessage(r *room.Room, m inMsg) error {
	switch m.T {
	case "addSlot", "removeSlot":
		if m.Slot == nil {
			return room.ErrBadRequest
		}
		if m.T == "addSlot" {
			return r.AddSlot(c.device, *m.Slot)
		}
		err := r.RemoveSlot(c.device, *m.Slot)
		if err == nil && c.current() == r && !r.Has(c.device) {
			c.left() // letzter Slot weg: Gerät hat den Raum verlassen
		}
		return err
	case "input":
		return c.input(r, m)
	case "leave":
		r.Leave(c.device)
		c.left()
		return nil
	}
	return room.ErrBadRequest
}

// input verrechnet die Eingaben aller genannten Slots; ack ist danach das höchste seq dieser Verbindung.
func (c *conn) input(r *room.Room, m inMsg) error {
	if len(m.P) == 0 || m.Seq <= 0 {
		return room.ErrBadRequest
	}
	for _, p := range m.P {
		if p.Slot == nil {
			return room.ErrBadRequest
		}
		if err := r.Input(c.device, *p.Slot, sim.PlayerCommand{MoveX: p.MoveX, Sprint: p.Sprint, Pay: p.Pay}); err != nil {
			return err
		}
	}
	if m.Seq > c.seq.Load() {
		c.seq.Store(m.Seq)
	}
	return nil
}

// left: Das Gerät ist in keinem Raum mehr und bekommt die Raumliste.
func (c *conn) left() {
	c.leaveRoom()
	c.enqueue(roomsMsg{"rooms", c.s.cfg.Rooms.Rooms()})
}

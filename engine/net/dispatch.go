package net

import (
	"encoding/json"
	"fmt"

	"k3c/engine/room"
	"k3c/engine/sim"
)

// handle verarbeitet eine Nachricht nach dem Handschlag. Fehler gehen nur an dieses Gerät, die Verbindung bleibt.
func (c *conn) handle(data []byte) {
	defer func() {
		if p := recover(); p != nil { // ein Fehler im Raum-Code beendet nur diese Nachricht
			c.s.log.Error("💥 Nachricht abgestürzt", "device", c.device, "err", fmt.Sprint(p))
			c.fail(codeBadRequest)
		}
	}()
	var m inMsg
	if err := json.Unmarshal(data, &m); err != nil {
		c.fail(codeBadRequest)
		return
	}
	if m.T != "input" { // input kommt je Tick, alles andere ist selten und zeigt den Ablauf
		c.s.log.Debug("📨 Nachricht", "ns", "ws", "device", short(c.device), "t", m.T, "room", m.Room, "save", m.Save, "slots", m.Slots)
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

func (c *conn) fail(code string) {
	c.s.log.Warn("🚫 Fehler an Gerät", "ns", "ws", "device", short(c.device), "code", code)
	c.enqueue(errMsg(code))
}

func (c *conn) create(m inMsg) error {
	if err := room.ValidSlots(m.Slots); err != nil {
		return err // too_many_slots vor allen anderen Prüfungen (docs/protocol.md › Beitreten, Schritt 1)
	}
	if m.Fresh == nil {
		return room.ErrBadRequest
	}
	return c.enter(c.s.cfg.Rooms.Create(c.device, c, m.Save, *m.Fresh, m.Depth, m.Slots, room.Options{Grade: m.Grade, Goal: m.Goal, Defeat: m.Defeat}))
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
			return r.AddSlot(c.device, c, *m.Slot)
		}
		left, err := r.RemoveSlot(c.device, c, *m.Slot)
		if left {
			c.left() // letzter Slot weg: Gerät hat den Raum verlassen
		}
		return err
	case "input":
		return c.input(r, m)
	case "leave":
		r.Leave(c.device, c)
		c.left()
		return nil
	case "dev":
		return r.Dev(c.device, c, room.DevAction{Action: m.Action, Slot: m.Slot, Amount: m.Amount, Resource: m.Resource, Factor: m.Factor, Paused: m.Paused})
	}
	return room.ErrBadRequest
}

// input verrechnet die Eingaben aller genannten Slots; ack ist danach das höchste seq dieser Verbindung.
func (c *conn) input(r *room.Room, m inMsg) error {
	if len(m.P) == 0 || m.Seq <= 0 {
		return room.ErrBadRequest
	}
	in := map[int]sim.PlayerCommand{}
	for _, p := range m.P {
		if p.Slot == nil {
			return room.ErrBadRequest
		}
		if _, dup := in[*p.Slot]; dup {
			return room.ErrBadRequest
		}
		in[*p.Slot] = sim.PlayerCommand{MoveX: p.MoveX, Sprint: p.Sprint, Pay: p.Pay}
	}
	if err := r.Input(c.device, c, in); err != nil {
		return err
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

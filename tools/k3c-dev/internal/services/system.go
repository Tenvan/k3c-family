package services

import (
	"context"
	"net"
	"strconv"
	"sync"
	"time"

	gnet "github.com/shirou/gopsutil/v4/net"
	"github.com/shirou/gopsutil/v4/process"
)

// Listener meldet, ob auf einem Port gelauscht wird, und welche PID das tut (0 = unbekannt). Test-Naht.
type Listener func(ctx context.Context, port int) (pid int, listening bool)

// Metrics sind die Messwerte eines Prozesses.
type Metrics struct {
	CPU     float64   // Prozent seit der letzten Messung
	Memory  uint64    // RSS in Bytes
	Started time.Time // Startzeit des Prozesses
}

// Sampler misst einen Prozess. Test-Naht.
type Sampler func(ctx context.Context, pid int) (Metrics, error)

// PortListener prüft per Verbindung, ob jemand am Port lauscht, und sucht die PID in der Verbindungstabelle.
func PortListener(ctx context.Context, port int) (int, bool) {
	conn, err := (&net.Dialer{Timeout: time.Second}).DialContext(ctx, "tcp", net.JoinHostPort("127.0.0.1", strconv.Itoa(port)))
	if err != nil {
		return 0, false
	}
	_ = conn.Close()
	conns, err := gnet.ConnectionsWithContext(ctx, "tcp")
	if err != nil {
		return 0, true
	}
	return ownerPID(conns, port), true
}

// systemPID ist die höchste PID, die Windows selbst gehört (0 Leerlauf, 4 System, z. B. http.sys); sie wird nie
// als Besitzer genommen, damit ein Stopp sie nicht treffen kann.
const systemPID = 4

// ownerPID sucht den Prozess, den eine Verbindung an 127.0.0.1:port tatsächlich erreicht: wer dort, an 0.0.0.0 oder
// (Dual-Stack) an :: lauscht. Andere Adressen (::1, LAN) zählen nicht, sonst träfe ein Stopp ein fremdes Programm
// mit demselben Port. 127.0.0.1 gewinnt vor den Wildcards, wie beim Verbinden selbst.
func ownerPID(conns []gnet.ConnectionStat, port int) int {
	best, rank := 0, 0
	for _, c := range conns {
		if c.Status != "LISTEN" || c.Laddr.Port != uint32(port) || c.Pid <= systemPID {
			continue
		}
		r := map[string]int{"127.0.0.1": 3, "0.0.0.0": 2, "::": 1}[c.Laddr.IP]
		if r > rank {
			best, rank = int(c.Pid), r
		}
	}
	return best
}

// maxCached begrenzt die gemerkten Prozesse des Samplers.
//
// ponytail: fester Deckel statt Aufräumen je Lauf; jeder Neustart bringt eine neue PID, mehr als ein paar Dutzend
// kommen in einer Sitzung nicht zusammen.
const maxCached = 64

// NewSampler liefert einen Sampler über gopsutil. Die CPU-Last braucht zwei Messungen am selben Prozess-Objekt,
// deshalb merkt er sich die Objekte je PID; die erste Messung liefert 0 %.
func NewSampler() Sampler {
	var mu sync.Mutex
	procs := map[int]*process.Process{}
	return func(ctx context.Context, pid int) (Metrics, error) {
		mu.Lock()
		p, ok := procs[pid]
		if !ok {
			if len(procs) >= maxCached {
				clear(procs)
			}
			var err error
			if p, err = process.NewProcessWithContext(ctx, int32(pid)); err != nil {
				mu.Unlock()
				return Metrics{}, err
			}
			procs[pid] = p
		}
		mu.Unlock()
		return sample(ctx, p)
	}
}

func sample(ctx context.Context, p *process.Process) (Metrics, error) {
	var m Metrics
	cpu, err := p.PercentWithContext(ctx, 0)
	if err != nil {
		return m, err
	}
	mem, err := p.MemoryInfoWithContext(ctx)
	if err != nil {
		return m, err
	}
	m.CPU, m.Memory = cpu, mem.RSS
	if ms, err := p.CreateTimeWithContext(ctx); err == nil {
		m.Started = time.UnixMilli(ms)
	}
	return m, nil
}

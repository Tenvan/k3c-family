package net

import (
	"math"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"
)

// CPU-Last des Server-Prozesses für /api/status (B-175): Feld cpu in Prozent einer CPU, gemittelt über das Intervall
// seit dem letzten Aufruf. Die Quelle ist austauschbar (Config.CPU); fehlt sie, fehlt das Feld.

// clockTicks ist USER_HZ von Linux (/proc/self/stat zählt in Ticks; auf allen gängigen Systemen 100).
const clockTicks = 100

// systemCPU ist die Quelle, die NewHandler ohne Config.CPU einsetzt; nil = keine (Tests setzen es).
// ponytail: nur Linux (/proc); auf Windows/macOS fehlt das Feld, Prozesszeit per Build-Tag, wenn nötig.
var systemCPU = newCPUMeter(procCPUTime)

// cpuMeter macht aus einer kumulativen Prozess-CPU-Zeit Prozent einer CPU je Intervall.
type cpuMeter struct {
	read func() (time.Duration, bool)
	mu   sync.Mutex
	at   time.Time
	used time.Duration
}

// newCPUMeter liefert die Quelle; nil, wenn read nicht verfügbar ist.
func newCPUMeter(read func() (time.Duration, bool)) func() (float64, bool) {
	if _, ok := read(); !ok {
		return nil
	}
	return (&cpuMeter{read: read}).percent
}

// percent liefert die Last seit dem letzten Aufruf (beim ersten seit Start des Messers).
func (m *cpuMeter) percent() (float64, bool) {
	used, ok := m.read()
	if !ok {
		return 0, false
	}
	m.mu.Lock()
	defer m.mu.Unlock()
	now := time.Now()
	if m.at.IsZero() {
		m.at = now.Add(-time.Second) // erster Aufruf: kein Vorwert, Näherung über 1 s
	}
	pct := float64(used-m.used) / float64(now.Sub(m.at)) * 100
	m.at, m.used = now, used
	return math.Round(max(pct, 0)*10) / 10, true
}

// procCPUTime liest utime+stime aus /proc/self/stat.
func procCPUTime() (time.Duration, bool) {
	data, err := os.ReadFile("/proc/self/stat")
	if err != nil {
		return 0, false
	}
	// Nach dem Prozessnamen "(…)" ist state Feld 3, utime 14, stime 15.
	s := string(data)
	f := strings.Fields(s[strings.LastIndex(s, ")")+1:])
	if len(f) < 13 {
		return 0, false
	}
	u, err1 := strconv.ParseInt(f[11], 10, 64)
	st, err2 := strconv.ParseInt(f[12], 10, 64)
	if err1 != nil || err2 != nil {
		return 0, false
	}
	return time.Duration(u+st) * time.Second / clockTicks, true
}

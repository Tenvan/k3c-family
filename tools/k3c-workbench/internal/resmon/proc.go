package resmon

// ProcInfo ist ein Kindprozess.
type ProcInfo struct {
	PID  uint32
	Name string
	RSS  uint64
}

// procSnapshot sind die Prozessdaten, die die Go-Laufzeit nicht kennt (plattformabhängig).
type procSnapshot struct {
	rss      uint64
	private  uint64
	handles  int
	children []ProcInfo // alle Nachkommen des Prozesses
}

package resmon

// procInfo ist ein Kindprozess.
type procInfo struct {
	pid  uint32
	name string
	rss  uint64
}

// procSnapshot sind die Prozessdaten, die die Go-Laufzeit nicht kennt (plattformabhängig).
type procSnapshot struct {
	rss      uint64
	private  uint64
	handles  int
	children []procInfo // alle Nachkommen des Prozesses
}

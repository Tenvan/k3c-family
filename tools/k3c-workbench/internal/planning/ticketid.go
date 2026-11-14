package planning

import (
	"strconv"
	"strings"
)

// Ticket-Nummern ab B-1000 (B-863): Nummern haben keine feste Länge, Schlüssel und Sortierung gehen über die Zahl.

// ticketNum ist die Zahl einer Ticket-ID (B-1000 → 1000); ohne Zahl -1.
func ticketNum(id string) int {
	n, err := strconv.Atoi(strings.TrimPrefix(id, "B-"))
	if err != nil || !strings.HasPrefix(id, "B-") {
		return -1
	}
	return n
}

// ticketLess ordnet Ticket-IDs nach Nummer (B-999 vor B-1000), sonst nach Text.
func ticketLess(a, b string) bool {
	na, nb := ticketNum(a), ticketNum(b)
	if na >= 0 && nb >= 0 && na != nb {
		return na < nb
	}
	return a < b
}

// ticketFileID ist die ID aus einem Dateinamen wie `B-1000-titel.md`.
func ticketFileID(name string) string {
	if rest, ok := strings.CutPrefix(name, "B-"); ok {
		if nr, _, ok := strings.Cut(rest, "-"); ok {
			return "B-" + nr
		}
	}
	return ""
}

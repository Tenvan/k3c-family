package playtest

import "testing"

func TestTranscriptTextZeilenMitAufnahmezeit(t *testing.T) {
	tr := Transcript{AudioStartMs: 1_000_000, Segments: []Segment{
		{StartMs: 1_000_000 + 65_000, EndMs: 1_000_000 + 70_000, Text: " Das war knapp. "},
		{StartMs: 1_000_000 + 3_000, EndMs: 1_000_000 + 4_000, Text: ""},
		{StartMs: 1_000_000 - 500, EndMs: 1_000_000 + 1_000, Text: "Los geht's"},
	}}
	got := TranscriptText(tr)
	if got != "[01:05] Das war knapp.\n[00:00] Los geht's\n" {
		t.Fatalf("%q", got)
	}
}

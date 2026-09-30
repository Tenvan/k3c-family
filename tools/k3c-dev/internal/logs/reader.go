package logs

import (
	"bytes"
	"errors"
	"io"
	"os"
	"regexp"
	"time"
)

// DefaultBudget ist die höchste Zahl gelesener Bytes je Anfrage.
const DefaultBudget = 8 << 20

// blockSize ist die Größe eines Leseblocks; eine Variable, damit Tests Blockgrenzen erzwingen können.
var blockSize int64 = 1 << 20

// Query filtert einen Rückwärts-Lauf. Leere Felder filtern nicht.
type Query struct {
	MinLevel string
	NS       string
	Pattern  *regexp.Regexp // auf Msg
	Since    time.Time
	Limit    int   // 0 = unbegrenzt
	Budget   int64 // 0 = DefaultBudget
}

// Result ist das Ergebnis eines Rückwärts-Laufs, neueste Einträge zuerst.
type Result struct {
	Entries   []Entry `json:"entries"`
	BytesRead int64   `json:"bytesRead"`
	BudgetHit bool    `json:"budgetHit"` // ältere Treffer möglich
	Skipped   int     `json:"skipped"`   // unlesbare Zeilen
}

// Scan liest path von hinten, bis Limit, Since oder Budget greift. Eine fehlende Datei ist ein leeres Ergebnis.
func Scan(path string, q Query) (Result, error) {
	res := Result{Entries: []Entry{}}
	f, err := os.Open(path)
	if errors.Is(err, os.ErrNotExist) {
		return res, nil
	}
	if err != nil {
		return res, err
	}
	defer func() { _ = f.Close() }()
	st, err := f.Stat()
	if err != nil {
		return res, err
	}
	budget := q.Budget
	if budget <= 0 {
		budget = DefaultBudget
	}
	minRank := LevelRank(q.MinLevel)
	res.BytesRead, res.BudgetHit, err = reverseLines(f, st.Size(), budget, func(line []byte) bool {
		e, ok := parse(line)
		if !ok {
			res.Skipped++
			return true
		}
		if !q.Since.IsZero() && e.Time.Before(q.Since) {
			return false
		}
		if q.matches(e, minRank) {
			res.Entries = append(res.Entries, e)
		}
		return q.Limit <= 0 || len(res.Entries) < q.Limit
	})
	return res, err
}

func (q Query) matches(e Entry, minRank int) bool {
	if q.MinLevel != "" && LevelRank(e.Level) < minRank {
		return false
	}
	if q.NS != "" && e.NS != q.NS {
		return false
	}
	return q.Pattern == nil || q.Pattern.MatchString(e.Msg)
}

// reverseLines gibt die Zeilen von hinten an fn, bis fn false liefert oder das Budget verbraucht ist. Kein
// bufio.Scanner: der bricht bei 64 KB je Zeile still ab.
func reverseLines(r io.ReaderAt, size, budget int64, fn func([]byte) bool) (read int64, budgetHit bool, err error) {
	pos := size
	var carry []byte
	for pos > 0 {
		if read >= budget {
			return read, true, nil
		}
		n := min(blockSize, pos)
		pos -= n
		buf := make([]byte, n, n+int64(len(carry)))
		if _, err := r.ReadAt(buf, pos); err != nil && !errors.Is(err, io.EOF) {
			return read, false, err
		}
		read += n
		lines := bytes.Split(append(buf, carry...), []byte{'\n'})
		carry = nil
		first := 0
		if pos > 0 {
			carry, first = lines[0], 1 // Anfang der Zeile liegt im nächsten Block
		}
		for i := len(lines) - 1; i >= first; i-- {
			if line := bytes.TrimRight(lines[i], "\r"); len(line) > 0 && !fn(line) {
				return read, false, nil
			}
		}
	}
	return read, false, nil
}

// SinceResult sind neue Einträge ab einem Byte-Cursor, älteste zuerst.
type SinceResult struct {
	Entries   []Entry `json:"entries"`
	Cursor    int64   `json:"cursor"`    // für den nächsten Aufruf
	Truncated bool    `json:"truncated"` // Datei kleiner geworden, Budget oder Limit schnitt ältere ab
	Skipped   int     `json:"skipped"`
}

// ReadSince liest vorwärts ab cursor bis zur letzten vollständigen Zeile, höchstens budget Bytes.
func ReadSince(path string, cursor int64, limit int, budget int64) (SinceResult, error) {
	res := SinceResult{Entries: []Entry{}, Cursor: cursor}
	data, size, err := readRange(path, &res, budget)
	start := res.Cursor
	if err != nil || data == nil {
		return res, err
	}
	end := bytes.LastIndexByte(data, '\n') + 1
	for line := range bytes.SplitSeq(data[:end], []byte{'\n'}) {
		if line = bytes.TrimRight(line, "\r"); len(line) == 0 {
			continue
		}
		if e, ok := parse(line); ok {
			res.Entries = append(res.Entries, e)
		} else {
			res.Skipped++
		}
	}
	res.Cursor += int64(end)
	if int64(len(data)) < size-start {
		res.Truncated = true // Budget: der Rest kommt beim nächsten Aufruf
	}
	if limit > 0 && len(res.Entries) > limit {
		res.Entries, res.Truncated = res.Entries[len(res.Entries)-limit:], true
	}
	return res, nil
}

// readRange liest ab res.Cursor höchstens budget Bytes; ist die Datei kleiner geworden, beginnt es bei 0.
func readRange(path string, res *SinceResult, budget int64) ([]byte, int64, error) {
	f, err := os.Open(path)
	if errors.Is(err, os.ErrNotExist) {
		return nil, 0, nil
	}
	if err != nil {
		return nil, 0, err
	}
	defer func() { _ = f.Close() }()
	st, err := f.Stat()
	if err != nil {
		return nil, 0, err
	}
	size := st.Size()
	if res.Cursor > size || res.Cursor < 0 {
		res.Cursor, res.Truncated = 0, true
	}
	if budget <= 0 {
		budget = DefaultBudget
	}
	data := make([]byte, min(size-res.Cursor, budget))
	if _, err := f.ReadAt(data, res.Cursor); err != nil && !errors.Is(err, io.EOF) {
		return nil, size, err
	}
	return data, size, nil
}

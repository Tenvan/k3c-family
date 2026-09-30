package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"regexp"
	"slices"
	"time"

	"k3c/tools/k3c-dev/internal/logs"
)

// Grenzen der Reiter Log und Fehler (B-064): feste Limits, Fehler über die letzten 24 h wie logs_errors.
var logLimits = []int{100, 200, 500}

const errorsWindow = 24 * time.Hour

// LogQuery sind die Filter des Reiters Log; ein leeres MinLevel heißt alle.
type LogQuery struct {
	MinLevel string `json:"minLevel"`
	NS       string `json:"ns"`
	Pattern  string `json:"pattern"`
	Limit    int    `json:"limit"`
}

// LogView sind die Einträge des Reiters Log, neueste zuerst (die Oberfläche dreht sie um).
type LogView struct {
	Entries   []logs.Entry `json:"entries"`
	BytesRead int64        `json:"bytesRead"`
	BudgetHit bool         `json:"budgetHit"`
	Skipped   int          `json:"skipped"`
	Missing   bool         `json:"missing"` // Datei gibt es (noch) nicht
}

// ErrorsView sind die Gruppen des Reiters Fehler (verdichtet), häufigste zuerst.
type ErrorsView struct {
	Groups    []logs.Group `json:"groups"`
	Entries   int          `json:"entries"`
	BytesRead int64        `json:"bytesRead"`
	BudgetHit bool         `json:"budgetHit"`
	Missing   bool         `json:"missing"`
}

// logFile prüft die Quelle gegen die Liste (nur so wird aus einem Namen ein Pfad) und ob die Datei fehlt.
func (a *App) logFile(source string) (path string, missing bool, err error) {
	a.wait()
	if path, err = a.srv.LogPath(source); err != nil {
		return "", false, err
	}
	_, err = os.Stat(path)
	return path, errors.Is(err, fs.ErrNotExist), nil
}

func checkLevel(level string, allowed []string) error {
	if !slices.Contains(allowed, level) {
		return fmt.Errorf("unbekanntes Level %q; gültig: %v", level, allowed)
	}
	return nil
}

// LogsQuery liefert gefilterte Einträge einer Log-Datei (Binding).
func (a *App) LogsQuery(source string, in LogQuery) (LogView, error) {
	path, missing, err := a.logFile(source)
	if err != nil {
		return LogView{}, err
	}
	if !slices.Contains(logLimits, in.Limit) {
		return LogView{}, fmt.Errorf("limit %d; gültig: %v", in.Limit, logLimits)
	}
	if err := checkLevel(in.MinLevel, append([]string{""}, logs.Levels()...)); err != nil {
		return LogView{}, err
	}
	q := logs.Query{MinLevel: in.MinLevel, NS: in.NS, Limit: in.Limit}
	if in.Pattern != "" {
		if q.Pattern, err = regexp.Compile(in.Pattern); err != nil {
			return LogView{}, fmt.Errorf("ungültige Suche, kein regulärer Ausdruck: %w", err)
		}
	}
	res, err := logs.Scan(path, q)
	if err != nil {
		return LogView{}, err
	}
	return LogView{Entries: res.Entries, BytesRead: res.BytesRead, BudgetHit: res.BudgetHit, Skipped: res.Skipped,
		Missing: missing}, nil
}

// LogsErrors fasst Warnungen oder Fehler der letzten 24 h zu Gruppen zusammen (Binding).
func (a *App) LogsErrors(source, level string) (ErrorsView, error) {
	path, missing, err := a.logFile(source)
	if err != nil {
		return ErrorsView{}, err
	}
	if err := checkLevel(level, []string{"WARN", "ERROR"}); err != nil {
		return ErrorsView{}, err
	}
	res, err := logs.Scan(path, logs.Query{MinLevel: level, Since: time.Now().Add(-errorsWindow)})
	if err != nil {
		return ErrorsView{}, err
	}
	groups := logs.Digest(res.Entries)
	if groups == nil {
		groups = []logs.Group{}
	}
	return ErrorsView{Groups: groups, Entries: len(res.Entries), BytesRead: res.BytesRead, BudgetHit: res.BudgetHit,
		Missing: missing}, nil
}

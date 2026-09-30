package main

import (
	"k3c/tools/k3c-dev/internal/mcpsrv"
	"k3c/tools/k3c-dev/internal/usage"
)

// McpOverview ist Band 1 und 2 der MCP-Seite (B-065): Zustand des Servers und seine Zähler.
type McpOverview struct {
	MCP   MCPState        `json:"mcp"`
	Stats mcpsrv.Snapshot `json:"stats"`
}

// UsageRules sind die Regeln der Statistik für die Erklärzeile; sie stehen nur in internal/usage.
type UsageRules struct {
	OutlierFactor      float64 `json:"outlierFactor"`
	OutlierFloorMs     float64 `json:"outlierFloorMs"`
	BaselineCalls      int     `json:"baselineCalls"`
	PercentileErrorPct int     `json:"percentileErrorPct"`
}

// McpUsageView ist die Nutzungsstatistik (Sitzung, Gesamtzeit, Minuten-Zeitreihe) mit ihren Regeln.
type McpUsageView struct {
	usage.Usage
	Rules UsageRules `json:"rules"`
}

// McpOverview liefert Zustand und Zähler des MCP-Servers (Binding).
func (a *App) McpOverview() McpOverview {
	a.wait()
	a.mu.Lock()
	st := a.mcp
	a.mu.Unlock()
	return McpOverview{MCP: st, Stats: a.srv.Stats()}
}

// McpRestart startet den HTTP-Teil des MCP-Servers neu; Zähler und Katalog bleiben (Binding). Ein Fehler steht im
// Zustand, damit das Badge ihn zeigt.
func (a *App) McpRestart() MCPState {
	a.wait()
	err := a.srv.Restart()
	a.setMCP(err)
	a.mu.Lock()
	defer a.mu.Unlock()
	return a.mcp
}

// McpInstructions liefert den Text, den jeder Client beim Verbinden bekommt (Binding).
func (a *App) McpInstructions() string {
	return mcpsrv.Instructions()
}

// McpCalls liefert das Aufruf-Log, neueste zuerst, laufende eingeschlossen (Binding).
func (a *App) McpCalls() []mcpsrv.Call {
	a.wait()
	return a.srv.Calls()
}

// McpUsage liefert die Nutzungsstatistik mit ihren Regeln (Binding).
func (a *App) McpUsage() McpUsageView {
	a.wait()
	return McpUsageView{Usage: a.tracker.Snapshot(), Rules: UsageRules{OutlierFactor: usage.OutlierFactor,
		OutlierFloorMs: usage.OutlierFloorMs, BaselineCalls: usage.BaselineCalls,
		PercentileErrorPct: usage.PercentileErrorPct}}
}

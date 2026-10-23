package main

import (
	"os"
	"path/filepath"
	"strconv"

	"k3c/tools/k3c-workbench/internal/resmon"
)

// envHeapDumpMB ist die Schwelle des Live-Heaps in MiB für das erste Heap-Profil (Vorgabe 1024).
const envHeapDumpMB = "K3C_DEV_HEAP_DUMP_MB"

// startResources startet das Ressourcen-Monitoring: Proben alle 30 s (Verlauf 24 h), eine Zeile je 5 min im Log
// (`ns=res`), Heap- und Goroutinen-Profile unter logs/profile/ bei Überschreiten einer Schwelle. Abfragen: Tool
// workbench_resources, `go tool pprof http://127.0.0.1:<MCP-Port>/debug/pprof/heap`. Muss nach dem Öffnen von Log
// und Konsolenpuffer laufen, aber vor dem MCP-Server (Config.Resources).
func (a *App) startResources() {
	cfg := resmon.Config{Dir: filepath.Join(a.root, "logs", "profile"), Log: a.log.Logger}
	if mb, err := strconv.ParseUint(os.Getenv(envHeapDumpMB), 10, 32); err == nil && mb > 0 {
		cfg.HeapDump = mb << 20
	}
	a.res = resmon.New(cfg)
	a.res.Gauge("Log-Dateien", "B", func() int64 { return dirBytes(filepath.Join(a.root, "logs")) })
	go a.res.Run(a.svcCtx)
}

// dirBytes summiert die Dateien eines Ordners (ohne Unterordner); ein Fehler zählt als 0.
func dirBytes(dir string) int64 {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return 0
	}
	var n int64
	for _, e := range entries {
		if fi, err := e.Info(); err == nil && !e.IsDir() && fi.Mode().IsRegular() {
			n += fi.Size()
		}
	}
	return n
}


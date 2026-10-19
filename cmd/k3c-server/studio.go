package main

import (
	"context"
	"log/slog"
	"net/http"
	"os"

	"k3c/engine/studio"
	assets "k3c/tools/assetkit"
)

// withStudio hängt das Asset-Studio (engine/studio) unter /api/studio/ an den Spiel-Handler. Es gibt es nur im Dev-Mode
// und nur, wenn der Server in einem Checkout läuft (go.mod mit „module k3c“ über dem Arbeitsordner): das Studio schreibt
// in data/ und public/. Aus ist es im Release-Image. Standardmäßig bedient es nur Loopback, K3C_STUDIO_LAN=1 öffnet es
// fürs Heimnetz; K3C_STUDIO=0 schaltet es aus.
func withStudio(ctx context.Context, cfg config, game http.Handler, log *slog.Logger) http.Handler {
	if !cfg.dev || os.Getenv("K3C_STUDIO") == "0" {
		return game
	}
	wd, err := os.Getwd()
	if err != nil {
		return game
	}
	root, err := assets.FindRoot(wd)
	if err != nil {
		log.Info("🚫 Asset-Studio aus: kein Checkout (go.mod mit module k3c) über dem Arbeitsordner", "ns", "main")
		return game
	}
	lan := os.Getenv("K3C_STUDIO_LAN") == "1"
	mux := http.NewServeMux()
	mux.Handle(studio.Prefix, studio.New(ctx, root).Handler(lan))
	mux.Handle("/", game)
	log.Info("🚀 Asset-Studio bereit: /studio/index.html", "ns", "main", "root", root, "lan", lan)
	return mux
}

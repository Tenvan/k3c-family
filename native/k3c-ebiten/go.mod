module k3c/native/k3c-ebiten

go 1.27.0

require (
	github.com/hajimehoshi/ebiten/v2 v2.10.6
	k3c/native/k3c-rig v0.0.0
)

require (
	github.com/ebitengine/gomobile v0.0.0-20260820040257-d11f821a26a6 // indirect
	github.com/ebitengine/hideconsole v1.0.0 // indirect
	github.com/ebitengine/purego v0.11.3 // indirect
	golang.org/x/sync v0.22.0 // indirect
	golang.org/x/sys v0.47.0 // indirect
)

// Rig-Rechnung und Zeichnen der Rig-Figuren (NV3.3): Pakete rig und paint aus dem Rig-Prototyp, ohne Kopie
replace k3c/native/k3c-rig => ../k3c-rig

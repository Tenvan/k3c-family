package hubplan

import "k3c/tools/k3c-studio/internal/goengine"

// editor ist der Planer unter root mit der Go-Engine als Referenz (goengine, statt der EXE; keine Simulation).
func editor(root string) Editor { return Editor{Root: root, Engine: goengine.Call("hub", root)} }

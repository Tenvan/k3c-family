package hubplan

// editor ist der Planer unter root mit der echten Engine (B-924: im Prozess, engine/simcli hub; keine Simulation).
func editor(root string) Editor { return Editor{Root: root, Engine: Direct(root)} }

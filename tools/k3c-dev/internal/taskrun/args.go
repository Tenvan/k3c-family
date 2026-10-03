package taskrun

import (
	"fmt"
	"strings"
)

// ForbiddenArgChars sind Zeichen, die auf der Kette StartArgs → task.exe →
// mvdan/sh → npm.cmd → cmd.exe → jest mindestens einmal als Shell- oder
// Batch-Metazeichen gelesen werden (am realen Pfad belegt, siehe
// internal/mcpsrv/tools_check.go und dessen Tests): "&" verkettet ein
// weiteres Kommando, ">"/"<" leiten um, "|" leitet weiter, '"' wird nicht
// maskierend transportiert, "^" ist cmd.exes Eskapezeichen, "%" löst
// Variablenexpansion in einer .cmd-Datei aus, "`" ist ein
// Kommandosubstitutions-Zeichen, ";" trennt Kommandos in mvdan/sh, "," bleibt
// ausgeschlossen, weil keine der Stationen unter eigener Kontrolle steht.
//
// Die Liste liegt hier und nicht mehr in mcpsrv, weil check_run, die
// Task-Bindung der Oberfläche und task_start dieselbe Kette bedienen. Zwei
// Listen, die auseinanderlaufen, wären genau das Loch, das diese Prüfung
// verhindern soll. Fail-closed: nicht bereinigen, ablehnen.
const ForbiddenArgChars = `&|<>^"%` + "`" + `;,`

// MaxArgLen begrenzt ein einzelnes Argument. Niemand braucht ein
// kilobytelanges Argument, und eine Grenze macht sehr lange Eingaben
// (Speicher, Log-Rauschen) uninteressant.
const MaxArgLen = 200

// ValidateArg lehnt ein Argument mit Begründung ab, statt es zu bereinigen.
// what benennt den Gegenstand im Fehlertext ("Argument", "Muster"), damit
// jeder Aufrufer seine gewohnte Formulierung behält.
func ValidateArg(what, value string) error {
	if value == "" {
		return fmt.Errorf("%s darf nicht leer sein", what)
	}
	if len(value) > MaxArgLen {
		return fmt.Errorf("%s %q ist zu lang (%d Zeichen, erlaubt sind höchstens %d)", what, value, len(value), MaxArgLen)
	}
	for _, r := range value {
		if r < 0x20 || r == 0x7f {
			return fmt.Errorf("%s %q enthält ein Steuerzeichen - abgelehnt statt bereinigt", what, value)
		}
		if strings.ContainsRune(ForbiddenArgChars, r) {
			return fmt.Errorf("%s %q enthält das nicht erlaubte Zeichen %q - abgelehnt statt bereinigt", what, value, string(r))
		}
	}
	return nil
}

// ValidateArgs prüft Zusatzargumente für `task <name> -- args...`. Anders als
// bei check_run-Testmustern ist ein führendes "-" hier erlaubt: `-run Foo`
// oder `--coverage` sind genau der Zweck des Feldes.
func ValidateArgs(args []string) error {
	for _, a := range args {
		if err := ValidateArg("Argument", a); err != nil {
			return err
		}
	}
	return nil
}

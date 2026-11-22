package uikit

import (
	"fmt"
	"reflect"
	"regexp"
	"slices"
)

var (
	goAppRe  = regexp.MustCompile(`(?s)interface GoApp \{(.*?)\n\}`)
	methodRe = regexp.MustCompile(`(?m)^\s+(\w+)\(`)
)

// TSMethods liest die Methodennamen von `interface GoApp` aus frontend/src/api/wails.ts einer App.
func TSMethods(src []byte) ([]string, error) {
	body := goAppRe.FindSubmatch(src)
	if body == nil {
		return nil, fmt.Errorf("interface GoApp nicht gefunden")
	}
	var names []string
	for _, m := range methodRe.FindAllSubmatch(body[1], -1) {
		names = append(names, string(m[1]))
	}
	slices.Sort(names)
	return names, nil
}

// CheckMethods prüft, dass die Freigabeliste genau GoApp aus wails.ts (src) ist und jede Methode an typ existiert.
// Jede App ruft es in ihrem Test (TestUIMethodsWieWailsTS) mit ihrer eigenen Liste und ihrem eigenen wails.ts auf.
func CheckMethods(src []byte, methods map[string]bool, typ reflect.Type) error {
	ts, err := TSMethods(src)
	if err != nil {
		return err
	}
	var goNames []string
	for name := range methods {
		goNames = append(goNames, name)
		if _, ok := typ.MethodByName(name); !ok {
			return fmt.Errorf("%s hat keine Methode %s", typ, name)
		}
	}
	slices.Sort(goNames)
	if !slices.Equal(ts, goNames) {
		return fmt.Errorf("GoApp = %v\nuiMethods = %v", ts, goNames)
	}
	return nil
}

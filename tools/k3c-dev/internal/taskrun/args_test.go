package taskrun

import (
	"strings"
	"testing"
)

func TestValidateArgs_LehntAbStattZuBereinigen(t *testing.T) {
	ok := []string{"-run", "TestFoo", "--coverage", "src/a b.spec.ts", "Foo.*Bar", "--testPathPattern=Foo"}
	if err := ValidateArgs(ok); err != nil {
		t.Fatalf("gültige Argumente abgelehnt: %v", err)
	}

	bad := map[string]string{
		"a&b":                            `"&"`,
		"a|b":                            `"|"`,
		"a>b":                            `">"`,
		"a<b":                            `"<"`,
		"a^b":                            `"^"`,
		`a"b`:                            `"\""`,
		"a%b":                            `"%"`,
		"a`b":                            "\"`\"",
		"a;b":                            `";"`,
		"a,b":                            `","`,
		"a\tb":                           "Steuerzeichen",
		"":                               "leer",
		strings.Repeat("x", MaxArgLen+1): "zu lang",
	}
	for arg, want := range bad {
		err := ValidateArgs([]string{"ok", arg})
		if err == nil {
			t.Errorf("%q nicht abgelehnt", arg)
			continue
		}
		if !strings.Contains(err.Error(), want) {
			t.Errorf("%q: Fehler nennt %q nicht: %v", arg, want, err)
		}
	}
}

func TestValidateArg_WhatSteuertDenText(t *testing.T) {
	err := ValidateArg("Muster", "a|b")
	if err == nil || !strings.HasPrefix(err.Error(), `Muster "a|b" enthält das nicht erlaubte Zeichen "|"`) {
		t.Errorf("Text mit what=Muster: %v", err)
	}
}

package simrun

import (
	"cmp"
	"unicode/utf8"
)

// Option „je Variante“ (B-662): Ein Auftrag mit n festen Varianten wird n + 1 Läufe in der Warteschlange: ein
// Basis-Lauf ohne Varianten und je Variante einer, mit gleichen Seeds und gemeinsamem Label-Präfix.

// perVariantPrefix ist das Präfix der Labels, wenn der Auftrag keines hat.
const perVariantPrefix = "je Variante"

// perVariant teilt den Auftrag: zuerst der Basis-Lauf, dann je Variante ein Lauf; keiner trägt PerVariant weiter.
func (s Spec) perVariant() []Spec {
	prefix := cmp.Or(s.Label, perVariantPrefix)
	base := s
	base.PerVariant, base.Vary = false, nil
	base.Label = variantLabel(prefix, "Basis")
	out := []Spec{base}
	for _, v := range s.Vary {
		one := base
		one.Vary = []string{v}
		one.Label = variantLabel(prefix, v)
		out = append(out, one)
	}
	return out
}

// variantLabel ist „<präfix> · <teil>“ in höchstens maxLabel Zeichen; zu lang kürzt zuerst das Präfix (mindestens
// 8 Zeichen bleiben), dann das Ende.
func variantLabel(prefix, part string) string {
	const sep = " · "
	room := max(maxLabel-utf8.RuneCountInString(sep+part), 8)
	return clipRunes(clipRunes(prefix, room)+sep+part, maxLabel)
}

// clipRunes kürzt s auf höchstens n Zeichen, das letzte ist dann „…“.
func clipRunes(s string, n int) string {
	r := []rune(s)
	if len(r) <= n {
		return s
	}
	return string(r[:n-1]) + "…"
}

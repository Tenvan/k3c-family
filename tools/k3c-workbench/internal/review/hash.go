package review

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
	"unicode/utf16"
)

// RendererVersion ist gleich RENDERER_VERSION in tools/paper/schema/hash.ts (paper-spec.md › Hash-Regel).
const RendererVersion = "paper-0+resvg-js-2.6.2"

// Kanonisch ist die kanonische JSON-Form aus hash.ts: Schlüssel nach UTF-16-Codeeinheiten sortiert, ohne Leerraum,
// CRLF in Texten als LF, Zahlen und Texte wie JavaScript JSON.stringify.
func Kanonisch(wert any) string {
	var b strings.Builder
	kanonisch(&b, wert)
	return b.String()
}

func kanonisch(b *strings.Builder, wert any) {
	switch v := wert.(type) {
	case string:
		b.WriteString(jsText(strings.ReplaceAll(v, "\r\n", "\n")))
	case []any:
		b.WriteByte('[')
		for i, e := range v {
			if i > 0 {
				b.WriteByte(',')
			}
			kanonisch(b, e)
		}
		b.WriteByte(']')
	case map[string]any:
		keys := make([]string, 0, len(v))
		for k := range v {
			keys = append(keys, k)
		}
		slices.SortFunc(keys, func(a, c string) int { return slices.Compare(utf16.Encode([]rune(a)), utf16.Encode([]rune(c))) })
		b.WriteByte('{')
		for i, k := range keys {
			if i > 0 {
				b.WriteByte(',')
			}
			b.WriteString(jsText(k))
			b.WriteByte(':')
			kanonisch(b, v[k])
		}
		b.WriteByte('}')
	case float64:
		b.WriteString(jsZahl(v))
	case bool:
		b.WriteString(strconv.FormatBool(v))
	default:
		b.WriteString("null")
	}
}

// jsZahl formatiert wie Number.prototype.toString: kürzeste Darstellung, Exponent ab 1e21 und unter 1e-6, nie -0.
func jsZahl(f float64) string {
	if f == 0 || math.IsNaN(f) || math.IsInf(f, 0) {
		if f == 0 {
			return "0"
		}
		return "null"
	}
	if a := math.Abs(f); a >= 1e21 || a < 1e-6 {
		s := strconv.FormatFloat(f, 'e', -1, 64)
		mant, exp, _ := strings.Cut(s, "e")
		sign, ziffern := exp[:1], strings.TrimLeft(exp[1:], "0")
		return mant + "e" + sign + ziffern
	}
	return strconv.FormatFloat(f, 'f', -1, 64)
}

// jsText setzt einen Text in Anführungszeichen wie JSON.stringify (kein HTML-Escaping, U+2028 bleibt roh).
func jsText(s string) string {
	var b strings.Builder
	b.WriteByte('"')
	for _, r := range s {
		switch r {
		case '"':
			b.WriteString(`\"`)
		case '\\':
			b.WriteString(`\\`)
		case '\b':
			b.WriteString(`\b`)
		case '\f':
			b.WriteString(`\f`)
		case '\n':
			b.WriteString(`\n`)
		case '\r':
			b.WriteString(`\r`)
		case '\t':
			b.WriteString(`\t`)
		default:
			if r < 0x20 {
				fmt.Fprintf(&b, `\u%04x`, r)
			} else {
				b.WriteRune(r)
			}
		}
	}
	b.WriteByte('"')
	return b.String()
}

func sha(text string) string {
	sum := sha256.Sum256([]byte(text))
	return hex.EncodeToString(sum[:])
}

// SpecHash ist der SHA-256 (hex) der kanonischen Form; `$schema` zählt nicht. Gilt ebenso für die Stil-Datei.
func SpecHash(raw []byte) (string, error) {
	var wert any
	if err := json.Unmarshal(raw, &wert); err != nil {
		return "", err
	}
	if m, ok := wert.(map[string]any); ok {
		delete(m, "$schema")
	}
	return sha(Kanonisch(wert)), nil
}

// RenderKey ist Cache- und Golden-Schlüssel: SHA-256 über `specHash:stilHash:RendererVersion`.
func RenderKey(specHash, stilHash string) string {
	return renderKeyMit(specHash, stilHash, RendererVersion)
}

func renderKeyMit(specHash, stilHash, renderer string) string {
	return sha(specHash + ":" + stilHash + ":" + renderer)
}

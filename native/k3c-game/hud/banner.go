package hud

// Meldungen (Spiegel von HudScene.ts › eventText/showBanner und bannerQueue.ts, B-414): Ereignis → Text, Kürzen auf
// eine Zeile, Warteschlange je Spieler. Die Zeit kommt nur aus Frame.Clock.

import (
	"math"
	"slices"
	"strings"
	"unicode/utf8"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Dauer einer Meldung in Sekunden; bei mehr als zwei wartenden die Hälfte.
const bannerSeconds = 2.8

// bannerMaxChars: längste Meldung in Zeichen; längerer Text wird mit „…“ gekürzt. Die längste Meldung der Texte
// (Burgfall, 81 Zeichen) passt ganz (B-813); was nicht in die Zelle passt, zeichnet draw kleiner (fitPx).
const bannerMaxChars = 90

// fitBanner macht eine Meldung einzeilig (Umbrüche und Leerraum zu einem Leerzeichen) und kürzt sie auf max Zeichen.
func fitBanner(text string, maxChars int) string {
	one := strings.Join(strings.Fields(text), " ")
	if utf8.RuneCountInString(one) <= maxChars {
		return one
	}
	r := []rune(one)
	return strings.TrimRight(string(r[:maxChars-1]), " ") + "…"
}

// enqueueBanner hängt eine Meldung an die Warteschlange; dieselbe Meldung steht nur einmal (laufend oder wartend),
// eine neue ersetzt die laufende nicht, eine leere wird nicht eingereiht.
func enqueueBanner(queue []string, showing, text string) []string {
	fitted := fitBanner(text, bannerMaxChars)
	if fitted == "" || fitted == showing || slices.Contains(queue, fitted) {
		return queue
	}
	return append(queue, fitted)
}

// bannerQueue ist die Meldung eines Spielers: die laufende bis until, dahinter die wartenden.
type bannerQueue struct {
	showing string
	until   float64
	queue   []string
}

// add reiht eine Meldung ein.
func (b *bannerQueue) add(text string) { b.queue = enqueueBanner(b.queue, b.showing, text) }

// step schaltet zur Zeit now weiter: Ist die laufende abgelaufen, kommt die nächste (oder keine).
func (b *bannerQueue) step(now float64) {
	if b.showing != "" && now < b.until {
		return
	}
	b.showing = ""
	if len(b.queue) == 0 {
		return
	}
	b.showing = b.queue[0]
	b.queue = b.queue[:copy(b.queue, b.queue[1:])]
	d := bannerSeconds
	if len(b.queue) > 2 {
		d /= 2
	}
	b.until = now + d
}

// eventText ist der Text der Meldung zu einem Ereignis der Simulation ("" = keine). biomeID ist das Biom der Stufe
// des Ereignisses (für „arrived“).
func eventText(lang string, e view.Event, biomeID string) string {
	d := e.Data
	switch e.Type() {
	case "dusk", "skillPoint", "armed", "castleFallen":
		return tr(lang, "ev."+e.Type())
	case "night", "dawn":
		return tf(lang, "ev."+e.Type(), "day", intOf(d, "day"))
	case "wave":
		if intOf(d, "count") <= 0 {
			return ""
		}
		return tf(lang, "ev.wave", "n", intOf(d, "count"))
	case "chest":
		return tf(lang, "ev.chest", "p", intOf(d, "player")+1, "gold", intOf(d, "gold"))
	case "built", "destroyed":
		return tf(lang, "ev."+e.Type(), "name", siteName(lang, strOf(d, "kind")))
	case "goldStolen":
		return tf(lang, "ev.goldStolen", "p", intOf(d, "player")+1, "amount", intOf(d, "amount"))
	case "playerDown":
		return tf(lang, "ev.playerDown", "p", intOf(d, "player")+1)
	case "arrived":
		return nameOf(lang, "biome", biomeID, strOf(d, "name"))
	}
	return ""
}

// siteName ist der Name einer Bauart aus den Texten, sonst aus den Daten, sonst die ID.
func siteName(lang, kind string) string {
	name := buildingInfo[kind].Name
	if name == "" {
		name = kind
	}
	return nameOf(lang, "site", kind, name)
}

// numOf liest eine Zahl aus einem Ereignis (int oder float64; fehlt: 0).
func numOf(d sim.Event, key string) float64 {
	switch v := d[key].(type) {
	case int:
		return float64(v)
	case float64:
		return v
	case int64:
		return float64(v)
	}
	return 0
}

func intOf(d sim.Event, key string) int { return int(math.Round(numOf(d, key))) }

func strOf(d sim.Event, key string) string {
	s, _ := d[key].(string)
	return s
}

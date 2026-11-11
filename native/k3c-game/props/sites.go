package props

// Preis-Anzeigen und Wartegründe an Bauplätzen und Burg (Spiegel von src/scenes/render/priceTagRules.ts,
// siteStatusView.ts und src/scenes/hud/economyLabels.ts). Reine Regeln ohne Ebitengine; Texte entstehen je Inhalt
// einmal (texts), danach ohne Allokation.

import (
	"math"
	"strconv"
	"strings"
)

// priceTagRange: Preisschild und Zahlen erscheinen, wenn ein Monarch so viele Units nah am Ziel steht (viewRules.ts).
const priceTagRange = 6

// candidate ist ein Ziel mit Preisschild: Platz oder Burg (Nummer und Ort).
type candidate struct {
	id int
	x  float64
}

// nearestTags hängt die Nummern der Ziele an out an, deren Preisschild steht: je lebendem Monarchen (players: Orte,
// gefallene fehlen) das nächste Ziel in Reichweite, bei gleichem Abstand die kleinere Nummer; jede Nummer einmal
// (B-395).
func nearestTags(out []int, cands []candidate, players []float64) []int {
	for _, px := range players {
		best := -1
		for i, c := range cands {
			d := math.Abs(c.x - px)
			if d >= priceTagRange {
				continue
			}
			if best < 0 {
				best = i
				continue
			}
			bd := math.Abs(cands[best].x - px)
			if d < bd || (d == bd && c.id < cands[best].id) {
				best = i
			}
		}
		if best >= 0 && !hasID(out, cands[best].id) {
			out = append(out, cands[best].id)
		}
	}
	return out
}

func hasID(ids []int, id int) bool {
	for _, v := range ids {
		if v == id {
			return true
		}
	}
	return false
}

// offersPrice: unbezahlt und bezahlbar (payable, B-653) oder gebaute Werkstatt mit freiem Bogen-Regal.
func offersPrice(kind, state string, payable bool, bows int) bool {
	if state == "unpaid" {
		return payable
	}
	return state == "built" && kind == "workshop" && bows < buildings["workshop"].BowRack
}

// offersHubUpgrade: Die Burg nimmt Münzen für den Ausbau, solange Gold offen ist und noch kein Ausbau läuft (B-591).
func offersHubUpgrade(has bool, state string, gold, paid int) bool {
	return has && state == "" && gold > paid
}

// waitReason ist der Grund, warum ein Bau wartet.
type waitReason uint8

const (
	waitNone waitReason = iota
	waitWorker
	waitMaterial
	waitDanger
)

// waitOf ist der Wartegrund aus dem Zustand eines Baus (state oder upgrade): waitingWorker heißt bei Gefahr Gefahr.
func waitOf(state string, danger bool) waitReason {
	switch state {
	case "waitingWorker":
		if danger {
			return waitDanger
		}
		return waitWorker
	case "waitingMaterial", "waitingNextIsland":
		return waitMaterial
	}
	return waitNone
}

// siteWait ist der Wartegrund eines Platzes: erst der Bau, dann der Ausbau; läuft der Bau schon (Fortschritt) und
// fehlt nur der Bauer, bleibt der Platz leer (siteStatusView.ts).
func siteWait(state, upgrade string, progress float64, danger bool) (waitReason, string) {
	if state != "built" {
		r := waitOf(state, danger)
		if r == waitWorker && progress > 0 {
			return waitNone, ""
		}
		return r, state
	}
	return waitOf(upgrade, danger), upgrade
}

// textKey ist der Inhalt eines Texts; texts legt je Inhalt einen String einmal an.
type textKey struct {
	what  uint8 // textPrice, textStatus, … (Konstanten unten)
	kind  string
	state string
	n, m  int
}

const (
	textPrice uint8 = iota
	textStatus
	textClearing
	textName
	textExit
	textHubPrice
	textHubStatus
	textBow
)

// texts merkt sich die gebauten Texte (wenige je Spiel: Platzart × Zustand × Tiefe).
type texts map[textKey]string

// get liefert den Text zu k; beim ersten Mal baut ihn make (eine Funktion ohne eingefangene Werte, sonst entsteht
// je Aufruf ein Abschluss).
func (t texts) get(k textKey, make func(textKey) string) string {
	if s, ok := t[k]; ok {
		return s
	}
	s := make(k)
	t[k] = s
	return s
}

// resList nennt die Rohstoffe als „Holz, Stein“.
func resList(c cost) string {
	names := materials(c)
	for i, r := range names {
		names[i] = resourceNames[r]
	}
	return strings.Join(names, ", ")
}

// priceLabel: Name und Material „Mauer · 20 Holz“ (siteView.ts › drawPrice).
func priceLabel(name string, c cost) string {
	parts := []string{name}
	for _, r := range materials(c) {
		parts = append(parts, strconv.Itoa(c[r])+" "+resourceNames[r])
	}
	return strings.Join(parts, " · ")
}

// waitText: Text des Wartegrunds (texts.de.ts › wait.*); res nennt das fehlende Material.
func waitText(r waitReason, state, res string) string {
	switch {
	case r == waitWorker:
		return "wartet auf Bauer"
	case r == waitDanger:
		return "wartet: Gefahr"
	case state == "waitingNextIsland":
		return "Tor-Material fehlt (" + res + ")"
	case res != "":
		return "wartet auf Material (" + res + ")"
	}
	return "wartet auf Material"
}

// siteText baut die Texte der Bauplätze (k.kind = Platzart; n = Tiefe bzw. Stufe, m = Wartegrund).
func siteText(k textKey) string {
	name := siteName(k.kind)
	switch k.what {
	case textPrice:
		return priceLabel(name, buildCost(k.kind, k.n))
	case textBow:
		return priceLabel("Bogen", archerCost)
	case textClearing:
		return name + ": erst roden"
	case textStatus:
		c := buildCost(k.kind, k.n)
		if k.state != "waitingMaterial" && k.state != "waitingWorker" {
			c = upgradeCost(k.kind, k.n) // Ausbau zielt auf Stufe n+1
		}
		return name + ": " + waitText(waitReason(k.m), k.state, resList(c))
	}
	return name
}

// hubTitle: „Ausbau 2/3“ mit Schritt (B-592), ohne Schritte „Ausbauen“.
func hubTitle(step, steps int) string {
	if steps > 0 && step > 0 {
		return "Ausbau " + strconv.Itoa(step) + "/" + strconv.Itoa(steps)
	}
	return "Ausbauen"
}

// hubMaterial sind die Materialkosten des Hub-Ausbaus als Kostenliste.
func hubMaterial(st [5]int) cost {
	c := cost{}
	for i, r := range resourceOrder {
		if st[i] > 0 {
			c[r] = st[i]
		}
	}
	return c
}

// missingArr sind die Materialien, die im Vorrat fehlen (Material-Tor: nur nennen, was fehlt; Review GEB17).
func missingArr(need, stock [5]int) [5]int {
	var out [5]int
	for i := range need {
		if need[i] > 0 && stock[i] < need[i] {
			out[i] = need[i]
		}
	}
	return out
}

// coversArr: Der Vorrat deckt das Material need.
func coversArr(stock, need [5]int) bool {
	for i := range need {
		if need[i] > stock[i] {
			return false
		}
	}
	return true
}

// canAfford: Der Vorrat deckt das Material der Kosten.
func canAfford(stock [5]int, c cost) bool {
	for i, r := range resourceOrder {
		if c[r] > stock[i] {
			return false
		}
	}
	return true
}

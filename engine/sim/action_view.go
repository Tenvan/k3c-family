package sim

// Ausbilden und Tauschen für die Aktionsliste (B-410, W11.3): nur lesend aus dem Zahlziel abgelesen, das ein Takt A am
// Ort des Spielers bezahlen würde (findPayTarget); nichts gespeichert, Welt-JSON und Golden bleiben gleich.

// TrainAt ist der Beruf (miner, builder, craftsman), den der Spieler an seinem Ort ausbilden kann: Das Zahlziel ist
// ein bezahlbares Berufs-Angebot (unit_professions.go) und der Spieler hat Gold; sonst "".
func TrainAt(w *World, p *Player) string {
	t := findPayTarget(w, p)
	if p.Gold <= 0 || t == nil || t.offer == nil {
		return ""
	}
	if _, ok := professionOf(t.offer.kind()); !ok {
		return "" // Elite-Angebot der Schmiede: kein Beruf
	}
	return t.offer.kind()
}

// TradeAt ist die Seite des Händlers ("buy", "sell"), an der der Spieler jetzt tauschen kann, mit dem Material des
// Händlers (merchant.go › payMerchant): Kaufen braucht Gold und Platz im Lager, Verkaufen genug Material im Vorrat;
// sonst "", "".
func TradeAt(w *World, p *Player) (side, resource string) {
	t := findPayTarget(w, p)
	if t == nil || t.merchant == "" {
		return "", ""
	}
	m, mer := economy.Merchant, w.Merchant
	switch t.merchant {
	case "sell":
		if f := stockField(w.Stock, mer.Resource); f == nil || *f < m.Material {
			return "", ""
		}
	default:
		if p.Gold <= 0 || mer.BuyPaid >= m.Gold || resourceFull(w, mer.Resource) {
			return "", ""
		}
	}
	return t.merchant, mer.Resource
}

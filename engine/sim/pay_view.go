package sim

// Zahl-Aktion für das Protokoll (B-410, W11.3): Was A halten am Ort des Spielers jetzt bezahlen würde, aber nur
// Ausbilden (Berufs-Angebot) und Tauschen (Händler). Nur lesend über findPayTarget und die Prüfungen der Zahlung
// (offerPayable, payMerchant); nichts gespeichert, Welt-JSON und Golden-Daten bleiben gleich. `actionsOf` (engine/net)
// setzt daraus die Einträge `train` und `trade`.

// PayAction ist ein Eintrag `train` (Kind, Gold = Preis, Paid) oder `trade` (Side, Resource, Material, Gold = Kurs).
type PayAction struct {
	Action, Kind, Side, Resource string
	Gold, Paid, Material         int
}

// PayActionOf liefert die Zahl-Aktion von p; ok false, wenn A halten jetzt weder einen Beruf noch den Händler bezahlt.
func PayActionOf(w *World, p *Player) (PayAction, bool) {
	if !isAlive(p) {
		return PayAction{}, false
	}
	t := findPayTarget(w, p)
	switch {
	case t == nil:
		return PayAction{}, false
	case t.offer != nil:
		prof, ok := professionOf(t.offer.kind())
		if !ok || p.Gold <= 0 {
			return PayAction{}, false
		}
		return PayAction{Action: "train", Kind: t.offer.kind(), Gold: cheap(w, prof.Cost).Gold, Paid: t.offer.paid()}, true
	case t.merchant != "" && merchantWorks(w, p, t.merchant):
		m := economy.Merchant
		return PayAction{Action: "trade", Side: t.merchant, Resource: w.Merchant.Resource, Material: m.Material, Gold: m.Gold}, true
	}
	return PayAction{}, false
}

// merchantWorks: Wirkt ein Takt A halten am Händler jetzt (Bedingungen wie in payMerchant)?
func merchantWorks(w *World, p *Player, side string) bool {
	m, mer := economy.Merchant, w.Merchant
	if side == "sell" {
		f := stockField(w.Stock, mer.Resource)
		return f != nil && *f >= m.Material
	}
	return p.Gold > 0 && mer.BuyPaid < m.Gold && !resourceFull(w, mer.Resource)
}

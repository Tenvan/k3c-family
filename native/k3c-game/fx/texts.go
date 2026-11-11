package fx

// Spieltexte der Effekte (Spiegel von src/core/texts.*.ts › cast.noTarget); einzige Stelle mit Text im Paket.

import "k3c/native/k3c-game/view"

// noTargetText ist „kein Ziel“ in der Sprache lang (Deutsch für jede andere).
func noTargetText(lang string) string {
	if lang == view.LanguageEnglish {
		return "no target"
	}
	return "kein Ziel"
}

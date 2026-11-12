package menu

// Lizenzen und Version (B-808, Vorbild src/scenes/menuPages.ts › licenses und lizenzen.html): Eine Liste ist die
// Quelle für die Lizenz-Seite im Menü und die Lizenzdatei im Paket (LicenseText, `go run ./menu/licensefile`, Task
// native:game:paket). Die Schrift liegt mit ihrer Lizenzdatei unter public/schrift/ (B-815).

import (
	"strings"
)

// Version ist die Version des Spiels; der Build setzt sie (-ldflags "-X k3c/native/k3c-game/menu.Version=…" aus
// git describe, native/Taskfile.yml › game:build). Leer = unbekannt.
var Version string

// license ist ein Werk, das das Spiel nutzt: Teil (Engine, Schrift, Grafik, Audio), Name, Urheber, Lizenz, Quelle.
type license struct {
	part                         textKey
	work, holder, license, where string
}

// licenses sind die Werke mit Lizenzhinweis (Lizenzen aus den LICENSE-Dateien der Module in go.mod bzw. aus
// public/schrift und public/audio/README.md).
var licenses = []license{
	{txtLicEngine, "Ebitengine", "Hajime Hoshi", "Apache-2.0", "github.com/hajimehoshi/ebiten"},
	{txtLicEngine, "Oto, purego", "The Ebitengine Authors", "Apache-2.0", "github.com/ebitengine"},
	{txtLicFont, "Go Regular", "Bigelow & Holmes, Google", "BSD-3-Clause", "public/schrift/LICENSE"},
	{txtLicFont, "go-text/typesetting", "The go-text Authors", "Unlicense / BSD-3-Clause", "github.com/go-text/typesetting"},
	{txtLicGraphics, "x/image, rasterx", "The Go Authors, Steven R. Wiley", "BSD-3-Clause", "golang.org/x/image"},
	{txtLicAudio, "go-mp3", "Hajime Hoshi", "Apache-2.0", "github.com/hajimehoshi/go-mp3"},
	{txtLicAudio, "oggvorbis, vorbis", "Johann Freymuth", "MIT", "github.com/jfreymuth/oggvorbis"},
	{txtLicAudio, "Klänge und Musik", "Kenney, OpenGameArt", "CC0", "public/audio/README.md"},
}

// versionText ist die Versionszeile des Hauptmenüs („Version …“, ohne Angabe „unbekannt“).
func versionText(lang string) string {
	v := Version
	if v == "" {
		v = tr(lang, txtVersionUnknown)
	}
	return tr1(lang, txtVersion, v)
}

// LicenseText ist der Inhalt der Lizenzdatei im Paket (LIZENZEN.txt): Version und je Werk eine Zeile, deutsch und
// englisch aus derselben Liste wie die Lizenz-Seite.
func LicenseText() string {
	var b strings.Builder
	b.WriteString("K3C – Family Three Crowns, natives Spiel · native game\n")
	b.WriteString(versionText("de") + " · " + versionText("en") + "\n\n")
	for _, lang := range []string{"de", "en"} {
		b.WriteString(tr(lang, txtLicenses) + "\n")
		for _, l := range licenses {
			b.WriteString("- " + tr(lang, l.part) + ": " + l.work + " – " + l.holder + " – " + l.license + " – " + l.where + "\n")
		}
		b.WriteString("\n")
	}
	b.WriteString("Lizenztext der Schrift / font license: public/schrift/LICENSE\n")
	return b.String()
}

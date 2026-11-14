package menu

// Tabellen-Seiten: Tastenbelegung im Pause-Menü (B-807, Vorbild src/scenes/keyTable.ts) und Lizenzen im Hauptmenü
// (B-808). Eine Tafel aus Leinen-Papier mit Kopfzeile und je Eintrag einer Zeile; B, Esc und Home gehen zurück. Die
// Zeilen kommen von außen (Tasten: view.KeyTable aus game) bzw. aus der Lizenzliste; Draw baut die Texte nur bei neuer
// Sprache.

import (
	"image"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// Grenzen der Tafel: höchstens maxTableRows Zeilen und tableCols Spalten.
const (
	maxTableRows = 12
	tableCols    = 4
)

// table ist der Inhalt einer Tabellen-Seite: Kopfzeile, Zeilen und Spaltenanteile (Summe 1).
type table struct {
	title, hint string
	head        [tableCols]string
	rows        [maxTableRows][tableCols]string
	n           int
	share       [tableCols]float64
	lang        string
	ok          bool
	px          float64         // Schriftgröße, bei der jede Zelle in ihre Spalte passt (Zwischenspeicher)
	fit         image.Rectangle // Fenster, für das px gilt
}

// tableKind ist die Art der Seite.
type tableKind int

const (
	tableKeys tableKind = iota
	tableLicenses
)

// tableScreen zeigt eine Tabelle; jede Menü-Taste außer Hoch/Runter/Links/Rechts führt zurück.
type tableScreen struct {
	base
	m    *menus
	kind tableKind
	tb   table
}

func newKeys(m *menus) *tableScreen     { return &tableScreen{m: m, kind: tableKeys} }
func newLicenses(m *menus) *tableScreen { return &tableScreen{m: m, kind: tableLicenses} }

func (sc *tableScreen) Update(in view.Input, s view.Session) view.Step { return sc.run(in, s, sc) }

func (sc *tableScreen) handle(_ view.DeviceID, k key, _ view.Session) (view.StepKind, view.Screen) {
	if k == keyBack || k == keyHome || k == keyOK {
		sc.cue(view.CueBack, -1, "")
		return view.StepPop, nil
	}
	return view.StepStay, nil
}

func (sc *tableScreen) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	if lang := f.Settings.Language; !sc.tb.ok || sc.tb.lang != lang {
		sc.fill(lang)
	}
	sc.m.paint.table(dst, c, &sc.tb)
}

// fill baut die Texte der Tabelle in der Sprache lang.
func (sc *tableScreen) fill(lang string) {
	t := &sc.tb
	*t = table{lang: lang, ok: true}
	if sc.kind == tableLicenses {
		fillLicenses(t, lang)
		return
	}
	t.title, t.hint = tr(lang, txtKeys), tr(lang, txtKeysHint)
	t.head = [tableCols]string{tr(lang, txtKeyAction), tr(lang, txtKeysLeft), tr(lang, txtKeysRight), tr(lang, txtPadColumn)}
	t.share = [tableCols]float64{0.31, 0.23, 0.25, 0.21}
	if sc.m.keys == nil {
		return
	}
	for _, r := range sc.m.keys(lang) {
		if t.n < maxTableRows {
			t.rows[t.n] = [tableCols]string{keyActionText(lang, r.Action), r.Left, r.Right, r.Pad}
			t.n++
		}
	}
}

// fillLicenses baut die Lizenz-Seite aus der Lizenzliste.
func fillLicenses(t *table, lang string) {
	t.title, t.hint = tr(lang, txtLicenses), tr(lang, txtLicensesHint)
	t.head = [tableCols]string{tr(lang, txtLicPart), tr(lang, txtLicWork), tr(lang, txtLicHolder), tr(lang, txtLicLicense)}
	t.share = [tableCols]float64{0.14, 0.27, 0.33, 0.26}
	for _, l := range licenses {
		if t.n < maxTableRows {
			t.rows[t.n] = [tableCols]string{tr(lang, l.part), l.work, l.holder, l.license}
			t.n++
		}
	}
}

// keyActionTexts beschriften die Aktionen der Tastenbelegung (view.KeyActions).
var keyActionTexts = map[string]textKey{
	view.KeyMove: txtKeyMove, view.KeySprint: txtKeySprint, view.KeyPay: txtKeyPay, view.KeyAttack: txtKeyAttack,
	view.KeySkillMenu: txtKeySkillMenu, view.KeyPause: txtKeyPause, view.KeyFullscreen: txtKeyFullscreen,
}

// keyActionText ist die Beschriftung einer Aktion; Skill-Slots „Skill n“, unbekannte Aktionen ihr Name.
func keyActionText(lang, action string) string {
	if k, ok := keyActionTexts[action]; ok {
		return tr(lang, k)
	}
	switch action {
	case view.KeySkill1, view.KeySkill2, view.KeySkill3, view.KeySkill4:
		return tr1(lang, txtKeySkill, action[len(action)-1:])
	}
	return action
}

// tableGeom ist die Lage einer Tafel mit n Zeilen im Fenster der Seite g: Tafel, Zeilenhöhe und Schriftgröße.
type tableGeom struct {
	x, y, w, h float64
	rowH, px   float64
}

// tableOf rechnet die Lage (rein): Tafel zwischen Titel und Hinweis, Kopfzeile und n Zeilen gleich hoch, höchstens
// eine Kartenhöhe je Zeile; die Schrift füllt gut die Hälfte der Zeile.
func tableOf(g geom, n int) tableGeom {
	t := tableGeom{w: 0.9 * float64(g.win.Dx())}
	t.x = float64(g.win.Min.X) + (float64(g.win.Dx())-t.w)/2
	t.y = g.subY
	room := g.noteY - g.rowGap - t.y
	t.rowH = min(g.rowH, room/float64(n+1))
	t.h = t.rowH * float64(n+1)
	t.px = min(g.smallPx, 0.58*t.rowH)
	return t
}

// row liefert das Rechteck der Zeile i (0 = Kopfzeile).
func (t tableGeom) row(i int) (x, y, w, h float64) { return t.x, t.y + float64(i)*t.rowH, t.w, t.rowH }

// fitTable liefert die Schriftgröße, bei der jede Zelle (mit Abstand in) in ihre Spalte passt: höchstens t.px.
func (p *painter) fitTable(tb *table, t tableGeom, in float64) float64 {
	px := t.px
	inner := t.w - 4*in
	for i := range tb.n + 1 {
		cells := &tb.head
		if i > 0 {
			cells = &tb.rows[i-1]
		}
		for j, s := range cells {
			if w, _ := p.measure(s, t.px); w > 0 {
				px = min(px, t.px*(tb.share[j]*inner-2*in)/w)
			}
		}
	}
	return px
}

// table zeichnet die Tafel tb ins Fenster der Zelle c.
func (p *painter) table(dst *ebiten.Image, c view.Cell, tb *table) {
	g := geomOf(c.Rect)
	p.backdrop(dst, g)
	cx := float64(c.Rect.Min.X) + float64(c.Rect.Dx())/2
	p.centered(dst, tb.title, cx, g.titleY, g.titlePx, colTitle)
	t := tableOf(g, tb.n)
	if tb.fit != c.Rect || tb.px == 0 {
		tb.fit, tb.px = c.Rect, p.fitTable(tb, t, g.stripInset())
	}
	t.px = tb.px
	p.paper(dst, g, t.x, t.y, t.w, t.h, matLinen, false, false)
	for i := range tb.n + 1 {
		x, y, w, h := t.row(i)
		cells, clr := &tb.head, colLine
		if i == 0 {
			p.rect(dst, x, y+h-2*g.k, w, 2*g.k, colLine)
		} else {
			cells = &tb.rows[i-1]
			if i%2 == 1 {
				p.rect(dst, x, y, w, h, matLinen.licht)
			}
		}
		tx := x + g.stripInset()*2
		for j, s := range cells {
			p.text(dst, s, tx, y+(h-t.px*1.25)/2, t.px, clr)
			tx += tb.share[j] * (w - 4*g.stripInset())
		}
	}
	p.centered(dst, tb.hint, cx, g.hintY, g.smallPx, colText)
}

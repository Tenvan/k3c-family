package main

import "image/color"

// db32 ist die DawnBringer-32-Palette (Namen nach Lospec). Index 0 der Bild-Palette bleibt transparent.
var db32 = []struct {
	name string
	rgb  uint32
}{
	{"black", 0x000000}, {"valhalla", 0x222034}, {"loulou", 0x45283c}, {"oiled-cedar", 0x663931},
	{"rope", 0x8f563b}, {"tahiti-gold", 0xdf7126}, {"twine", 0xd9a066}, {"pancho", 0xeec39a},
	{"golden-fizz", 0xfbf236}, {"atlantis", 0x99e550}, {"christi", 0x6abe30}, {"elf-green", 0x37946e},
	{"dell", 0x4b692f}, {"verdigris", 0x524b24}, {"opal", 0x323c39}, {"deep-koamaru", 0x3f3f74},
	{"venice-blue", 0x306082}, {"royal-blue", 0x5b6ee1}, {"cornflower", 0x639bff}, {"viking", 0x5fcde4},
	{"light-steel", 0xcbdbfc}, {"white", 0xffffff}, {"heather", 0x9badb7}, {"topaz", 0x847e87},
	{"dim-gray", 0x696a6a}, {"smokey-ash", 0x595652}, {"clairvoyant", 0x76428a}, {"brown", 0xac3232},
	{"mandy", 0xd95763}, {"plum", 0xd77bba}, {"rain-forest", 0x8f974a}, {"stinger", 0x8a6f30},
}

// palette liefert transparent + DB32 und die Zuordnung Name → Index.
func palette() (color.Palette, map[string]uint8) {
	p := color.Palette{color.NRGBA{}}
	idx := make(map[string]uint8, len(db32))
	for i, c := range db32 {
		p = append(p, color.NRGBA{R: uint8(c.rgb >> 16), G: uint8(c.rgb >> 8), B: uint8(c.rgb), A: 0xff})
		idx[c.name] = uint8(i + 1)
	}
	return p, idx
}

package review

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strconv"
)

// Overlay vergleicht einen Frame von Basis (qa) und Arbeitsstand (qn) mit `pixeldiff`; das Overlay liegt im Cache des
// Arbeitsstands (Datei absolut).
func Overlay(ctx context.Context, r Renderer, root string, qa, qn Quelle, animation string, frame int) (PixelAntwort, error) {
	alt, err := Bild(ctx, r, root, qa, animation, frame, "")
	if err != nil {
		return PixelAntwort{}, err
	}
	neu, err := Bild(ctx, r, root, qn, animation, frame, "")
	if err != nil {
		return PixelAntwort{}, err
	}
	key := RenderKey(qa.SpecHash, qa.StilHash)
	name := "pixeldiff-" + key[:16] + "-" + animation + "-" + strconv.Itoa(frame) + ".png"
	ziel := filepath.Join(qn.ordner(root, r), name)
	tmp, err := tempName(filepath.Dir(ziel), name)
	if err != nil {
		return PixelAntwort{}, err
	}
	p, err := r.PixelDiff(ctx, alt.Datei, neu.Datei, tmp)
	if err == nil {
		err = uebernehme(tmp, ziel)
	}
	if err != nil {
		_ = os.Remove(tmp)
		return PixelAntwort{}, err
	}
	p.Datei = ziel
	return p, nil
}

// PixelAnteil ist der pixeldiff-Anteil von Frame 0 der ersten Animation (Vertrag); ohne Animation 0. Lässt sich die
// Basis nicht mehr rendern (fachlich abgelehnt), gilt das ganze Bild als geändert (1).
func PixelAnteil(ctx context.Context, r Renderer, root string, qa, qn Quelle) (float64, error) {
	animation, err := ErsteAnimation(qn.Spec)
	if err != nil || animation == "" {
		return 0, err
	}
	p, err := Overlay(ctx, r, root, qa, qn, animation, 0)
	var abgelehnt *Abgelehnt
	if errors.As(err, &abgelehnt) {
		return 1, nil
	}
	return p.Anteil, err
}

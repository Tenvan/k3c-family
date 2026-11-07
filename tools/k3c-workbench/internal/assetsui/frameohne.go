package assetsui

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"regexp"

	"k3c/tools/k3c-workbench/internal/review"
)

// ohneRenderer rendert einen Frame ohne die Lagen einzelner Teile (review.CLI; B-734).
type ohneRenderer interface {
	FrameOhne(ctx context.Context, spec, animation string, frame int, ohne []string, out string) (review.FrameAntwort, error)
}

var teilName = regexp.MustCompile(`^[a-z0-9][a-z0-9-]*$`)

// AssetsFrameOhne liefert einen Frame des Arbeitsstands, in dem die Teile ohne ausgeblendet sind (Data-URL mit Teil-Boxen).
// Ohne Cache auf der Platte: die Oberfläche merkt sich die Bilder selbst.
func (d Dienst) AssetsFrameOhne(id, anim string, frame int, ohne []string) (Frame, error) {
	r, ok := d.renderer().(ohneRenderer)
	if !ok {
		return Frame{}, fmt.Errorf("der Renderer kann keine Teile ausblenden")
	}
	for _, t := range ohne {
		if !teilName.MatchString(t) {
			return Frame{}, fmt.Errorf("Teil „%s“ ungültig", t) //nolint:staticcheck // Text der Oberfläche
		}
	}
	ctx, cancel := kontext()
	defer cancel()
	q, err := d.quelle(ctx, id, "")
	if err != nil {
		return Frame{}, err
	}
	if anim, err = animation(q, anim); err != nil {
		return Frame{}, err
	}
	if err := pruefeFrame(q, anim, frame); err != nil {
		return Frame{}, err
	}
	spec, err := q.SpecDatei(d.Root, d.renderer())
	if err != nil {
		return Frame{}, err
	}
	dir, err := os.MkdirTemp("", "k3c-ohne-")
	if err != nil {
		return Frame{}, err
	}
	defer func() { _ = os.RemoveAll(dir) }()
	out := filepath.Join(dir, "frame.png")
	renderPlatz <- struct{}{}
	a, err := r.FrameOhne(ctx, spec, anim, frame, ohne, out)
	<-renderPlatz
	if err != nil {
		return Frame{}, err
	}
	bild, err := dataURL(out)
	if a.Teile == nil {
		a.Teile = map[string][]int{}
	}
	return Frame{Bild: bild, Teile: a.Teile}, err
}

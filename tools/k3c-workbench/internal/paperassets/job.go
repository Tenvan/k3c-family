package paperassets

import (
	"context"
	"fmt"
	"strings"
	"sync"
	"time"
)

// AssetErgebnis ist das Ergebnis eines Assets im Render-Job; Uebersprungen nennt die erste verletzte Regel.
type AssetErgebnis struct {
	ID            string
	PNGs          int
	Dateien       []string
	Manifest      string
	Uebersprungen string
	Fehler        string
}

// Stand ist der Zustand eines Render-Jobs.
type Stand struct {
	ID         string
	N          int
	Erledigt   int
	Aktuell    string
	Fertig     bool
	Ergebnisse []AssetErgebnis
	Dauer      time.Duration
}

type renderJob struct {
	mu    sync.Mutex
	stand Stand
	start time.Time
	done  chan struct{}
}

// Jobs rendert Assets im Hintergrund (eigener Mechanismus, nicht task_*); IDs j-<n> fortlaufend je Prozess.
type Jobs struct {
	mu   sync.Mutex
	n    int
	jobs map[string]*renderJob
	// Render rendert ein Asset; Tests ersetzen es.
	Render func(ctx context.Context, cli CLI, a Asset) AssetErgebnis
}

// NewJobs liefert eine leere Job-Tabelle, die über die Render-CLI rendert.
func NewJobs() *Jobs { return &Jobs{jobs: map[string]*renderJob{}, Render: RenderEins} }

// RenderEins rendert ein Asset; eine ungültige Spec wird mit ihrer ersten Regel übersprungen.
func RenderEins(ctx context.Context, cli CLI, a Asset) AssetErgebnis {
	e := AssetErgebnis{ID: a.ID}
	defer sperre(cli.Root, a.ID)()
	r, err := cli.Render(ctx, a.Abs(cli.Root))
	if ab, ok := AlsAbgelehnt(err); ok {
		e.Uebersprungen = ab.Fehler[0].Regel
		return e
	}
	if err != nil {
		e.Fehler = err.Error()
		return e
	}
	e.Dateien = r.Dateien
	for _, d := range r.Dateien {
		if strings.HasSuffix(d, ".png") {
			e.PNGs++
		}
	}
	if len(r.Manifeste) > 0 {
		e.Manifest = r.Manifeste[0]
	}
	return e
}

// Start legt einen Job an und kehrt sofort zurück; gerendert wird nacheinander im Hintergrund.
func (j *Jobs) Start(cli CLI, assets []Asset) Stand {
	j.mu.Lock()
	j.n++
	job := &renderJob{stand: Stand{ID: fmt.Sprintf("j-%d", j.n), N: len(assets)}, start: time.Now(), done: make(chan struct{})}
	if len(assets) > 0 {
		job.stand.Aktuell = assets[0].ID
	}
	j.jobs[job.stand.ID] = job
	render := j.Render
	j.mu.Unlock()
	go func() {
		defer close(job.done)
		for i, a := range assets {
			job.mu.Lock()
			job.stand.Aktuell = assets[i].ID
			job.mu.Unlock()
			e := sicher(render, cli, a)
			job.mu.Lock()
			job.stand.Ergebnisse = append(job.stand.Ergebnisse, e)
			job.stand.Erledigt++
			job.mu.Unlock()
		}
		job.mu.Lock()
		job.stand.Fertig, job.stand.Aktuell, job.stand.Dauer = true, "", time.Since(job.start)
		job.mu.Unlock()
	}()
	return job.kopie()
}

// sicher rendert ein Asset; eine Panic wird zum Fehler des Assets statt den Prozess zu beenden.
func sicher(render func(context.Context, CLI, Asset) AssetErgebnis, cli CLI, a Asset) (e AssetErgebnis) {
	defer func() {
		if p := recover(); p != nil {
			e = AssetErgebnis{ID: a.ID, Fehler: fmt.Sprintf("Absturz: %v", p)}
		}
	}()
	return render(context.Background(), cli, a)
}

func (r *renderJob) kopie() Stand {
	r.mu.Lock()
	defer r.mu.Unlock()
	s := r.stand
	s.Ergebnisse = append([]AssetErgebnis(nil), r.stand.Ergebnisse...)
	if !s.Fertig {
		s.Dauer = time.Since(r.start)
	}
	return s
}

func (j *Jobs) finde(id string) (*renderJob, error) {
	j.mu.Lock()
	defer j.mu.Unlock()
	job, ok := j.jobs[id]
	if !ok {
		return nil, fmt.Errorf("Job „%s“ unbekannt", id) //nolint:staticcheck // Satz für die Antwort
	}
	return job, nil
}

// Status liefert den aktuellen Stand eines Jobs.
func (j *Jobs) Status(id string) (Stand, error) {
	job, err := j.finde(id)
	if err != nil {
		return Stand{}, err
	}
	return job.kopie(), nil
}

// Wait wartet bis zum Ende des Jobs; endet ctx vorher, bricht nur das Warten ab.
func (j *Jobs) Wait(ctx context.Context, id string) (Stand, error) {
	job, err := j.finde(id)
	if err != nil {
		return Stand{}, err
	}
	select {
	case <-job.done:
		return job.kopie(), nil
	case <-ctx.Done():
		return job.kopie(), ctx.Err()
	}
}

package paperassets

import (
	"context"
	"errors"
	"strings"
	"testing"
	"time"
)

const bannerUngueltig = `{"id":"banner","rig":"requisite","version":1,"beschreibung":"Banner","teile":[{"id":"tuch","pivot":[0,0],` +
	`"lagen":[{"art":"flaeche","form":"M0 0L10 0L10 10Z","material":"scharlach"}]}],"ausgabe":{"hoehe_px":64,"rahmen":[0,0,10,10]}}`

// MC1/AC-05: Start kehrt vor dem ersten Render zurück; Status zeigt den Stand, Wait das Ergebnis.
func TestJobAlle(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	assets, _ := Assets(root)
	frei := make(chan struct{})
	jobs := NewJobs()
	jobs.Render = func(_ context.Context, _ CLI, a Asset) AssetErgebnis {
		<-frei
		return AssetErgebnis{ID: a.ID, PNGs: 2, Manifest: a.ID + "/manifest.json"}
	}
	st := jobs.Start(cli, assets)
	if st.ID != "j-1" || st.N != 3 || st.Fertig {
		t.Fatalf("Start: %+v", st)
	}
	if st, _ = jobs.Status("j-1"); st.Erledigt != 0 || st.Aktuell != "mauer" || st.Fertig {
		t.Errorf("Status vor Render: %+v", st)
	}
	close(frei)
	st, err := jobs.Wait(context.Background(), "j-1")
	if err != nil || !st.Fertig || st.Erledigt != 3 || len(st.Ergebnisse) != 3 || st.Ergebnisse[1].ID != "monarch" {
		t.Fatalf("Wait: %+v %v", st, err)
	}
	if st2 := jobs.Start(cli, assets); st2.ID != "j-2" {
		t.Errorf("zweiter Job: %s", st2.ID)
	}
}

// Ein Job mit der echten Attrappe überspringt ungültige Specs mit ihrer Regel.
func TestJobUeberspringtUngueltige(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	schreibe(t, root, "public/grafik/requisiten/banner.paper.json", bannerUngueltig)
	assets, _ := Assets(root)
	jobs := NewJobs()
	st, err := jobs.Wait(context.Background(), jobs.Start(cli, assets).ID)
	if err != nil || st.Erledigt != 4 {
		t.Fatalf("Wait: %+v %v", st, err)
	}
	for _, e := range st.Ergebnisse {
		switch e.ID {
		case "banner":
			if e.Uebersprungen != "material-unbekannt" {
				t.Errorf("banner: %+v", e)
			}
		default:
			if e.PNGs == 0 || e.Manifest == "" || e.Uebersprungen != "" || e.Fehler != "" {
				t.Errorf("%s: %+v", e.ID, e)
			}
		}
	}
}

func TestJobUnbekannt(t *testing.T) {
	t.Parallel()
	jobs := NewJobs()
	if _, err := jobs.Status("j-99"); err == nil || err.Error() != "Job „j-99“ unbekannt" {
		t.Errorf("Status: %v", err)
	}
	if _, err := jobs.Wait(context.Background(), "j-99"); err == nil {
		t.Error("Wait ohne Fehler")
	}
}

// Ein abgebrochenes Warten beendet nur das Warten, der Job läuft weiter.
func TestJobWaitAbbruch(t *testing.T) {
	t.Parallel()
	frei := make(chan struct{})
	jobs := NewJobs()
	jobs.Render = func(_ context.Context, _ CLI, a Asset) AssetErgebnis { <-frei; return AssetErgebnis{ID: a.ID} }
	st := jobs.Start(CLI{}, []Asset{{ID: "a"}})
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Millisecond)
	defer cancel()
	if _, err := jobs.Wait(ctx, st.ID); !errors.Is(err, context.DeadlineExceeded) {
		t.Errorf("Abbruch: %v", err)
	}
	close(frei)
	if st, err := jobs.Wait(context.Background(), st.ID); err != nil || !st.Fertig {
		t.Errorf("weiter: %+v %v", st, err)
	}
	if !strings.HasPrefix(st.ID, "j-") {
		t.Errorf("ID: %s", st.ID)
	}
}

// Review MC1.1: eine Panic beim Rendern beendet nicht den Prozess, sondern wird Fehler des Assets.
func TestJobPanic(t *testing.T) {
	t.Parallel()
	jobs := NewJobs()
	jobs.Render = func(context.Context, CLI, Asset) AssetErgebnis { panic("kaputt") }
	st, err := jobs.Wait(context.Background(), jobs.Start(CLI{}, []Asset{{ID: "a"}, {ID: "b"}}).ID)
	if err != nil || !st.Fertig || len(st.Ergebnisse) != 2 || !strings.Contains(st.Ergebnisse[0].Fehler, "kaputt") {
		t.Errorf("Panic: %+v %v", st, err)
	}
}

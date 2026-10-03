package taskrun

import (
	"context"
	"errors"
	"sync"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/proc"
)

// fake ersetzt `task <name>` durch einen Go-Aufruf: "go version" endet mit 0, "go nonsense" mit Exit 2.
func newRunner(argv []string, out *[]string, ended chan Run) *Runner {
	var mu sync.Mutex
	return New(Options{
		Command: func(ctx context.Context, _ []string) *proc.Cmd { return proc.Command(ctx, argv) },
		Out: func(_, _, text string) {
			mu.Lock()
			*out = append(*out, text)
			mu.Unlock()
		},
		OnChange: func(r Run) {
			if r.State != Running {
				ended <- r
			}
		},
	})
}

func TestStart_ErfolgUndFehler(t *testing.T) {
	for _, c := range []struct {
		argv []string
		want State
	}{{[]string{"go", "version"}, Succeeded}, {[]string{"go", "nonsense"}, Failed}} {
		var out []string
		ended := make(chan Run, 1)
		r := newRunner(c.argv, &out, ended)
		if _, err := r.Start("x", nil); err != nil {
			t.Fatal(err)
		}
		select {
		case run := <-ended:
			if run.State != c.want || (c.want == Failed && run.ExitCode == 0) {
				t.Fatalf("%v: %+v", c.argv, run)
			}
		case <-time.After(20 * time.Second):
			t.Fatal("Lauf endet nicht")
		}
	}
}

func TestStart_LehntArgumenteMitMetazeichenAb(t *testing.T) {
	r := New(Options{})
	if _, err := r.Start("x", []string{"a;b"}); err == nil {
		t.Fatal("erwartet: abgelehnt")
	}
	if _, err := r.Stop(context.Background(), "x"); !errors.Is(err, ErrNotRunning) {
		t.Fatalf("Stop ohne Lauf: %v", err)
	}
}

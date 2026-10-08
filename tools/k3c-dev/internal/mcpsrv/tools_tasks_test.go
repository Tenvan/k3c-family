package mcpsrv

import (
	"errors"
	"strings"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/taskcat"
	"k3c/tools/k3c-dev/internal/taskgate"
	"k3c/tools/k3c-dev/internal/taskrun"
)

// fakeTasks ist ein Tasks-Host mit zwei Tasks; nur "check" ist freigegeben.
type fakeTasks struct{ started []string }

func (f *fakeTasks) TaskList() ([]taskcat.Task, error) {
	return []taskcat.Task{
		{Name: "check", Namespace: "Workspace", Leaf: "check", Desc: "Lint und Tests"},
		{Name: "golden:update", Namespace: "golden", Leaf: "update", Desc: "Golden neu"},
	}, nil
}

func (f *fakeTasks) TaskStart(name string, args []string) (taskrun.Run, error) {
	f.started = append(f.started, name+" "+strings.Join(args, " "))
	return taskrun.Run{Name: name, State: taskrun.Running, PID: 7, Args: args, StartedAt: time.Now()}, nil
}

func (f *fakeTasks) TaskStop(name string) (taskrun.Run, error) {
	return taskrun.Run{}, errors.New("task läuft nicht")
}

func (f *fakeTasks) TaskRuns() []taskrun.Run {
	return []taskrun.Run{{Name: "check", State: taskrun.Succeeded, DurationMs: 4200}}
}

func (f *fakeTasks) TaskState(name string) taskgate.State {
	if name == "check" {
		return taskgate.Open
	}
	return taskgate.Closed
}

func TestTaskTools(t *testing.T) {
	host := &fakeTasks{}
	s := New(Config{Version: "test", Root: t.TempDir(), Tasks: host})
	cs := connect(t, s)
	if text, _ := callText(t, cs, "task_list", nil); !strings.Contains(text, "  * check · Lint und Tests") || !strings.Contains(text, "    golden:update") {
		t.Errorf("task_list: %q", text)
	}
	if text, isErr := callText(t, cs, "task_start", map[string]any{"task": "golden:update"}); !isErr || !strings.Contains(text, "nicht freigegeben") {
		t.Errorf("gesperrt: %q", text)
	}
	if text, isErr := callText(t, cs, "task_start", map[string]any{"task": "check", "args": "a;b"}); !isErr || len(host.started) != 0 {
		t.Errorf("Metazeichen: %q", text)
	}
	if text, isErr := callText(t, cs, "task_start", map[string]any{"task": "check", "args": "-run Foo"}); isErr ||
		!strings.HasPrefix(text, "check · running · PID 7") || host.started[0] != "check -run Foo" {
		t.Errorf("Start: %q %v", text, host.started)
	}
	if text, _ := callText(t, cs, "task_status", nil); !strings.HasPrefix(text, "check · succeeded · Exit 0") {
		t.Errorf("task_status: %q", text)
	}
	s.console.Add("task:check", "stdout", "fertig")
	if text, _ := callText(t, cs, "task_output", map[string]any{"task": "check"}); !strings.HasSuffix(text, "\nfertig") {
		t.Errorf("task_output: %q", text)
	}
}

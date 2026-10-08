package mcpsrv

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-dev/internal/taskcat"
	"k3c/tools/k3c-dev/internal/taskgate"
	"k3c/tools/k3c-dev/internal/taskrun"
)

// TaskHost ist die Tasks-Seite für die MCP-Tools (Workbench-Spec § 4): Katalog, Läufe und Freigabe-Schloss.
type TaskHost interface {
	TaskList() ([]taskcat.Task, error)
	TaskStart(name string, args []string) (taskrun.Run, error)
	TaskStop(name string) (taskrun.Run, error)
	TaskRuns() []taskrun.Run
	TaskState(name string) taskgate.State
}

type taskListIn struct {
	Namespace string `json:"namespace,omitempty" jsonschema:"nur dieser Namensraum, z. B. golden"`
	Query     string `json:"query,omitempty" jsonschema:"Freitext auf Name und Beschreibung"`
}

type taskStartIn struct {
	Task string `json:"task" jsonschema:"Name aus task_list, z. B. golden:update"`
	Args string `json:"args,omitempty" jsonschema:"Zusatzargumente, angehängt als task <name> -- <args>; keine Shell-Metazeichen"`
}

type taskIn struct {
	Task string `json:"task" jsonschema:"Name aus task_list"`
}

type taskStatusIn struct {
	Task string `json:"task,omitempty" jsonschema:"nur dieser Task; leer: alle Läufe dieser Sitzung"`
}

type taskOutputIn struct {
	Task  string `json:"task" jsonschema:"Name aus task_list"`
	Limit int    `json:"limit,omitempty" jsonschema:"Zahl der letzten Zeilen, Standard 50, höchstens 500"`
	Since int64  `json:"since,omitempty" jsonschema:"nur Zeilen nach dieser Nummer"`
}

func registerTasks(s *Server) {
	no, yes, closed := false, true, false
	add(s, &mcp.Tool{
		Name: "task_list",
		Description: "Taskfile-Baum nach Namensraum; freigegebene Tasks mit * (Schloss offen, task_start erlaubt). " +
			"Nutze es bei: welcher Task existiert und ob er startbar ist. Statt: task --list-all.",
		Annotations: readOnly(),
	}, s.taskList)
	add(s, &mcp.Tool{
		Name: "task_start",
		Description: "Startet einen freigegebenen Task und kehrt sofort zurück; Freigabe laut Schloss der Tasks-Seite, nicht laut Name. " +
			"Nutze es bei: Werkzeug-Tasks ohne eigenes Tool. Statt: task <name> in der Shell. Auf das Ende warten: check_run.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &no, OpenWorldHint: &closed},
	}, s.taskStart)
	add(s, &mcp.Tool{
		Name: "task_stop",
		Description: "Stoppt einen laufenden Task samt Prozessbaum. Nutze es bei: Task hängt oder läuft ungewollt. Statt: taskkill.",
		Annotations: &mcp.ToolAnnotations{DestructiveHint: &yes, OpenWorldHint: &closed},
	}, s.taskStop)
	add(s, &mcp.Tool{
		Name: "task_status",
		Description: "Läufe dieser Sitzung mit Zustand, Exit-Code, Dauer und Argumenten. " +
			"Nutze es bei: ist mein Task fertig. Statt: Prozessliste.",
		Annotations: readOnly(),
	}, s.taskStatus)
	add(s, &mcp.Tool{
		Name: "task_output",
		Description: "Konsole eines Task-Laufs (wie die rechte Hälfte der Tasks-Seite), since für neuere Zeilen. " +
			"Nutze es bei: Ausgabe eines laufenden oder beendeten Tasks. Statt: Ausgabe in der Shell mitlesen.",
		Annotations: readOnly(),
	}, s.taskOutput)
}

func (s *Server) taskHost(ctx context.Context) (TaskHost, error) {
	if s.cfg.Tasks == nil {
		return nil, errors.New("keine Tasks: k3c-dev läuft ohne Tasks-Seite")
	}
	if ws := s.ws(ctx); ws.name != "" {
		return nil, fmt.Errorf("task_* bedient nur die Repo-Wurzel, nicht den Worktree %s; im Worktree check_run nutzen", ws.name)
	}
	return s.cfg.Tasks, nil
}

func (s *Server) taskList(ctx context.Context, in taskListIn) (string, error) {
	host, err := s.taskHost(ctx)
	if err != nil {
		return "", err
	}
	tasks, err := host.TaskList()
	if err != nil {
		return "", err
	}
	tasks = taskcat.Filter(tasks, in.Query)
	var out []string
	for _, ns := range taskcat.Group(tasks) {
		if in.Namespace != "" && !strings.EqualFold(ns.Name, in.Namespace) {
			continue
		}
		out = append(out, ns.Name)
		for _, t := range ns.Tasks {
			mark := " "
			if host.TaskState(t.Name).Allowed() {
				mark = "*"
			}
			out = append(out, fmt.Sprintf("  %s %s · %s", mark, t.Name, t.Desc))
		}
	}
	if len(out) == 0 {
		return "keine Tasks gefunden", nil
	}
	return strings.Join(out, "\n"), nil
}

func (s *Server) taskStart(ctx context.Context, in taskStartIn) (string, error) {
	host, err := s.taskHost(ctx)
	if err != nil {
		return "", err
	}
	if st := host.TaskState(in.Task); !st.Allowed() {
		return "", fmt.Errorf("task %q ist nicht freigegeben (Schloss %s); 🧑 öffnet es auf der Tasks-Seite", in.Task, st)
	}
	args := strings.Fields(in.Args)
	if err := taskrun.ValidateArgs(args); err != nil {
		return "", err
	}
	run, err := host.TaskStart(in.Task, args)
	if err != nil {
		return "", err
	}
	return runLine(run) + " · Ausgabe: task_output " + in.Task, nil
}

func (s *Server) taskStop(ctx context.Context, in taskIn) (string, error) {
	host, err := s.taskHost(ctx)
	if err != nil {
		return "", err
	}
	run, err := host.TaskStop(in.Task)
	if err != nil {
		return "", err
	}
	return runLine(run), nil
}

func (s *Server) taskStatus(ctx context.Context, in taskStatusIn) (string, error) {
	host, err := s.taskHost(ctx)
	if err != nil {
		return "", err
	}
	var out []string
	for _, r := range host.TaskRuns() {
		if in.Task == "" || r.Name == in.Task {
			out = append(out, runLine(r))
		}
	}
	if len(out) == 0 {
		return "keine Läufe in dieser Sitzung", nil
	}
	return strings.Join(out, "\n"), nil
}

func (s *Server) taskOutput(ctx context.Context, in taskOutputIn) (string, error) {
	if _, err := s.taskHost(ctx); err != nil {
		return "", err
	}
	return s.consoleTail(ctx, tailIn{Source: "task:" + in.Task, Lines: in.Limit, Since: in.Since})
}

// runLine ist ein Lauf in einer Zeile, z. B. "golden:update · succeeded · Exit 0 · 4,2 s · -- --dry".
func runLine(r taskrun.Run) string {
	parts := []string{r.Name, string(r.State)}
	if r.State == taskrun.Running {
		parts = append(parts, fmt.Sprintf("PID %d", r.PID), "seit "+r.StartedAt.Local().Format(time.TimeOnly))
	} else {
		parts = append(parts, fmt.Sprintf("Exit %d", r.ExitCode), formatMs(float64(r.DurationMs)))
	}
	if r.Reason != "" {
		parts = append(parts, r.Reason)
	}
	if len(r.Args) > 0 {
		parts = append(parts, "-- "+strings.Join(r.Args, " "))
	}
	return strings.Join(parts, " · ")
}

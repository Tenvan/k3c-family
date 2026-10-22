package taskcat

import "testing"

const sample = `{"tasks":[
 {"name":"check","desc":"alles prüfen","aliases":null,"location":{"line":3,"taskfile":"/r/Taskfile.yml"}},
 {"name":"dev:build","desc":"Frontend bauen","aliases":["db"],"location":{"line":9,"taskfile":"/elsewhere/T.yml"}},
 {"name":"dev:test","desc":"","location":{"line":1,"taskfile":"/r/Taskfile.yml"}}]}`

func TestParseGroupFilter(t *testing.T) {
	tasks, err := Parse([]byte(sample), "/r")
	if err != nil {
		t.Fatal(err)
	}
	if tasks[0].Namespace != Workspace || tasks[1].Namespace != "dev" || tasks[1].Leaf != "build" {
		t.Fatalf("split: %+v", tasks)
	}
	if tasks[0].Aliases == nil || tasks[0].File != "Taskfile.yml" {
		t.Fatalf("aliases/file: %+v", tasks[0])
	}
	g := Group(tasks)
	if len(g) != 2 || g[0].Name != Workspace || g[1].Tasks[0].Name != "dev:build" {
		t.Fatalf("group: %+v", g)
	}
	if got := Filter(tasks, " FRONTEND "); len(got) != 1 || got[0].Name != "dev:build" {
		t.Fatalf("filter: %+v", got)
	}
}

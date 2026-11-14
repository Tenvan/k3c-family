package planning

import (
	"flag"
	"os"
	"testing"
)

// gitParallel: so viele Tests laufen gleichzeitig, wenn -parallel nicht gesetzt ist. Die Tests mit echten Git-Repos starten
// je Test bis zu zehn git-Prozesse; mit GOMAXPROCS parallel (22 auf dem Entwickler-PC) warten sie aufeinander und ein
// einzelner Test zeigt dann über 3 s, obwohl er allein 2 s braucht. Mit 4 bleibt das Paket bei rund 7 s (B-830).
const gitParallel = "4"

func TestMain(m *testing.M) {
	flag.Parse()
	set := false
	flag.Visit(func(f *flag.Flag) { set = set || f.Name == "test.parallel" })
	if !set {
		_ = flag.Set("test.parallel", gitParallel)
	}
	os.Exit(m.Run())
}

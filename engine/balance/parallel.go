package balance

import (
	"runtime"
	"sync"
)

// Läufe parallel (B-634, BS1.2): Jeder Lauf hat seine eigene Insel und liest nur gemeinsame Tabellen, daher rechnen
// die Szenarien eines Berichts gleichzeitig. Die Ergebnisse stehen in der Reihenfolge der Szenarien, das Ergebnis
// jedes Laufs ist dasselbe wie nacheinander (deterministisch je Szenario).

// workers ist die Zahl gleichzeitiger Läufe; Tests setzen sie auf 1, um die Reihenfolge zu vergleichen.
var workers = runtime.NumCPU()

// runAll rechnet alle Szenarien mit höchstens workers Läufen gleichzeitig; progress (darf nil sein) erfährt nach jedem
// fertigen Lauf die Zahl der fertigen und aller Läufe.
func runAll(all []Scenario, threshold int, progress func(done, total int)) []Result {
	results := make([]Result, len(all))
	jobs := make(chan int)
	var mu sync.Mutex
	var wg sync.WaitGroup
	done := 0
	for range max(1, min(workers, len(all))) {
		wg.Go(func() {
			for i := range jobs {
				results[i] = runOne(all[i], threshold)
				if progress != nil {
					mu.Lock()
					done++
					progress(done, len(all))
					mu.Unlock()
				}
			}
		})
	}
	for i := range all {
		jobs <- i
	}
	close(jobs)
	wg.Wait()
	return results
}

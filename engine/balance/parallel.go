package balance

import (
	"fmt"
	"runtime"
	"runtime/debug"
	"sync"
)

// Läufe parallel (B-634, BS1.2): Jeder Lauf hat seine eigene Insel und liest nur gemeinsame Tabellen, daher rechnen
// die Szenarien eines Berichts gleichzeitig. Die Ergebnisse stehen in der Reihenfolge der Szenarien, das Ergebnis
// jedes Laufs ist dasselbe wie nacheinander (deterministisch je Szenario).

// workers ist die Zahl gleichzeitiger Läufe; Tests setzen sie auf 1, um die Reihenfolge zu vergleichen.
var workers = runtime.NumCPU()

// Crash ist die Panik eines Laufs oder von progress, die runAll beim Aufrufer erneut auslöst; Stack ist der Stack der
// ursprünglichen Panik (der Ort bleibt sichtbar, auch wenn der Aufrufer sie fängt).
type Crash struct {
	Value any
	Stack []byte
}

func (c *Crash) Error() string { return fmt.Sprintf("%v\n%s", c.Value, c.Stack) }

// runAll rechnet alle Szenarien mit höchstens workers Läufen gleichzeitig; progress (darf nil sein) erfährt nach jedem
// fertigen Lauf die Zahl der fertigen und aller Läufe. Eine Panik in einem Lauf oder in progress (B-924: die Workbench
// beendet so einen abgebrochenen Lauf) lässt die übrigen Läufe aus und kommt in der aufrufenden Goroutine an, damit
// der Aufrufer sie fangen kann; in der Worker-Goroutine risse sie den Prozess mit.
func runAll(all []Scenario, threshold int, progress func(done, total int)) []Result {
	results := make([]Result, len(all))
	jobs := make(chan int)
	var mu sync.Mutex
	var wg sync.WaitGroup
	var crash *Crash
	done := 0
	report := func() {
		mu.Lock()
		defer mu.Unlock()
		done++
		progress(done, len(all))
	}
	one := func(i int) {
		defer func() {
			if r := recover(); r != nil {
				mu.Lock()
				defer mu.Unlock()
				if crash == nil {
					crash = &Crash{Value: r, Stack: debug.Stack()}
				}
			}
		}()
		results[i] = runOne(all[i], threshold)
		if progress != nil {
			report()
		}
	}
	for range max(1, min(workers, len(all))) {
		wg.Go(func() {
			for i := range jobs {
				mu.Lock()
				skip := crash != nil
				mu.Unlock()
				if !skip {
					one(i)
				}
			}
		})
	}
	for i := range all {
		jobs <- i
	}
	close(jobs)
	wg.Wait()
	if crash != nil {
		panic(crash)
	}
	return results
}

// tickProgress hängt tick (darf nil sein) vor progress (darf nil sein); beide nil ergibt nil.
func tickProgress(tick func(), progress func(done, total int)) func(done, total int) {
	if tick == nil {
		return progress
	}
	return func(done, total int) {
		tick()
		if progress != nil {
			progress(done, total)
		}
	}
}

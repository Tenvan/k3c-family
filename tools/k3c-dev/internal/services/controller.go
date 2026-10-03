package services

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"slices"
	"sync"
	"sync/atomic"
	"time"

	"k3c/tools/k3c-dev/internal/applog"
	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/proc"
)

// State ist der Zustand eines Dienstes, mit den Namen aus B-067.
type State string

// Die Zustände eines Dienstes.
const (
	Stopped  State = "gestoppt"
	Starting State = "startet"
	Running  State = "läuft"
	Adopted  State = "übernommen"
	Stopping State = "stoppt"
	Failed   State = "fehlgeschlagen"
)

// Status ist der Zustand eines Dienstes für MCP und Oberfläche.
type Status struct {
	Name      string    `json:"name"`
	Desc      string    `json:"description"`
	Tags      []string  `json:"tags,omitempty"`
	Port      int       `json:"port"`
	Health    string    `json:"health"`
	Log       string    `json:"log"` // Name unter logs/ (leer: kein Log); die Oberfläche zeigt dann keinen Log-Kasten
	State     State     `json:"state"`
	PID       int       `json:"pid"`
	StartedAt time.Time `json:"startedAt"`
	Restarts  int       `json:"restarts"`
	LastError string    `json:"lastError"`
	CPU       float64   `json:"cpu"`    // Prozent, alle 2 s gemessen (Monitor)
	Memory    uint64    `json:"memory"` // RSS in Bytes
	// Seq zählt je Dienst jede Änderung. OnChange-Rückrufe kommen ungeordnet an (Messung und Befehl laufen
	// parallel); wer sie weitergibt, verwirft einen Status mit kleinerer Seq als dem zuletzt gesehenen.
	Seq uint64 `json:"seq"`
}

// Process ist ein laufender Dienst-Prozess (Test-Naht; Produktion: procProcess in runtime.go).
type Process interface {
	PID() int
	Wait() error
	Kill() error
}

// Starter startet einen Dienst; out bekommt jede Ausgabezeile mit ihrem Strom.
type Starter func(svc Service, root string, out func(stream, text string)) (Process, error)

// Checker prüft, ob ein Dienst gesund ist.
type Checker func(ctx context.Context, svc Service) error

// Options sind die Teile und Zeiten eines Controllers; leere Felder bekommen die Vorgaben aus B-067.
type Options struct {
	Root          string
	Console       *console.Store
	Log           *slog.Logger
	OnChange      func(Status) // nur melden: darf den Controller nicht aufrufen (läuft teils unter der Befehlssperre)
	Start         Starter
	Check         Checker
	Listen        Listener
	Sample        Sampler
	KillPID       func(pid int) error // übernommene Prozesse (ohne Job)
	Now           func() time.Time
	StartTimeout  time.Duration // 60 s bis gesund
	StartPoll     time.Duration // 1 s zwischen zwei Prüfungen beim Start
	WatchEvery    time.Duration // 10 s zwischen zwei Prüfungen im Lauf
	FailLimit     int           // 3 Fehlschläge in Folge
	RestartLimit  int           // 3 Neustarts …
	RestartWindow time.Duration // … je 10 min
	StopTimeout   time.Duration // 10 s Warten auf das Ende nach Kill
	FilesEvery    time.Duration // 1 s zwischen zwei Prüfungen der beobachteten Dateien (watch)
	FilesQuiet    time.Duration // 500 ms Ruhe, bevor eine Änderung zum Neustart führt
}

func (o *Options) defaults() {
	if o.Console == nil {
		o.Console = console.New(console.DefaultCapacity, nil)
	}
	if o.Log == nil {
		o.Log = applog.Discard()
	}
	if o.OnChange == nil {
		o.OnChange = func(Status) {}
	}
	if o.Start == nil {
		o.Start = ProcStarter
	}
	if o.Check == nil {
		o.Check = HealthCheck
	}
	if o.Listen == nil {
		o.Listen = PortListener
	}
	if o.Sample == nil {
		o.Sample = NewSampler()
	}
	if o.KillPID == nil {
		o.KillPID = proc.KillTree
	}
	if o.Now == nil {
		o.Now = time.Now
	}
	setDur(&o.StartTimeout, 60*time.Second)
	setDur(&o.StartPoll, time.Second)
	setDur(&o.WatchEvery, 10*time.Second)
	setDur(&o.RestartWindow, 10*time.Minute)
	setDur(&o.StopTimeout, 10*time.Second)
	setDur(&o.FilesEvery, time.Second)
	setDur(&o.FilesQuiet, 500*time.Millisecond)
	if o.FailLimit <= 0 {
		o.FailLimit = 3
	}
	if o.RestartLimit <= 0 {
		o.RestartLimit = 3
	}
}

func setDur(d *time.Duration, v time.Duration) {
	if *d <= 0 {
		*d = v
	}
}

// unit ist ein Dienst im Controller. cmd serialisiert Befehle (MCP, Oberfläche, Überwachung); mu schützt die Felder.
type unit struct {
	svc      Service
	cmd      sync.Mutex
	mu       sync.Mutex
	st       Status
	run      *run
	restarts []time.Time
	unwatch  context.CancelFunc // Überwachung eines übernommenen Dienstes
	unfiles  context.CancelFunc // Beobachtung der Dateien (Watch-Modus)
}

// run ist ein eigener laufender Prozess mit seiner Überwachung.
type run struct {
	proc Process
	done chan struct{} // geschlossen, wenn der Prozess geendet hat
	err  error         // Ergebnis von Wait, gültig nach done
	stop context.CancelFunc
	// killed ist gesetzt, sobald k3c-dev den Prozess beenden will (Log: Ende durch k3c-dev statt von selbst).
	killed atomic.Bool
}

// Controller ist der eine Steuerpunkt aller Dienste.
type Controller struct {
	opts  Options
	units []*unit // Reihenfolge der Konfiguration
}

// New legt einen Controller an; kein Dienst läuft danach.
func New(list []Service, opts Options) *Controller {
	opts.defaults()
	c := &Controller{opts: opts}
	for _, s := range list {
		c.units = append(c.units, &unit{svc: s, st: Status{Name: s.Name, Desc: s.Description, Tags: s.Tags, Port: s.Port, Health: s.HealthURL(), Log: s.Log, State: Stopped}})
	}
	return c
}

// Statuses liefert alle Dienste in der Reihenfolge der Konfiguration.
func (c *Controller) Statuses() []Status {
	out := make([]Status, len(c.units))
	for i, u := range c.units {
		out[i] = u.status()
	}
	return out
}

// Names liefert die Dienstnamen in der Reihenfolge der Konfiguration.
func (c *Controller) Names() []string {
	out := make([]string, len(c.units))
	for i, u := range c.units {
		out[i] = u.svc.Name
	}
	return out
}

func (c *Controller) unit(name string) (*unit, error) {
	for _, u := range c.units {
		if u.svc.Name == name {
			return u, nil
		}
	}
	return nil, fmt.Errorf("unbekannter Dienst %q; gültig: %v", name, c.Names())
}

func (u *unit) status() Status {
	u.mu.Lock()
	defer u.mu.Unlock()
	return u.st
}

// set ändert den Zustand, schreibt den Wechsel ins Log und meldet ihn.
func (c *Controller) set(u *unit, change func(*Status)) Status {
	u.mu.Lock()
	before := u.st.State
	change(&u.st)
	u.st.Seq++
	st := u.st
	u.mu.Unlock()
	if st.State != before {
		c.opts.Log.Info("dienst "+st.Name+": "+string(st.State), "ns", "svc", "pid", st.PID, "grund", st.LastError)
	}
	c.opts.OnChange(st)
	return st
}

func (c *Controller) fail(u *unit, reason string) Status {
	c.opts.Log.Warn("dienst "+u.svc.Name+" fehlgeschlagen: "+reason, "ns", "svc")
	return c.set(u, func(s *Status) { s.State, s.LastError, s.PID, s.CPU, s.Memory = Failed, reason, 0, 0, 0 })
}

// Start startet einen Dienst und wartet, bis er gesund ist.
func (c *Controller) Start(ctx context.Context, name string) (Status, error) {
	u, err := c.unit(name)
	if err != nil {
		return Status{}, err
	}
	c.watchFiles(u) // ab jetzt: bei Änderungen neu starten, auch wenn dieser Start scheitert (Build-Fehler)
	u.cmd.Lock()
	defer u.cmd.Unlock()
	return c.start(ctx, u)
}

// Stop beendet einen Dienst samt Prozessbaum; einen übernommenen nur mit force.
func (c *Controller) Stop(ctx context.Context, name string, force bool) (Status, error) {
	u, err := c.unit(name)
	if err != nil {
		return Status{}, err
	}
	c.unwatchFiles(u) // zuerst, damit ein laufender Neustart abbricht und kein neuer folgt
	u.cmd.Lock()
	defer u.cmd.Unlock()
	if u.status().State == Adopted {
		return c.stopAdopted(u, force)
	}
	return c.stop(u)
}

// Restart ist Stop und Start unter einer Sperre.
func (c *Controller) Restart(ctx context.Context, name string) (Status, error) {
	u, err := c.unit(name)
	if err != nil {
		return Status{}, err
	}
	u.cmd.Lock()
	defer u.cmd.Unlock()
	if st := u.status(); st.State == Adopted {
		return st, fmt.Errorf("%s ist übernommen; erst mit force stoppen, dann starten", name)
	}
	if st, err := c.stop(u); err != nil {
		return st, err
	}
	return c.start(ctx, u)
}

// StartAll startet alle Dienste parallel, die nicht schon laufen, starten oder übernommen sind.
func (c *Controller) StartAll(ctx context.Context) error {
	errs := make([]error, len(c.units))
	var wg sync.WaitGroup
	for i, u := range c.units {
		if st := u.status().State; st == Running || st == Starting || st == Adopted {
			continue
		}
		wg.Go(func() { _, errs[i] = c.Start(ctx, u.svc.Name) })
	}
	wg.Wait()
	return errors.Join(errs...)
}

// StopAll stoppt alle eigenen Dienste in umgekehrter Reihenfolge der Konfiguration; übernommene bleiben.
func (c *Controller) StopAll(ctx context.Context) {
	for _, u := range slices.Backward(c.units) {
		_, _ = c.Stop(ctx, u.svc.Name, false)
	}
}

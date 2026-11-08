package simexe

import (
	"context"
	"encoding/json"
	"runtime"
)

// Pool sind mehrere Prozesse `k3c-sim bot` für ein Profil: Jede Anfrage nimmt einen freien, so entscheiden mehrere
// Bot-Geräte nebeneinander wie früher im eigenen Goroutine (höchstens so viele Prozesse wie Kerne).
type Pool struct {
	free chan *Bot
	all  []*Bot
}

// StartPool startet min(n, Kerne) Entscheider, mindestens einen; ctx beendet alle.
func StartPool(ctx context.Context, root, name string, n int) (*Pool, error) {
	n = max(1, min(n, runtime.NumCPU()))
	p := &Pool{free: make(chan *Bot, n)}
	for range n {
		b, err := StartBot(ctx, root, name)
		if err != nil {
			p.Close()
			return nil, err
		}
		p.all = append(p.all, b)
		p.free <- b
	}
	return p, nil
}

// Decide wie Bot.Decide auf einem freien Prozess.
func (p *Pool) Decide(state json.RawMessage, players []int) ([]Command, error) {
	b := <-p.free
	defer func() { p.free <- b }()
	return b.Decide(state, players)
}

// Close beendet alle Prozesse.
func (p *Pool) Close() {
	for _, b := range p.all {
		b.Close()
	}
}

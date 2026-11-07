package core

import (
	"runtime"
	"unsafe"
)

// CoinsCall ist der Aufruf-Block von k3c_collect_coins (Spike RS1.6); das Layout entspricht `#[repr(C)] CoinsCall`
// in rust/k3c-core/src/coins.rs Feld für Feld (Go füllt Zeiger auf seine Felder).
type CoinsCall struct {
	NCoins           uintptr
	CoinX            *float64
	CoinBlockedID    *int32 // -1: nicht gesperrt
	CoinBlockedUntil *float64
	NPlayers         uintptr
	PlayerX          *float64
	PlayerID         *int32
	PlayerGold       *int32 // wird erhöht
	PlayerAlive      *uint8
	Time             float64
	Reach            float64
	Purse            int32
	OutTaker         *int32 // je Münze: Index des Nehmers oder -1
}

// CollectCoins ruft k3c_collect_coins. Die Felder von c müssen bis zur Rückkehr gültig sein.
func (l *Lib) CollectCoins() (func(c *CoinsCall), error) {
	p, err := l.Proc("k3c_collect_coins")
	if err != nil {
		return nil, err
	}
	return func(c *CoinsCall) {
		escape(unsafe.Pointer(c))
		p(uintptr(unsafe.Pointer(c)))
		runtime.KeepAlive(c)
	}, nil
}

package game

import (
	"errors"

	"k3c/native/k3c-game/view"
)

// errOffline meldet, dass die Simulation noch nicht eingebunden ist.
var errOffline = errors.New("Simulation noch nicht eingebunden (NG1.2)")

// Offline ist die Session, bis local (NG1.2) sie ersetzt: kein Spiel, keine Spielstände.
type Offline struct{}

var _ view.Session = Offline{}

func (Offline) New(view.NewGame) error          { return errOffline }
func (Offline) Continue() error                 { return errOffline }
func (Offline) Load(string) error               { return errOffline }
func (Offline) List() ([]view.SaveInfo, error)  { return nil, nil }
func (Offline) Delete(string) error             { return errOffline }
func (Offline) Save() error                     { return errOffline }
func (Offline) Close() error                    { return nil }
func (Offline) Active() bool                    { return false }
func (Offline) Pause(bool)                      {}
func (Offline) Join(view.DeviceID) (int, error) { return 0, errOffline }
func (Offline) Leave(int) error                 { return errOffline }

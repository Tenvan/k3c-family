package rig

import "math"

// Zustände wie in den Sheets (tools/paper/rig/animation.ts, reiten.ts): Pose je Frame und der eingeschwungene Umhang.
// Für den Test gegen die Sollwerte und den Bildvergleich; im Spiel läuft der Umhang mit der echten Zeit.

// rideFloor: tiefster y-Wert des Umhangs beim Reiten (Rücken des Reittiers, Ride-Gruppe).
const rideFloor = -103

// FrameTime: Pose-Zeit von Frame n einer Animation mit `frames` Frames und Frequenz freq.
func FrameTime(n, frames int, freq float64) float64 { return float64(n) / (freq * float64(frames)) }

// CloakAnchor: Anker des Umhangs in Welt-Einheiten (Punkt am Anker-Teil).
func (r *Rig) CloakAnchor(world []M) Point {
	return world[r.Index(r.Cloak.Anchor)].Apply(r.Cloak.Point)
}

// FigureCloakFrames liefert die Umhang-Kette je Frame einer figur-Animation zu Fuß (Boden y = 0).
func (r *Rig) FigureCloakFrames(name string) ([][]Point, error) {
	freq, err := r.FigureFreq(name)
	if err != nil {
		return nil, err
	}
	var pose FigurePose
	var world []M
	state := func(k int) (Point, float64) {
		if err = r.FigurePose(name, StepTime(k), &pose); err != nil {
			return Point{}, 0
		}
		world = r.World(pose.Angles, Translate(pose.Offset[0], pose.Offset[1]), world)
		return r.CloakAnchor(world), pose.Wind
	}
	out := CloakRun(r.Cloak, FrameSteps(freq, r.Animations[name].Frames), state, 0)
	return out, err
}

// Ride hält den Zustand eines Reiters auf seinem Reittier bei einer Phase: Posen, Welt-Matrizen und Ride-Matrix.
type Ride struct {
	Pose       GaitPose
	MountWorld []M
	RiderWorld []M
	Base       M // Ride-Matrix ohne Maßstab (Hub und Neigung); darin liegen Umhang und Durchhang der Zügel
	angles     []float64
}

// Set rechnet Reittier und Reiter bei Phase ph der Gangart; at verschiebt beide (Ort in der Welt).
func (s *Ride) Set(mount, rider *Rig, gaitName string, ph float64, at M) error {
	if err := mount.GaitPose(gaitName, ph, &s.Pose); err != nil {
		return err
	}
	s.Base = Mul(at, RideMatrix(s.Pose.Dy, s.Pose.Pitch, 1))
	s.MountWorld = mount.World(s.Pose.Angles, Mul(at, RideMatrix(s.Pose.Dy, s.Pose.Pitch, mount.Scale)), s.MountWorld)
	s.angles = rider.RiderAngles(&s.Pose, s.angles)
	s.RiderWorld = rider.World(s.angles, RiderBase(mount, s.MountWorld, rider, &s.Pose), s.RiderWorld)
	return nil
}

// CloakAnchor: Anker des Umhangs in der Ride-Gruppe (dort rechnet die Physik beim Reiten).
func (s *Ride) CloakAnchor(rider *Rig) Point {
	return s.Base.Invert().Apply(rider.CloakAnchor(s.RiderWorld))
}

// RideCloakFrames liefert die Umhang-Kette je Frame beim Reiten (Koordinaten der Ride-Gruppe, Boden −103).
func RideCloakFrames(mount, rider *Rig, gaitName string, frames int) ([][]Point, error) {
	freq, err := GaitFreq(gaitName)
	if err != nil {
		return nil, err
	}
	var s Ride
	state := func(k int) (Point, float64) {
		ph := math.Mod(math.Mod(StepTime(k)*freq, 1)+1, 1)
		if err = s.Set(mount, rider, gaitName, ph, Identity); err != nil {
			return Point{}, 0
		}
		return s.CloakAnchor(rider), s.Pose.Wind
	}
	out := CloakRun(rider.Cloak, FrameSteps(freq, frames), state, rideFloor)
	return out, err
}

// RideFloor: Boden des Umhangs beim Reiten.
func RideFloor() float64 { return rideFloor }

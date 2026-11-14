package rig

import "sort"

// keyframeIDs nennt alle Teile, die in den Keyframes einen Winkel haben (sortiert, einmal beim Laden).
func keyframeIDs(kf []Keyframe) []string {
	seen := map[string]bool{}
	var ids []string
	for _, f := range kf {
		for id := range f.Angles {
			if !seen[id] {
				seen[id] = true
				ids = append(ids, id)
			}
		}
	}
	sort.Strings(ids)
	return ids
}

// addKeyframes addiert die Winkel der Keyframes bei Phase p (0..1) auf angles: linear und zyklisch, Teile ohne Wert in
// einem Keyframe zählen dort 0 (tools/paper/render/pose.ts › keyframeWinkel). Die Keyframes sind nach t sortiert.
func (r *Rig) addKeyframes(a Animation, p float64, angles []float64) {
	kf := a.Keyframes
	if len(kf) == 0 {
		return
	}
	last := -1
	for j, f := range kf {
		if f.T <= p {
			last = j
		}
	}
	from, fromT := kf[len(kf)-1], kf[len(kf)-1].T-1
	if last >= 0 {
		from, fromT = kf[last], kf[last].T
	}
	to, toT := kf[0], kf[0].T+1
	if last+1 < len(kf) {
		to, toT = kf[last+1], kf[last+1].T
	}
	share := 0.0
	if toT != fromT {
		share = (p - fromT) / (toT - fromT)
	}
	for _, id := range a.ids {
		if i, ok := r.index[id]; ok {
			va, vb := from.Angles[id], to.Angles[id]
			angles[i] += va + (vb-va)*share
		}
	}
}

// KeyframeHidden sagt, ob ein Teil bei Phase p ausgeblendet ist: Es gilt der letzte Keyframe mit sichtbar-Eintrag und
// t ≤ p, sonst (zyklisch) der letzte überhaupt; ohne Eintrag sichtbar (pose.ts › keyframeAusgeblendet).
func (r *Rig) KeyframeHidden(name, id string, p float64) bool {
	visible, before := true, false
	for _, f := range r.Animations[name].Keyframes {
		v, ok := f.Visible[id]
		if !ok || (before && f.T > p) {
			continue
		}
		visible = v
		before = before || f.T <= p
	}
	return !visible
}

use k3c_sim::state::Coin;

// Fontäne der Truhen (B-953): Münzen fliegen von der Truhe im Bogen zum Zielpunkt und liegen danach am Boden. Die Bahn
// rechnet die Anzeige aus den Feldern der Münze (FromX, LaunchAt, LandsAt); die Simulation entscheidet allein, wann sie
// aufsammelbar ist.

/// `FOUNTAIN_HEIGHT` ist die Scheitelhöhe des Bogens in Spiel-Pixeln.
pub const FOUNTAIN_HEIGHT: f64 = 96.0;

/// `coin_pose` liefert Ort und Höhe (Spiel-Pixel über dem Boden) der Münze zum Zeitpunkt now; `shown` ist false, solange
/// die Münze noch in der Truhe liegt (vor LaunchAt). Eine Münze ohne Fontäne liegt bei X.
pub fn coin_pose(c: &Coin, now: f64) -> (f64, f64, bool) {
    if c.lands_at <= c.launch_at || now >= c.lands_at {
        return (c.x, 0.0, true);
    }
    if now < c.launch_at {
        return (c.from_x, 0.0, false);
    }
    let u = (now - c.launch_at) / (c.lands_at - c.launch_at);
    (
        c.from_x + (c.x - c.from_x) * u,
        4.0 * FOUNTAIN_HEIGHT * u * (1.0 - u),
        true,
    )
}

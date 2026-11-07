//! Spike RS1.6: `collectCoins` (engine/sim/economy.go) als Rust-Kern. Heißester Pfad der Baseline (77 % der Sim-CPU im
//! Insel-Tick). Die Grenze ist klein: Münzen und Spieler als flache Felder rein, je Münze der Nehmer raus; Ereignisse
//! (`coinPickup`) und `GoldCollected` bleiben in Go. Die Reihenfolge folgt dem Go-Code genau: je Münze der erste
//! Spieler (Listenreihenfolge) in Reichweite, der lebt, noch Platz im Beutel hat und nicht gesperrt ist; ein Nehmer
//! bekommt sofort +1 Gold (wirkt auf die folgenden Münzen).

/// Aufruf-Block (`repr(C)`, Go füllt ihn mit denselben Feldern, `engine/core/coins.go`).
#[repr(C)]
pub struct CoinsCall {
    pub n_coins: usize,
    pub coin_x: *const f64,
    /// Gesperrter Spieler (ID) oder -1.
    pub coin_blocked_id: *const i32,
    pub coin_blocked_until: *const f64,
    pub n_players: usize,
    pub player_x: *const f64,
    pub player_id: *const i32,
    /// Gold je Spieler, wird erhöht.
    pub player_gold: *mut i32,
    /// 1: lebt (RespawnIn <= 0).
    pub player_alive: *const u8,
    pub time: f64,
    pub reach: f64,
    pub purse: i32,
    /// Je Münze: Index des Nehmers oder -1.
    pub out_taker: *mut i32,
}

/// Rechnet `collectCoins` für flache Eingaben (reine Funktion, für Tests auch ohne FFI nutzbar).
#[allow(clippy::too_many_arguments)]
pub fn collect(
    coin_x: &[f64],
    coin_blocked_id: &[i32],
    coin_blocked_until: &[f64],
    player_x: &[f64],
    player_id: &[i32],
    player_gold: &mut [i32],
    player_alive: &[u8],
    time: f64,
    reach: f64,
    purse: i32,
    out_taker: &mut [i32],
) {
    for (c, &cx) in coin_x.iter().enumerate() {
        let mut taker = -1;
        for (p, &px) in player_x.iter().enumerate() {
            if (px - cx).abs() > reach {
                continue;
            }
            let blocked = coin_blocked_id[c] >= 0
                && coin_blocked_id[c] == player_id[p]
                && time < coin_blocked_until[c];
            if player_alive[p] != 0 && player_gold[p] < purse && !blocked {
                taker = p as i32;
                break;
            }
        }
        out_taker[c] = taker;
        if taker >= 0 {
            player_gold[taker as usize] += 1;
        }
    }
}

/// C-ABI: rechnet einen Aufruf-Block.
///
/// # Safety
/// `call` und alle Zeiger darin müssen auf Felder der angegebenen Länge zeigen (`out_taker`/`player_gold` beschreibbar).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn k3c_collect_coins(call: *mut CoinsCall) {
    if call.is_null() {
        return;
    }
    // SAFETY: Aufrufer garantiert gültige Zeiger und Längen.
    unsafe {
        let c = &*call;
        let (nc, np) = (c.n_coins, c.n_players);
        // Leere Felder kommen aus Go als Nullzeiger; from_raw_parts mit Null wäre UB.
        if nc == 0 {
            return;
        }
        if np == 0 {
            std::slice::from_raw_parts_mut(c.out_taker, nc).fill(-1);
            return;
        }
        collect(
            std::slice::from_raw_parts(c.coin_x, nc),
            std::slice::from_raw_parts(c.coin_blocked_id, nc),
            std::slice::from_raw_parts(c.coin_blocked_until, nc),
            std::slice::from_raw_parts(c.player_x, np),
            std::slice::from_raw_parts(c.player_id, np),
            std::slice::from_raw_parts_mut(c.player_gold, np),
            std::slice::from_raw_parts(c.player_alive, np),
            c.time,
            c.reach,
            c.purse,
            std::slice::from_raw_parts_mut(c.out_taker, nc),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erster_spieler_in_reichweite_mit_platz() {
        let mut gold = [3, 0];
        let mut out = [0; 3];
        collect(
            &[10.0, 10.0, 99.0],
            &[-1, -1, -1],
            &[0.0; 3],
            &[10.5, 10.2],
            &[7, 8],
            &mut gold,
            &[1, 1],
            0.0,
            1.0,
            4,
            &mut out,
        );
        // erste Münze: Spieler 0 (Gold 3 < 4) -> 4; zweite: Spieler 0 voll, Spieler 1 nimmt; dritte außer Reichweite
        assert_eq!(out, [0, 1, -1]);
        assert_eq!(gold, [4, 1]);
    }

    #[test]
    fn gesperrt_und_tot() {
        let mut gold = [0, 0];
        let mut out = [0; 1];
        collect(
            &[0.0],
            &[7],
            &[5.0],
            &[0.0, 0.0],
            &[7, 8],
            &mut gold,
            &[1, 0],
            1.0,
            1.0,
            9,
            &mut out,
        );
        assert_eq!(out, [-1]); // Spieler 7 gesperrt bis 5, Spieler 8 tot
        collect(
            &[0.0],
            &[7],
            &[5.0],
            &[0.0, 0.0],
            &[7, 8],
            &mut gold,
            &[1, 0],
            6.0,
            1.0,
            9,
            &mut out,
        );
        assert_eq!(out, [0]); // Sperre abgelaufen
    }
}

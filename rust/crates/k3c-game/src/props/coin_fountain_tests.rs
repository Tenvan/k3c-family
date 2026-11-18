use k3c_sim::state::Coin;

use super::coin_fountain::{FOUNTAIN_HEIGHT, coin_pose};

#[test]
fn coin_pose_bogen() {
    let c = Coin {
        x: 110.0,
        from_x: 100.0,
        launch_at: 1.0,
        lands_at: 2.0,
        ..Default::default()
    };
    let (_, _, shown) = coin_pose(&c, 0.5);
    assert!(!shown, "vor dem Start liegt die Münze noch in der Truhe");
    let (x, lift, shown) = coin_pose(&c, 1.5);
    assert!(
        shown && (x - 105.0).abs() <= 1e-9 && (lift - FOUNTAIN_HEIGHT).abs() <= 1e-9,
        "Scheitel: x {x}, Höhe {lift}, sichtbar {shown}"
    );
    let (x, lift, shown) = coin_pose(&c, 2.0);
    assert!(shown && x == 110.0 && lift == 0.0, "gelandet: x {x}, Höhe {lift}");
    let (x, lift, shown) = coin_pose(
        &Coin {
            x: 7.0,
            ..Default::default()
        },
        99.0,
    );
    assert!(
        shown && x == 7.0 && lift == 0.0,
        "Münze ohne Fontäne: x {x}, Höhe {lift}"
    );
}

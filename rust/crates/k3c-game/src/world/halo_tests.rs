//! RU55.5 (AC-05): Höfe an jeder Lichtquellen-Art im Sichtbereich, Obergrenze, nichts außerhalb, Profil go ohne Hof.

use k3c_data::look::{current, with_profile};
use k3c_platform::{Frame as Batch, NullCanvas};
use k3c_sim::state::{ActiveEvent, Castle, Site, World as SimWorld};

use super::*;
use crate::world::light::load_light;
use crate::world::testutil::{cycle, layer_cell};

/// Welt in der Nacht: Burg bei 40 (Lagerfeuer), `n` gebaute Türme (je zwei Fackeln) ab x = 36 in Schritten von 1.
fn night_world(towers: usize) -> SimWorld {
    SimWorld {
        castle: Castle {
            x: 40.0,
            ..Castle::default()
        },
        hub_level: 1,
        hub_x: 40.0,
        cycle: cycle("night", 0.5),
        sites: (0..towers)
            .map(|i| Site {
                x: 36.0 + i as f64,
                kind: "tower".into(),
                state: "built".into(),
                ..Site::default()
            })
            .collect(),
        ..SimWorld::default()
    }
}

fn count(out: &[Halo], kind: Kind) -> usize {
    out.iter().filter(|h| h.kind == kind).count()
}

fn run(level: &str, wd: &SimWorld, x: f64) -> Vec<Halo> {
    crate::world::register_look();
    let (light, g) = (load_light(), current().glow);
    let mut out = Vec::new();
    collect(&light, &g, &layer_cell(x), wd, level, &mut out);
    out
}

#[test]
fn jede_lichtquellen_art_im_sichtbereich_hat_einen_hof() {
    with_profile("k2c", || {
        let out = run("meadow", &night_world(2), 40.0);
        assert_eq!(count(&out, Kind::Campfire), 1, "Lagerfeuer der Burg");
        assert_eq!(count(&out, Kind::Torch), 4, "zwei Türme mit je zwei Fackeln");
        assert_eq!(count(&out, Kind::Moon), 1);
        assert_eq!(count(&out, Kind::MoonDisc), 1);
        let (light, g) = (load_light(), current().glow);
        let fire = out
            .iter()
            .find(|h| h.kind == Kind::Campfire)
            .copied()
            .unwrap_or_else(|| out[0]);
        let want = light.campfire.radius * g.radius * layer_cell(40.0).camera.scale;
        assert!(
            (fire.rx - want).abs() < 1e-9,
            "Radius aus light.json: {} gegen {want}",
            fire.rx
        );
        assert!(fire.alpha > 0.0 && fire.alpha <= 1.0);
    });
}

#[test]
fn nichts_ausserhalb_des_sichtbereichs() {
    with_profile("k2c", || {
        // Kamera weit weg von Burg und Türmen: kein Boden-Hof, nur der Mond am Himmel
        let out = run("meadow", &night_world(2), 500.0);
        assert_eq!(count(&out, Kind::Campfire) + count(&out, Kind::Torch), 0, "{out:?}");
        assert_eq!(count(&out, Kind::Moon), 1);
    });
}

#[test]
fn obergrenze_je_zelle_wird_eingehalten() {
    with_profile("k2c", || {
        let out = run("meadow", &night_world(40), 40.0);
        let ground = count(&out, Kind::Campfire) + count(&out, Kind::Torch);
        assert!(ground <= current().glow.max, "{ground} Höfe am Boden");
        assert_eq!(ground, current().glow.max, "voll ausgeschöpft");
    });
}

#[test]
fn am_tag_kein_hof_und_hoehle_ohne_mond() {
    with_profile("k2c", || {
        let mut day = night_world(2);
        day.cycle = cycle("day", 0.0);
        assert!(run("meadow", &day, 40.0).is_empty(), "am Tag keine Höfe");
        let cave = run("cave", &night_world(2), 40.0);
        assert_eq!(
            count(&cave, Kind::Moon) + count(&cave, Kind::MoonDisc),
            0,
            "Höhle ohne Mond"
        );
        assert_eq!(count(&cave, Kind::Campfire), 1, "Lichter der Höhle bleiben");
    });
}

#[test]
fn blutmond_ist_groesser_und_rot() {
    with_profile("k2c", || {
        let mut w = night_world(0);
        let plain = run("meadow", &w, 40.0);
        w.night_events.push(ActiveEvent {
            id: "bloodMoon".into(),
            seconds_left: 5.0,
        });
        let blood = run("meadow", &w, 40.0);
        let moon = |o: &[Halo]| o.iter().find(|h| h.kind == Kind::Moon).copied().unwrap_or(o[0]);
        assert!(moon(&blood).rx > moon(&plain).rx);
        assert!(moon(&blood).color[0] > moon(&blood).color[2], "rötlich");
    });
}

#[test]
fn profil_go_zeichnet_keinen_hof() {
    with_profile("go", || {
        assert!(run("meadow", &night_world(2), 40.0).is_empty());
        let mut w = World::new(None);
        let (mut cv, wd) = (NullCanvas::default(), night_world(2));
        w.draw_halos(&mut cv, &layer_cell(40.0), &wd, "meadow");
        assert!(w.halos.is_empty());
    });
}

#[test]
fn hof_netz_verlaeuft_von_der_mitte_nach_aussen() {
    with_profile("k2c", || {
        let mut w = World::new(None);
        let mut b = Batch::new();
        b.begin();
        w.draw_halos(&mut b, &layer_cell(40.0), &night_world(1), "meadow");
        b.finish();
        let meshes = w.halos.len();
        assert!(meshes >= 4, "Feuer, Fackeln und Mond: {meshes}");
        let v = &w.halo_mesh.verts;
        assert_eq!(v.len(), 1 + 3 * SEGMENTS);
        let last = w.halos[meshes - 1];
        assert!(
            v[0].color[3] > 0.0 && (f64::from(v[0].pos[0]) - last.x).abs() < 1e-3,
            "Mitte"
        );
        assert_eq!(v[v.len() - 1].color[3], 0.0, "Rand ist durchsichtig");
        assert!(v[1].color[3] >= v[1 + SEGMENTS].color[3], "Kern deckt am stärksten");
    });
}

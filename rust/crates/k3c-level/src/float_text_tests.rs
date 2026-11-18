use k3c_det::golden::go_float;

use super::write_go_float;

#[test]
fn write_go_float_gleich_go_float() {
    let werte = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.1,
        -0.1,
        1e21,
        1e-7,
        1.5e-7,
        123456.789,
        999999.0,
        1e6,
        1234567.0,
        0.0001,
        0.00012345,
        0.00009999,
        100.0,
        12.5,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        5e-324,
        f64::EPSILON,
        0.1 + 0.2,
        1.0 / 3.0,
    ];
    for x in werte {
        let mut s = String::new();
        write_go_float(&mut s, x).expect("Schreiben in String");
        assert_eq!(s, go_float(x), "{x:e}");
    }
}

#[test]
fn write_go_float_gleich_go_float_viele_werte() {
    let mut s = String::new();
    for i in 0..20000i64 {
        for x in [
            i as f64 * 0.37,
            -(i as f64) * 1.0e-3,
            i as f64 * 1.0e4 + 0.5,
            (i as f64).powi(3) * 1.0e-9,
        ] {
            s.clear();
            write_go_float(&mut s, x).expect("Schreiben in String");
            assert_eq!(s, go_float(x), "{x:e}");
        }
    }
}

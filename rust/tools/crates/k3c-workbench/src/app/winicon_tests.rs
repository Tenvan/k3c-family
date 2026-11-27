//! Port von `winicon_test.go`: das eingebettete PNG wird zu einem gültigen Icon-Handle.

#[test]
#[cfg(windows)]
fn fenster_icon_aus_eingebettetem_png() {
    for px in [16, 32] {
        let handle = k3c_kit::winicon::handle(super::window::ICON_PNG, px);
        assert_ne!(handle, 0, "kein Icon der Größe {px}");
    }
}

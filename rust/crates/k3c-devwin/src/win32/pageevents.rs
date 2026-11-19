//! Ereignisse der Seite: aus einem Klick (oder einer Auswahl) wird der Befehl an das Spiel.

use k3c_devproto::Msg;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::CBN_SELCHANGE;

use super::controls;
use crate::layout::{Action, Control, Group, Kind, Values};

/// Kennungen der Controls: `ID_BASE` + flacher Index (Gruppe für Gruppe).
pub(super) const ID_BASE: usize = 1000;
const BN_CLICKED: usize = 0;

/// collect liest die Werte aller Controls der Seite (`hwnds` flach, Gruppe für Gruppe).
fn collect(groups: &[Group], hwnds: &[HWND]) -> Values {
    let mut v = Values::new();
    for (c, h) in groups.iter().flat_map(|g| g.controls.iter()).zip(hwnds) {
        match c.kind {
            Kind::Toggle => {
                v.insert(c.key.clone(), if controls::is_on(*h) { "1" } else { "0" }.to_owned());
            }
            Kind::Select => {
                let idx = controls::selected(*h);
                let text = idx.and_then(|i| c.options.get(i)).cloned().unwrap_or_default();
                v.insert(c.key.clone(), text);
                v.insert(format!("{}.index", c.key), idx.map_or(-1, |i| i as i64).to_string());
            }
            Kind::Input => {
                v.insert(c.key.clone(), controls::text(*h));
            }
            Kind::Button | Kind::Label => {}
        }
    }
    v
}

fn label(c: &Control, v: &Values) -> String {
    if !c.text.is_empty() {
        c.text.clone()
    } else {
        v.get(&c.key).cloned().unwrap_or_else(|| c.key.clone())
    }
}

/// command wertet WM_COMMAND eines Controls aus (`id` = Kennung, `note` = Benachrichtigungscode) und liefert den
/// Befehl samt Beschriftung für die Statuszeile, wenn das Control einen auslöst.
pub(super) fn command(groups: &[Group], hwnds: &[HWND], id: usize, note: usize) -> Option<(Msg, String)> {
    let flat = id.checked_sub(ID_BASE)?;
    let c = groups.iter().flat_map(|g| g.controls.iter()).nth(flat)?;
    let h = *hwnds.get(flat)?;
    let fires = match c.kind {
        Kind::Button | Kind::Toggle => note == BN_CLICKED,
        Kind::Select => note == CBN_SELCHANGE as usize,
        Kind::Input | Kind::Label => false,
    };
    if !fires {
        return None;
    }
    if c.kind == Kind::Toggle {
        controls::toggle(h);
    }
    let values = collect(groups, hwnds);
    let msg = match &c.action {
        Action::None => None,
        Action::Msg(m) => Some((**m).clone()),
        Action::Form(f) => f(&values),
    }?;
    Some((msg, label(c, &values)))
}

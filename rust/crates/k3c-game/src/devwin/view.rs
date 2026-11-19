//! Befehle des Bereichs Anzeige (RU56.10): Rumpf, die Bereichs-Session füllt `dev` und `ui`.

use k3c_devproto::DevCommand;

use super::Host;
use super::commands::Outcome;

/// dev führt die Aktionen des Bereichs aus; `None`: nicht Sache dieses Bereichs.
pub fn dev(_host: &mut dyn Host, _player: i64, _c: &DevCommand) -> Outcome {
    None
}

/// ui führt die Tasten des Bereichs aus; `None`: nicht Sache dieses Bereichs.
pub fn ui(_host: &mut dyn Host, _key: &str, _value: Option<i64>) -> Outcome {
    None
}

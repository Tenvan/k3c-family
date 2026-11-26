//! Asset-Oberfläche (Port von `app_assets.go`, docs/standards/paper-mcp.md › Oberfläche): echte Daten aus dem Checkout.
//! Die Logik liegt in `assetsui`; geändert werden Assets nur über die MCP-Tools `assets_*`. Die Methoden blockieren
//! (git, bun).

use k3c_kit::uikit;
use serde_json::Value;

use super::App;
use super::rpc::encode;
use crate::assetsui::Dienst;

impl App {
    /// Asset-Dienst der Repo-Wurzel; alle Aufrufe teilen das Memo der Prüfergebnisse.
    fn assets(&self) -> Dienst {
        self.wait();
        let mut dienst = Dienst::new(&self.root);
        dienst.memo = self.asset_memo.clone();
        dienst
    }
}

pub(super) fn list(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    encode(app.assets().assets_list()?)
}

pub(super) fn index(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    encode(app.assets().assets_index()?)
}

pub(super) fn check(app: &App, args: &[Value]) -> Result<Value, String> {
    let (pfade,): (Option<Vec<String>>,) = uikit::args(args.to_vec())?;
    encode(app.assets().assets_check(&pfade.unwrap_or_default())?)
}

pub(super) fn groesse(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id,): (String,) = uikit::args(args.to_vec())?;
    encode(app.assets().assets_groesse(&id)?)
}

pub(super) fn groesse_basis(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, basis): (String, String) = uikit::args(args.to_vec())?;
    encode(app.assets().assets_groesse_basis(&id, &basis)?)
}

pub(super) fn frame_ohne(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, animation, frame, ohne): (String, String, i64, Option<Vec<String>>) = uikit::args(args.to_vec())?;
    let ohne = ohne.unwrap_or_default();
    encode(app.assets().assets_frame_ohne(&id, &animation, frame, &ohne)?)
}

pub(super) fn stufen(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    encode(app.assets().assets_stufen()?)
}

pub(super) fn welt_sicht(app: &App, args: &[Value]) -> Result<Value, String> {
    let (welt,): (String,) = uikit::args(args.to_vec())?;
    encode(app.assets().welt_sicht(&welt)?)
}

pub(super) fn hintergruende(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    encode(app.assets().assets_hintergruende()?)
}

pub(super) fn hintergrund(app: &App, args: &[Value]) -> Result<Value, String> {
    let (teil,): (String,) = uikit::args(args.to_vec())?;
    encode(app.assets().assets_hintergrund(&teil)?)
}

pub(super) fn get(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id,): (String,) = uikit::args(args.to_vec())?;
    app.assets().assets_get(&id)
}

pub(super) fn style(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    app.assets().assets_style()
}

pub(super) fn prompts(app: &App, args: &[Value]) -> Result<Value, String> {
    uikit::args::<()>(args.to_vec())?;
    encode(app.assets().assets_prompts()?)
}

pub(super) fn review(app: &App, args: &[Value]) -> Result<Value, String> {
    let (basis,): (String,) = uikit::args(args.to_vec())?;
    encode(app.assets().assets_review(&basis)?)
}

pub(super) fn frame(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, animation, frame, variante, basis): (String, String, i64, String, String) = uikit::args(args.to_vec())?;
    let bild = app.assets().assets_frame(&id, &animation, frame, &variante, &basis)?;
    encode(bild)
}

pub(super) fn pixel_diff(app: &App, args: &[Value]) -> Result<Value, String> {
    let (id, animation, frame, basis): (String, String, i64, String) = uikit::args(args.to_vec())?;
    encode(app.assets().assets_pixel_diff(&id, &animation, frame, &basis)?)
}

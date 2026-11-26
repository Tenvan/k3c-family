//! Prüfen und Rendern eines Set in einem Sammelaufruf der Render-CLI (B-856, Port von `set_plan.go`): validate, bei
//! vorhandener alter Spec diff und je Animation frame alt, frame neu, pixeldiff, zuletzt render. validate, diff und
//! render brechen bei Fehler ab; ein gescheiterter Frame betrifft nur den Pixel-Anteil seiner Animation.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use super::Error;
use super::basis::Asset;
use super::batch::{Schritt, SchrittErgebnis, deute};
use super::cli::{Cli, DiffErgebnis, FrameArgs, PixelErgebnis, RenderErgebnis, frame_args, text};
use super::set::{Pixel, SetErgebnis, set_overlay};

/// Die Indizes und Dateien der Schritte einer Animation im Sammelaufruf; `None`: Name taugt nicht für einen Pfad.
struct PixelSchritte {
    anim: String,
    lauf: Option<(PathBuf, usize)>,
}

/// Animationen (auch `idle@spieler2`, `stand.s2`) sind Schlüssel der Spec, nie Pfade.
static RE_ANIMATION: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new("^[a-z0-9][a-z0-9.@_-]*$").ok());

fn ist_animation(s: &str) -> bool {
    RE_ANIMATION.as_ref().is_some_and(|re| re.is_match(s)) && !s.contains("..")
}

fn schritt(args: Vec<String>, stopp: bool, braucht: Vec<usize>) -> Schritt {
    Schritt { args, stopp, braucht }
}

/// Führt validate, diff, Pixel-Vergleich und render für `spec` aus (`alt_spec` leer: neues Asset) und trägt Felder und
/// Pixel in `r` ein. Scheitert validate, wird `r` geleert.
pub(super) fn pruefe_und_rendere(
    cli: &Cli,
    a: &Asset,
    alt_spec: Option<&Path>,
    spec: &Path,
    r: &mut SetErgebnis,
) -> Result<RenderErgebnis, Error> {
    let mut schritte = vec![schritt(vec!["validate".into(), text(spec)], true, vec![])];
    let mut diff = None;
    let mut pixel = Vec::new();
    if let Some(alt) = alt_spec {
        diff = Some(schritte.len());
        schritte.push(schritt(vec!["diff".into(), text(alt), text(spec)], true, vec![]));
        pixel = pixel_plan(cli, a, (alt, spec), &r.animationen, &mut schritte);
    }
    let render = schritte.len();
    schritte.push(schritt(vec!["render".into(), text(spec)], true, vec![]));
    let res = match cli.batch(&schritte) {
        Ok(res) => res,
        Err(e) => return Err(abbruch(cli, alt_spec, spec, r, e)),
    };
    if let Some(Err(e)) = res.first() {
        *r = SetErgebnis::default();
        return Err(e.clone());
    }
    if let Some(d) = diff {
        match deute::<DiffErgebnis>(&res[d], "diff") {
            Ok(d) => r.felder = d.felder,
            Err(e) => return Err(abbruch(cli, None, spec, r, e)),
        }
        r.pixel = pixel.into_iter().map(|p| pixel_aus(cli, p, &res)).collect();
    }
    match deute::<RenderErgebnis>(&res[render], "render") {
        Ok(out) => Ok(cli.render_rel(out)),
        Err(e) => Err(abbruch(cli, None, spec, r, e)),
    }
}

/// Hängt je Animation frame alt, frame neu (braucht alt) und pixeldiff (braucht beide) an.
fn pixel_plan(
    cli: &Cli,
    a: &Asset,
    (alt_spec, spec): (&Path, &Path),
    anims: &[String],
    schritte: &mut Vec<Schritt>,
) -> Vec<PixelSchritte> {
    let cache = cli.root.join(".cache").join("paper").join("mcp");
    let _ = fs::create_dir_all(cache.join("set"));
    let dir = alt_spec.parent().unwrap_or(alt_spec);
    let mut out = Vec::new();
    for anim in anims {
        let lauf = ist_animation(anim).then(|| {
            let pa = dir.join(format!("{anim}-alt.png"));
            let pn = dir.join(format!("{anim}-neu.png"));
            let mut overlay = cache.clone();
            overlay.extend(set_overlay(&a.id, anim).split('/'));
            let alt = schritte.len();
            let frame = FrameArgs {
                animation: anim.clone(),
                ..FrameArgs::default()
            };
            let diff = vec!["pixeldiff".into(), text(&pa), text(&pn), "--out".into(), text(&overlay)];
            schritte.push(schritt(frame_args(alt_spec, &frame, &pa), false, vec![]));
            schritte.push(schritt(frame_args(spec, &frame, &pn), false, vec![alt]));
            schritte.push(schritt(diff, false, vec![alt, alt + 1]));
            (overlay, alt)
        });
        out.push(PixelSchritte {
            anim: anim.clone(),
            lauf,
        });
    }
    out
}

/// Deutet die drei Schritte einer Animation: alter Frame abgelehnt → Animation neu (Anteil 1), sonst der erste Fehler
/// oder der Pixel-Anteil mit Overlay.
fn pixel_aus(cli: &Cli, p: PixelSchritte, res: &[SchrittErgebnis]) -> Pixel {
    let mut px = Pixel {
        animation: p.anim,
        ..Pixel::default()
    };
    let Some((overlay, alt)) = p.lauf else {
        px.fehler = "Name der Animation taugt nicht für einen Dateinamen".to_owned();
        return px;
    };
    if let Err(e) = &res[alt] {
        if e.abgelehnt().is_some() {
            px.anteil = 1.0;
            px.fehler = "neu".to_owned();
        } else {
            px.fehler = e.to_string();
        }
        return px;
    }
    if let Err(e) = &res[alt + 1] {
        px.fehler = e.to_string();
        return px;
    }
    match deute::<PixelErgebnis>(&res[alt + 2], "pixeldiff") {
        Ok(d) => {
            px.anteil = d.anteil;
            px.boxen = d.boxen;
            px.overlay = cli.rel(&overlay);
        }
        Err(e) => px.fehler = e.to_string(),
    }
    px
}

/// Hält fest, was vor einem Abbruch bekannt war (B-870): Scheitert der Sammelaufruf als Ganzes (Zeitlimit, bun fehlt,
/// Exit 2), liefert ein einzelner diff (nur mit `alt_spec`) wenigstens die Felder. Ablehnungen setzen keinen Grund.
fn abbruch(cli: &Cli, alt_spec: Option<&Path>, spec: &Path, r: &mut SetErgebnis, e: Error) -> Error {
    if e.abgelehnt().is_some() {
        return e;
    }
    r.abbruch = e.to_string();
    // Nach einem Zeitlimit hängt die CLI vermutlich noch: kein weiterer Aufruf, der die Asset-Sperre länger hält.
    if let Some(alt) = alt_spec
        && !r.abbruch.contains("Zeitlimit")
        && let Ok(d) = cli.diff(alt, spec)
    {
        r.felder = d.felder;
    }
    e
}

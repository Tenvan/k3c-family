//! Fixtures der Tests von `next`, `waves`, `queue` und `check` (Go `next_test.go`, `next_lanes_test.go`,
//! `waves_test.go`). Wurzeln unter `.work/`; git ist je Thread abgeschottet, damit nie die Refs und Worktrees des echten
//! Repos zählen.

use std::path::{Path, PathBuf};

use super::testhooks::isolate_git;
use super::testsupport;
use crate::gitops::fixture_tests::Fixture;

/// Eine leere Planungs-Wurzel; beim Fallenlassen verschwindet sie.
pub(super) struct Plan {
    pub fixture: Fixture,
    pub root: PathBuf,
}

impl Plan {
    /// Nur Backlog-Index und der (leere) Ordner der geplanten Sprints.
    pub fn new() -> Plan {
        let fixture = Fixture::empty();
        let root = fixture.tmp.path().join("work");
        isolate_git(fixture.env.clone());
        let plan = Plan { fixture, root };
        plan.write("backlog/README.md", "# Backlog\n");
        plan.write("sprints/geplant/.keep", "");
        plan
    }

    /// Schreibt eine Datei unter docs/.
    pub fn write(&self, rel: &str, text: &str) {
        testsupport::write(&self.root, &format!("docs/{rel}"), text);
    }

    pub fn read(&self, rel: &str) -> String {
        testsupport::read(&self.root, &format!("docs/{rel}")).replace("\r\n", "\n")
    }

    /// Ersetzt den ersten Treffer in einer Datei unter docs/.
    pub fn replace(&self, rel: &str, from: &str, to: &str) {
        let text = self.read(rel);
        assert!(text.contains(from), "{rel}: {from:?} fehlt");
        self.write(rel, &text.replacen(from, to, 1));
    }
}

#[track_caller]
pub(super) fn check_text(out: &str, want: &[&str], not: &[&str]) {
    for s in want {
        assert!(out.contains(s), "fehlt {s:?} in:\n{out}");
    }
    for s in not {
        assert!(!out.contains(s), "unerwartet {s:?} in:\n{out}");
    }
}

pub(super) fn next_project(id: &str, status: &str, rang: &str, sprints: &[&str]) -> String {
    let mut text = format!(
        "# {id} · Projekt {id}\n\n- **Status:** {status}\n- **Rang:** {rang}\n\n## Sprints\n\n| Sprint | Status |\n|---|---|\n"
    );
    for s in sprints {
        text.push_str(&format!("| {s} | aktiv |\n"));
    }
    text
}

/// Nr., Agent, Status, Domäne, Abhängig von, Kriterien.
pub(super) type NextSession<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str);

pub(super) fn next_sprint(plan: &Plan, state: &str, id: &str, project: &str, sessions: &[NextSession]) {
    let dir = format!("sprints/{state}/{id}-s/");
    let mut readme = format!(
        "# {id} · SRV · Sprint {id}\n\n- **Status:** {state}\n- **Projekt:** {project}\n\n## Akzeptanzkriterien\n\n\
         - **AC-01** Erstes.\n- **AC-02** Zweites\n  mit Folgezeile.\n\nNachsatz.\n\n## Sessions\n\n\
         | Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n"
    );
    for &(nr, agent, status, domain, deps, crit) in sessions {
        let file = format!("{nr}-x.md");
        readme.push_str(&format!("| {nr} | `{file}` | Umsetzung | {agent} | {status} |\n"));
        let text = format!(
            "# {nr} · Titel {nr}\n\n- **Agent:** {agent}\n- **Domäne:** {domain}\n- **Umgebung:** offline\n\
             - **Branch:** {}/1-x\n- **Abhängig von:** {deps}\n- **Kriterien:** {crit}\n\n## Ziel\n\nZiel {nr}.\n",
            id.to_lowercase()
        );
        plan.write(&format!("{dir}{file}"), &text);
    }
    plan.write(&format!("{dir}README.md"), &readme);
}

/// BBB (Rang 1) vor AAA (Rang 2), obwohl AAA im Ordner zuerst steht; RUH ruht, CCC hat keinen aktiven Sprint. AAA hat
/// zwei aktive Sprints: A1 (A1.1 in Arbeit sperrt den ganzen Sprint, B-838) und A2.
pub(super) fn next_repo() -> Plan {
    let plan = Plan::new();
    plan.write("backlog/B-010-offen.md", "# B-010 · Offen\n\n- **Status:** offen\n");
    plan.write(
        "backlog/archiv/B-011-fertig.md",
        "# B-011 · Fertig\n\n- **Status:** erledigt\n",
    );
    plan.write("projekte/AAA-a.md", &next_project("AAA", "aktiv", "2", &["A1", "A2"]));
    plan.write("projekte/BBB-b.md", &next_project("BBB", "aktiv", "1", &["B1"]));
    plan.write("projekte/CCC-c.md", &next_project("CCC", "aktiv", "3", &["C9"]));
    plan.write("projekte/RUH-r.md", &next_project("RUH", "ruht", "–", &["R1"]));
    next_sprint(
        &plan,
        "erledigt",
        "E1",
        "–",
        &[("E1.1", "autonom", "fertig", "SIM", "–", "AC-01")],
    );
    next_sprint(
        &plan,
        "aktiv",
        "B1",
        "BBB",
        &[
            ("B1.1", "Mensch", "offen", "SIM", "–", "AC-01"),
            ("B1.2", "autonom", "offen", "SIM", "B1.1", "AC-01"),
            ("B1.3", "autonom", "offen", "DEV", "B-010", "AC-01"),
            ("B1.4", "autonom", "offen", "CLI", "E1.1, B-011", "AC-02"),
        ],
    );
    next_sprint(
        &plan,
        "aktiv",
        "A1",
        "AAA",
        &[
            ("A1.1", "autonom", "in Arbeit", "SRV", "–", "AC-01"),
            ("A1.2", "autonom", "offen", "SRV", "–", "AC-01"),
        ],
    );
    next_sprint(
        &plan,
        "aktiv",
        "A2",
        "AAA",
        &[
            ("A2.3", "autonom", "offen", "INF", "–", "alle"),
            ("A2.4", "autonom", "offen", "CLI", "–", "AC-01"),
            ("A2.5", "autonom", "offen", "INF", "–", "AC-01"),
        ],
    );
    next_sprint(
        &plan,
        "aktiv",
        "R1",
        "RUH",
        &[("R1.1", "autonom", "offen", "PLAT", "–", "AC-01")],
    );
    plan
}

/// Eine Session im Lane-Fixture (Nr., Status, Domäne, Lane, Modell); jede liegt in einem eigenen aktiven Sprint
/// (Sessions eines Sprints laufen nacheinander).
pub(super) type LaneSession<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str);

/// Ein aktives Projekt LAN mit je einem aktiven Sprint pro Session, in der Reihenfolge der Eingabe.
pub(super) fn lane_repo(sessions: &[LaneSession]) -> Plan {
    let plan = Plan::new();
    let mut ids = Vec::new();
    for &(nr, status, domain, lane, model) in sessions {
        let id = nr.split('.').next().unwrap_or_default();
        ids.push(id);
        let dir = format!("sprints/aktiv/{id}-s/");
        plan.write(
            &format!("{dir}README.md"),
            &format!(
                "# {id} · {domain} · Sprint {id}\n\n- **Status:** aktiv\n- **Projekt:** LAN\n\n## Akzeptanzkriterien\n\n\
                 - **AC-01** Eins.\n\n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n\
                 | {nr} | `{nr}-x.md` | Umsetzung | autonom | {status} |\n"
            ),
        );
        let mut head = format!("- **Agent:** autonom\n- **Domäne:** {domain}\n");
        for (name, value) in [("Lane", lane), ("Modell", model)] {
            if !value.is_empty() {
                head.push_str(&format!("- **{name}:** {value}\n"));
            }
        }
        plan.write(
            &format!("{dir}{nr}-x.md"),
            &format!(
                "# {nr} · Titel {nr}\n\n{head}- **Umgebung:** offline\n- **Branch:** {}/1-x\n- **Abhängig von:** –\n\
                 - **Kriterien:** AC-01\n\n## Ziel\n\nZiel.\n",
                id.to_lowercase()
            ),
        );
    }
    plan.write("projekte/LAN-l.md", &next_project("LAN", "aktiv", "1", &ids));
    plan
}

/// Der Abschnitt „Regeln und Einschränkungen“ mit den Planungsangaben.
pub(super) fn rules(area: &str, deps: &str, needs: &str, size: &str, lock: &str) -> String {
    format!(
        "## Regeln und Einschränkungen\n\nDomäne DEV.\n\n- **Bereich:** {area}\n- **Abhängig von:** {deps}\n\
         - **Braucht 🧑:** {needs}\n- **Größe:** {size}\n- **Sperre:** {lock}\n\n## Beispiele\n\n–\n"
    )
}

/// Nr., Agent, Status, Abhängig von, Erlaubte Dateien (kommagetrennt), Kontext.
pub(super) type WaveSession<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str);

pub(super) fn wave_sprint(plan: &Plan, id: &str, project: &str, state: &str, lock: &str, sessions: &[WaveSession]) {
    let dir = format!("sprints/{state}/{id}-s/");
    let area = format!("`tools/{}/**`", id.to_lowercase());
    let mut readme = format!(
        "# {id} · DEV · Sprint {id}\n\n- **Status:** {state}\n- **Projekt:** {project}\n- **Spec:** freigegeben\n\n{}\
         \n## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n",
        rules(&area, "–", "–", "M", lock)
    );
    for &(nr, agent, status, deps, files, context) in sessions {
        let file = format!("{nr}-x.md");
        readme.push_str(&format!("| {nr} | `{file}` | Umsetzung | {agent} | {status} |\n"));
        let mut text = format!(
            "# {nr} · Titel {nr}\n\n- **Agent:** {agent}\n- **Domäne:** DEV\n- **Umgebung:** offline\n\
             - **Branch:** {}/1-x\n- **Abhängig von:** {deps}\n- **Kriterien:** AC-01\n\n## Ziel\n\nZiel.\n\n\
             ## Kontext\n\n{context}\n\n## Erlaubte Dateien\n\n",
            id.to_lowercase()
        );
        for f in files.split(',').filter(|f| !f.is_empty()) {
            text.push_str(&format!("- `{f}`\n"));
        }
        plan.write(&format!("{dir}{file}"), &format!("{text}\n## Nicht-Ziele\n\n–\n"));
    }
    plan.write(&format!("{dir}README.md"), &readme);
}

fn wave_ticket(plan: &Plan, nr: &str, project: &str, prio: &str, spec: &str, rules_text: &str) {
    let text = format!(
        "# {nr} · Ticket {nr}\n\n- **Domäne:** DEV\n- **Typ:** Idee\n- **Prio:** {prio}\n- **Umgebung:** offline\n\
         - **Status:** offen\n- **Sprint:** –\n- **Projekt:** {project}\n- **Spec:** {spec}\n\n## Ziel\n\nZiel.\n\n{rules_text}"
    );
    plan.write(&format!("backlog/{nr}-t.md"), &text);
}

/// AAA (Rang 1) mit A1 (Sperre exe) und R1 (R1.1 in Arbeit), BBB (Rang 2) mit B1 (Sperre exe) und C1.
pub(super) fn waves_repo() -> Plan {
    let plan = Plan::new();
    plan.write("projekte/AAA-a.md", &next_project("AAA", "aktiv", "1", &["A1", "R1"]));
    plan.write("projekte/BBB-b.md", &next_project("BBB", "aktiv", "2", &["B1", "C1"]));
    wave_sprint(
        &plan,
        "A1",
        "AAA",
        "aktiv",
        "exe",
        &[
            ("A1.1", "autonom", "offen", "–", "tools/a/**", ""),
            ("A1.2", "autonom", "offen", "A1.1", "tools/a/x/**", ""),
            (
                "A1.3",
                "Mensch",
                "offen",
                "A1.2",
                "",
                "Abnahme am Fernseher.\n\nGerät: PC",
            ),
        ],
    );
    wave_sprint(
        &plan,
        "R1",
        "AAA",
        "aktiv",
        "–",
        &[("R1.1", "autonom", "in Arbeit", "–", "tools/r/**", "")],
    );
    wave_sprint(
        &plan,
        "B1",
        "BBB",
        "aktiv",
        "exe",
        &[("B1.1", "autonom", "offen", "–", "tools/b/**", "")],
    );
    wave_sprint(
        &plan,
        "C1",
        "BBB",
        "aktiv",
        "–",
        &[
            ("C1.1", "autonom", "offen", "–", "tools/a/sub/file.go", ""),
            ("C1.2", "autonom", "offen", "–", "tools/c/**,docs/backlog/README.md", ""),
            ("C1.3", "Mensch", "offen", "–", "", "Am Gerät prüfen."),
        ],
    );
    wave_tickets(&plan);
    plan
}

fn wave_tickets(plan: &Plan) {
    for (nr, project, prio, spec, area, deps, needs) in [
        ("B-001", "AAA", "hoch", "freigegeben", "`tools/d/**`", "–", "–"),
        ("B-002", "BBB", "niedrig", "freigegeben", "`tools/e/**`", "–", "–"),
        ("B-003", "BBB", "mittel", "freigegeben", "", "–", "–"),
        ("B-004", "BBB", "mittel", "freigegeben", "`tools/f/**`", "B-999", "–"),
        ("B-005", "BBB", "mittel", "freigegeben", "`tools/g/**`", "B-003", "–"),
        ("B-006", "BBB", "mittel", "Entwurf", "`tools/h/**`", "–", "Freigabe"),
        ("B-007", "BBB", "mittel", "Entwurf", "`tools/i/**`", "–", "Entscheidung"),
        ("B-008", "BBB", "mittel", "freigegeben", "`tools/j/**`", "–", "Freigabe"),
    ] {
        wave_ticket(plan, nr, project, prio, spec, &rules(area, deps, needs, "S", "–"));
    }
}

/// `docs/` der Wurzel liegt wirklich in der Fixture.
pub(super) fn assert_inside(plan: &Plan, path: &Path) {
    assert!(
        path.starts_with(plan.fixture.tmp.path()),
        "außerhalb: {}",
        path.display()
    );
}

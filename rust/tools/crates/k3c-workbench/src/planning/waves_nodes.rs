//! Knoten der Wellenberechnung (B-783, Go `waves_nodes.go`): Sessions in Arbeit (belegen Plätze), offene autonome
//! Sessions aktiver Sprints, freigegebene Tickets ohne Sprint und freigegebene geplante Sprints, die 🧑 nicht brauchen.

use std::collections::HashMap;
use std::path::Path;

use super::list::unplanned;
use super::next::{TICKET_REF, session_head, ticket_refs};
use super::next_lanes::lane_of;
use super::planning::{Data, Sprint, is_session_file, is_ticket_file, load, parse_ticket};
use super::priority::{NO_RANK, SESSION_REF};
use super::store::{docs_path, entries, read_text};
use super::store_resolve::{find_entry, find_sprint, session_parts};
use super::tables::first_of;
use super::waves_spec::{PlanSpec, allowed_files, parse_plan_spec};

/// Ein Kandidat für eine Welle.
#[derive(Debug, Clone, Default)]
pub(super) struct Node {
    pub id: String,
    pub kind: &'static str,
    pub title: String,
    pub domain: String,
    pub project: String,
    /// Sprint der Session.
    pub sprint: String,
    pub rank: i64,
    pub prio: u8,
    pub area: Vec<String>,
    pub locks: Vec<String>,
    pub deps: Vec<String>,
    pub running: bool,
    /// Leer = in einer Welle; sonst der Grund.
    pub why: String,
    /// `why` ist ein Fehler der Planungsangaben (unbekannte ID).
    pub fault: bool,
    pub level: usize,
    pub wave: usize,
    /// Warum nicht in einer früheren Welle.
    pub note: String,
    /// Lane-Feld der Session (Wunsch) und die zugewiesene Lane (`plan_waves`); leer = keine.
    pub want_lane: String,
    pub lane: &'static str,
}

/// Worauf eine ID aus „Abhängig von“ zeigt.
#[derive(Debug, Default)]
pub(super) struct Target {
    pub done: bool,
    /// Knoten, nach denen der Abnehmer kommt (Plätze in [`WaveData::nodes`]).
    pub nodes: Vec<usize>,
    /// Grund, warum der Abnehmer in keiner Welle steht; leer = keiner.
    pub wait: String,
    /// Die ID gibt es nicht.
    pub unknown: bool,
}

impl Target {
    fn done() -> Target {
        Target {
            done: true,
            ..Target::default()
        }
    }

    fn unknown() -> Target {
        Target {
            unknown: true,
            ..Target::default()
        }
    }

    fn waiting(wait: String) -> Target {
        Target {
            wait,
            ..Target::default()
        }
    }
}

/// Die geladene Planung und die Knoten.
#[derive(Debug, Default)]
pub(super) struct WaveData {
    root: std::path::PathBuf,
    pub data: Data,
    pub nodes: Vec<Node>,
    by_id: HashMap<String, usize>,
    /// Sprint-ID → Platz in `data.sprints` (ein späterer gleicher Name gewinnt).
    sprints: HashMap<String, usize>,
    /// Session-ID → Status
    status: HashMap<String, String>,
    /// Ticket-Nr → Platz in `data.tickets`
    tickets: HashMap<String, usize>,
    /// Ticket-Nr oder Sprint-ID → Planungsangaben
    pub specs: HashMap<String, PlanSpec>,
    /// Projekt → Platz (aktive nach Rang)
    ranks: HashMap<String, i64>,
    /// Projekt aktiv
    active: HashMap<String, bool>,
}

/// Lädt die Planung mit den Planungsangaben aller nicht archivierten Tickets und offenen Sprints.
pub(super) fn load_wave_data(root: &Path) -> Result<WaveData, String> {
    let data = load(root)?;
    let mut w = WaveData {
        root: root.to_owned(),
        specs: ticket_specs(root),
        ..WaveData::default()
    };
    for (i, p) in (0_i64..).zip(&data.projects) {
        w.active.insert(p.id.clone(), p.status == "aktiv");
        if p.rang.parse::<i64>().is_ok() && p.status == "aktiv" {
            w.ranks.insert(p.id.clone(), i);
        }
    }
    for (i, t) in data.tickets.iter().enumerate() {
        w.tickets.insert(t.nr.clone(), i);
    }
    for (i, sprint) in data.sprints.iter().enumerate() {
        w.sprints.insert(sprint.id.clone(), i);
        for x in &sprint.sessions {
            w.status.insert(x.nr.clone(), x.status.clone());
        }
        if sprint.status != "erledigt"
            && let Some(found) = find_sprint(root, &sprint.id)
            && let Ok((text, _)) = read_text(root, &found.rel)
        {
            w.specs.insert(sprint.id.clone(), parse_plan_spec(&text));
        }
    }
    w.data = data;
    w.build();
    Ok(w)
}

/// Planungsangaben aller Tickets in docs/backlog (ohne Archiv).
fn ticket_specs(root: &Path) -> HashMap<String, PlanSpec> {
    entries(&docs_path(root, "backlog"))
        .iter()
        .filter(|e| !e.is_dir && is_ticket_file(&e.name))
        .filter_map(|e| {
            let (text, _) = read_text(root, &format!("backlog/{}", e.name)).ok()?;
            Some((parse_ticket(&text).nr, parse_plan_spec(&text)))
        })
        .collect()
}

/// `Braucht 🧑` nennt etwas (nicht leer, nicht `–`, auch nicht `– (Notiz)`).
pub(super) fn needs_human(needs: &str) -> bool {
    !needs.is_empty() && !needs.starts_with('–') && needs != "-"
}

/// Die Zeile `Braucht 🧑` steht da und ist `–`.
fn autonomous(spec: &PlanSpec) -> bool {
    !spec.needs.is_empty() && !needs_human(&spec.needs)
}

/// Der Grund „Bereich fehlt“ für Angaben ohne Bereich.
fn missing_area(spec: &PlanSpec) -> String {
    if spec.area.is_empty() {
        "Bereich fehlt".to_owned()
    } else {
        String::new()
    }
}

/// hoch vor mittel vor niedrig.
fn prio_order(prio: &str) -> u8 {
    match prio.to_lowercase().as_str() {
        "hoch" => 0,
        "mittel" => 1,
        "niedrig" => 2,
        _ => 3,
    }
}

impl WaveData {
    pub fn sprint(&self, id: &str) -> Option<&Sprint> {
        self.sprints.get(id).and_then(|&i| self.data.sprints.get(i))
    }

    fn rank_of(&self, project: &str) -> i64 {
        self.ranks.get(project).copied().unwrap_or(NO_RANK)
    }

    fn add(&mut self, node: Node) {
        self.by_id.insert(node.id.clone(), self.nodes.len());
        self.nodes.push(node);
    }

    /// Legt die Knoten an: zuerst Sessions aktiver Sprints, dann Tickets, dann geplante Sprints.
    fn build(&mut self) {
        let mut found = Vec::new();
        for sprint in &self.data.sprints {
            let project_open = sprint.project.is_empty() || self.active.get(&sprint.project) != Some(&false);
            if sprint.status == "aktiv" && project_open {
                found.extend(self.session_nodes(sprint));
            }
        }
        found.extend(self.ticket_nodes());
        found.extend(self.sprint_nodes());
        for node in found {
            self.add(node);
        }
    }

    fn ticket_nodes(&self) -> Vec<Node> {
        let free = PlanSpec::default();
        self.data
            .tickets
            .iter()
            .filter_map(|t| {
                let spec = self.specs.get(&t.nr).unwrap_or(&free);
                let ready = t.status == "offen"
                    && unplanned(first_of(&[&t.sprint, "–"]))
                    && t.spec == "freigegeben"
                    && autonomous(spec);
                ready.then(|| Node {
                    id: t.nr.clone(),
                    kind: "Ticket",
                    title: t.title.clone(),
                    domain: t.domain.clone(),
                    project: t.project.clone(),
                    rank: self.rank_of(&t.project),
                    prio: prio_order(&t.prio),
                    area: spec.area.clone(),
                    locks: spec.locks.clone(),
                    deps: spec.deps.clone(),
                    why: missing_area(spec),
                    ..Node::default()
                })
            })
            .collect()
    }

    fn sprint_nodes(&self) -> Vec<Node> {
        let free = PlanSpec::default();
        self.data
            .sprints
            .iter()
            .filter_map(|s| {
                let spec = self.specs.get(&s.id).unwrap_or(&free);
                let ready = s.status == "geplant" && s.spec == "freigegeben" && autonomous(spec);
                ready.then(|| Node {
                    id: s.id.clone(),
                    kind: "Sprint",
                    title: s.title.clone(),
                    domain: s.domain.clone(),
                    project: s.project.clone(),
                    rank: self.rank_of(&s.project),
                    prio: 1,
                    area: spec.area.clone(),
                    locks: spec.locks.clone(),
                    deps: spec.deps.clone(),
                    why: missing_area(spec),
                    ..Node::default()
                })
            })
            .collect()
    }

    /// Sessions in Arbeit (alle Agenten) und offene autonome Sessions mit Datei.
    fn session_nodes(&self, sprint: &Sprint) -> Vec<Node> {
        let free = PlanSpec::default();
        let spec = self.specs.get(&sprint.id).unwrap_or(&free);
        let mut out = Vec::new();
        for x in &sprint.sessions {
            let running = x.status == "in Arbeit";
            if !running && (x.status != "offen" || x.agent != "autonom" || !is_session_file(&x.file)) {
                continue;
            }
            let mut area = allowed_files(&x.text);
            let mut why = String::new();
            if area.is_empty() {
                area.clone_from(&spec.area);
                why = missing_area(spec);
            }
            let head = session_head(x);
            let wanted = head.get("Abhängig von").map_or("", String::as_str);
            let mut deps: Vec<String> = SESSION_REF.as_ref().map_or_else(Vec::new, |re| {
                re.find_iter(wanted).map(|m| m.as_str().to_owned()).collect()
            });
            deps.extend(ticket_refs(&TICKET_REF, wanted));
            deps.extend(spec.deps.iter().cloned());
            out.push(Node {
                id: x.nr.clone(),
                kind: "Session",
                title: x.titel.clone(),
                domain: x.domain.clone(),
                project: sprint.project.clone(),
                sprint: sprint.id.clone(),
                rank: self.rank_of(&sprint.project),
                prio: 1,
                area,
                locks: spec.locks.clone(),
                deps,
                running,
                why: if running { String::new() } else { why },
                want_lane: lane_of(x),
                ..Node::default()
            });
        }
        out
    }

    /// Löst eine ID aus „Abhängig von“ auf.
    pub fn dep_target(&self, id: &str) -> Target {
        if let Some(&node) = self.by_id.get(id) {
            return Target {
                nodes: vec![node],
                ..Target::default()
            };
        }
        if session_parts(id).is_some() {
            return match self.status.get(id) {
                None => Target::unknown(),
                Some(s) => Target {
                    done: s == "fertig" || s == "verworfen",
                    wait: format!("{id} ({s})"),
                    ..Target::default()
                },
            };
        }
        if id.starts_with("B-") {
            return self.ticket_dep(id);
        }
        self.sprint(id)
            .map_or_else(Target::unknown, |sprint| self.sprint_dep(sprint))
    }

    fn ticket_dep(&self, id: &str) -> Target {
        let Some(ticket) = self.tickets.get(id).and_then(|&i| self.data.tickets.get(i)) else {
            let archived = find_entry(&docs_path(&self.root, "backlog/archiv"), &format!("{id}-"), false);
            return if archived.is_some() {
                Target::done()
            } else {
                Target::unknown()
            };
        };
        if ticket.status == "erledigt" || ticket.status == "verworfen" {
            return Target::done();
        }
        if let Some(sprint) = self.sprint(&ticket.sprint) {
            let mut target = self.sprint_dep(sprint);
            let from = format!("{} ", sprint.id);
            target.wait = target.wait.replacen(&from, &format!("{id} → {from}"), 1);
            return target;
        }
        Target::waiting(format!("{id} ({})", ticket.status))
    }

    /// Ein Sprint ist erledigt, wenn er im Ordner erledigt liegt; sonst kommt der Abnehmer nach allen offenen Sessions,
    /// die selbst Knoten sind. Eine offene Session, die kein Knoten ist (Mensch, ohne Datei), hält ihn auf.
    fn sprint_dep(&self, sprint: &Sprint) -> Target {
        if sprint.status == "erledigt" {
            return Target::done();
        }
        let mut nodes = Vec::new();
        for x in &sprint.sessions {
            if x.status == "fertig" || x.status == "verworfen" {
                continue;
            }
            let Some(&node) = self.by_id.get(&x.nr) else {
                return Target::waiting(format!(
                    "{} ({} {}, Agent {})",
                    sprint.id,
                    x.nr,
                    x.status,
                    first_of(&[&x.agent, "?"])
                ));
            };
            nodes.push(node);
        }
        if nodes.is_empty() && sprint.status == "geplant" {
            return Target::waiting(format!("{} (geplant)", sprint.id));
        }
        Target {
            nodes,
            ..Target::default()
        }
    }
}

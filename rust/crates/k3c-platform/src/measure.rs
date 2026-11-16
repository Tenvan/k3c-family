//! Messung je Bild: Ringpuffer mit Bildabstand (dt) und CPU-Zeit bis `present`, Auswertung als JSON.

use std::fmt::Write as _;

/// Ab dieser Bildzeit gilt ein Bild als spät (ms).
pub const LATE_MS: f64 = 25.0;

#[derive(Clone, Copy, Default)]
struct Sample {
    dt: f32,
    cpu: f32,
}

/// Zähler der letzten vollen Sekunde (für den Fenstertitel).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SecondStat {
    pub frames: u32,
    pub late: u32,
}

/// Ringpuffer der letzten Bilder plus späte Bilder je Sekunde.
pub struct Measure {
    buf: Vec<Sample>,
    head: usize,
    len: usize,
    clock_ms: f64,
    second_frames: u32,
    second_late: u32,
    late_per_s: Vec<u32>,
    late_total: u64,
    frames_total: u64,
}

/// Kennzahlen eines Laufs; Zeiten in ms.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Summary {
    pub frames: u64,
    pub dt_median: f64,
    pub dt_p99: f64,
    pub dt_max: f64,
    pub cpu_median: f64,
    pub cpu_p99: f64,
    pub cpu_max: f64,
    pub late_total: u64,
    pub late_per_s: Vec<u32>,
}

impl Measure {
    /// `cap` = Anzahl der Bilder im Ring (älteste fallen heraus).
    pub fn new(cap: usize) -> Self {
        Self {
            buf: vec![Sample::default(); cap.max(1)],
            head: 0,
            len: 0,
            clock_ms: 0.0,
            second_frames: 0,
            second_late: 0,
            late_per_s: Vec::new(),
            late_total: 0,
            frames_total: 0,
        }
    }

    /// Trägt ein Bild ein; liefert die Zähler, wenn damit eine Sekunde voll ist.
    pub fn push(&mut self, dt_ms: f64, cpu_ms: f64) -> Option<SecondStat> {
        self.buf[self.head] = Sample {
            dt: dt_ms as f32,
            cpu: cpu_ms as f32,
        };
        self.head = (self.head + 1) % self.buf.len();
        self.len = (self.len + 1).min(self.buf.len());
        self.frames_total += 1;
        self.second_frames += 1;
        if dt_ms > LATE_MS {
            self.second_late += 1;
            self.late_total += 1;
        }
        self.clock_ms += dt_ms;
        if self.clock_ms < 1000.0 {
            return None;
        }
        self.clock_ms -= 1000.0;
        let st = SecondStat {
            frames: self.second_frames,
            late: self.second_late,
        };
        self.late_per_s.push(self.second_late);
        self.second_frames = 0;
        self.second_late = 0;
        Some(st)
    }

    pub fn summary(&self) -> Summary {
        let dt = sorted(self.buf[..self.len].iter().map(|s| s.dt));
        let cpu = sorted(self.buf[..self.len].iter().map(|s| s.cpu));
        Summary {
            frames: self.frames_total,
            dt_median: pick(&dt, 0.5),
            dt_p99: pick(&dt, 0.99),
            dt_max: pick(&dt, 1.0),
            cpu_median: pick(&cpu, 0.5),
            cpu_p99: pick(&cpu, 0.99),
            cpu_max: pick(&cpu, 1.0),
            late_total: self.late_total,
            late_per_s: self.late_per_s.clone(),
        }
    }
}

fn sorted(it: impl Iterator<Item = f32>) -> Vec<f32> {
    let mut v: Vec<f32> = it.collect();
    v.sort_unstable_by(f32::total_cmp);
    v
}

fn pick(v: &[f32], q: f64) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    let i = ((v.len() as f64 * q).ceil() as usize).clamp(1, v.len()) - 1;
    f64::from(v[i])
}

impl Summary {
    /// JSON mit Median, p99, längstem Bild und späten Bildern je Sekunde; `label` nennt Backend/Lauf.
    pub fn to_json(&self, label: &str) -> String {
        let label = label.replace('\\', "\\\\").replace('"', "\\\"");
        let late: Vec<String> = self.late_per_s.iter().map(u32::to_string).collect();
        let mut s = String::new();
        let _ = write!(
            s,
            "{{\"label\":\"{label}\",\"frames\":{},\"late_ms\":{LATE_MS},\
             \"dt_ms\":{{\"median\":{:.3},\"p99\":{:.3},\"max\":{:.3}}},\
             \"cpu_ms\":{{\"median\":{:.3},\"p99\":{:.3},\"max\":{:.3}}},\
             \"late_total\":{},\"late_per_s\":[{}]}}",
            self.frames,
            self.dt_median,
            self.dt_p99,
            self.dt_max,
            self.cpu_median,
            self.cpu_p99,
            self.cpu_max,
            self.late_total,
            late.join(","),
        );
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn median_p99_und_maximum() {
        let mut m = Measure::new(1000);
        for i in 1..=100 {
            m.push(f64::from(i), 1.0);
        }
        let s = m.summary();
        assert_eq!(s.dt_median, 50.0);
        assert_eq!(s.dt_p99, 99.0);
        assert_eq!(s.dt_max, 100.0);
        assert_eq!(s.late_total, 75);
    }

    #[test]
    fn sekunden_zaehlen_fruehe_und_spaete_bilder() {
        let mut m = Measure::new(64);
        let mut stats = Vec::new();
        for i in 0..130 {
            let dt = if i == 10 { 40.0 } else { 16.0 };
            stats.extend(m.push(dt, 2.0));
        }
        assert_eq!(stats.len(), 2);
        assert_eq!(stats[0].late, 1);
        assert_eq!(stats[1].late, 0);
        assert_eq!(m.summary().late_per_s, [1, 0]);
    }

    #[test]
    fn ring_vergisst_aelteste_bilder() {
        let mut m = Measure::new(4);
        for dt in [100.0, 1.0, 1.0, 1.0, 1.0] {
            m.push(dt, 0.0);
        }
        let s = m.summary();
        assert_eq!(s.dt_max, 1.0);
        assert_eq!(s.frames, 5);
    }

    #[test]
    fn json_nennt_alle_kennzahlen() {
        let mut m = Measure::new(8);
        m.push(16.0, 1.0);
        let j = m.summary().to_json("probe \"x\"");
        for k in [
            "\"median\"",
            "\"p99\"",
            "\"max\"",
            "\"late_per_s\":[]",
            "probe \\\"x\\\"",
        ] {
            assert!(j.contains(k), "{k} fehlt in {j}");
        }
    }
}

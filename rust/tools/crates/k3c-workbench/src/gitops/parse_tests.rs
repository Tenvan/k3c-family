//! Port von `parse_test.go`.

use super::branches::{parse_refs, parse_track};
use super::files::parse_diff;
use super::repo::last_line;
use super::status::{advise, parse_status, parse_worktrees};
use super::*;

fn change(path: &str, code: &str, staged: bool) -> Change {
    Change {
        path: path.to_owned(),
        code: code.to_owned(),
        staged,
    }
}

#[test]
fn parse_status_liest_kopf_und_dateien() {
    let out = [
        "# branch.oid 73abdd1b",
        "# branch.head feat/git-seite",
        "# branch.upstream origin/feat/git-seite",
        "# branch.ab +2 -1",
        "1 .M N... 100644 100644 100644 aaa bbb src/a b.ts",
        "1 A. N... 000000 100644 100644 000 ccc docs/neu.md",
        "1 MM N... 100644 100644 100644 aaa bbb beides.go",
        "2 R. N... 100644 100644 100644 ddd eee R100 neu.go\talt.go",
        "u UU N... 100644 100644 100644 100644 f1 f2 f3 konflikt.go",
        "? tmp/ü.txt",
    ]
    .join("\n");
    let s = parse_status(&out);
    assert_eq!(s.branch, "feat/git-seite");
    assert_eq!(s.upstream, "origin/feat/git-seite");
    assert_eq!((s.ahead, s.behind, s.upstream_gone), (2, 1, false));
    let want = vec![
        change("src/a b.ts", "M", false),
        change("docs/neu.md", "A", true),
        change("beides.go", "M", true),
        change("beides.go", "M", false),
        change("neu.go", "R", true),
        change("konflikt.go", "U", false),
        change("tmp/ü.txt", "?", false),
    ];
    assert_eq!(s.changes, want);
}

#[test]
fn parse_status_gone_und_detached() {
    let s = parse_status("# branch.head docs/x\n# branch.upstream origin/docs/x\n");
    assert!(s.upstream_gone, "Upstream ohne branch.ab muss gone sein");
    assert_eq!(parse_status("# branch.head (detached)\n").branch, "");
}

#[test]
fn parse_worktrees_ohne_wurzel() {
    let out = "worktree C:/k3c\nHEAD 1111111111\nbranch refs/heads/main\n\n\
               worktree C:/k3c/.claude/worktrees/a\nHEAD 4348d4fc99\ndetached\n\n\
               worktree C:/k3c/.claude/worktrees/b\nHEAD abcdef0123\nbranch refs/heads/sprint/k5\n";
    let want = |path: &str, branch: &str, head: &str| Worktree {
        path: path.to_owned(),
        branch: branch.to_owned(),
        head: head.to_owned(),
    };
    assert_eq!(
        parse_worktrees(out),
        vec![
            want("C:/k3c/.claude/worktrees/a", "", "4348d4fc"),
            want("C:/k3c/.claude/worktrees/b", "sprint/k5", "abcdef01"),
        ]
    );
}

#[test]
fn parse_refs_und_track() {
    let out = [
        "refs/heads/main\torigin/main\t[behind 1]\ts1\t100\tKRT",
        "refs/heads/sprint/bd2\torigin/sprint/bd2\t[gone]\ts2\t90\tReview",
        "refs/heads/lokal\t\t\ts3\t80\tnur hier",
        "refs/heads/beides\torigin/beides\t[ahead 1, behind 2]\ts4\t70\tx",
        "refs/remotes/origin/HEAD\t\t\ts0\t0\t",
        "refs/remotes/origin/main\t\t\ts1\t100\tKRT",
        "refs/remotes/origin/sprint/k5\t\t\ts5\t60\tK5",
    ]
    .join("\n");
    let refs = parse_refs(&out);
    let names: Vec<&str> = refs.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["main", "sprint/bd2", "lokal", "beides", "sprint/k5"]);
    let tracks: Vec<&str> = refs.iter().map(|r| parse_track(r).0).collect();
    assert_eq!(tracks, [TRACK_BEHIND, TRACK_GONE, TRACK_NONE, TRACK_BOTH, TRACK_REMOTE]);
}

#[test]
fn parse_diff_verbindet_code_und_zeilen() {
    let got = parse_diff(
        "A\tneu.go\nM\tsrc/a.ts\nD\tbild.png",
        "10\t0\tneu.go\n3\t2\tsrc/a.ts\n-\t-\tbild.png",
    );
    let want = |path: &str, code: &str, added, deleted, binary| DiffFile {
        path: path.to_owned(),
        code: code.to_owned(),
        added,
        deleted,
        binary,
    };
    assert_eq!(
        got,
        vec![
            want("neu.go", "A", 10, 0, false),
            want("src/a.ts", "M", 3, 2, false),
            want("bild.png", "D", 0, 0, true),
        ]
    );
}

#[test]
fn valid_branch_lehnt_optionen_und_sonderzeichen_ab() {
    for ok in [
        "main",
        "sprint/bd2",
        "feat/git-seite",
        "claude/k4-abschluss-review-487fe9",
        "w11_lauf.3",
    ] {
        assert!(valid_branch(ok), "{ok:?} sollte gültig sein");
    }
    for bad in [
        "", "-D", "--delete", "a..b", "a b", "a;b", "x/", "a//b", "x.lock", "ä", "a\nb", "$(x)",
    ] {
        assert!(!valid_branch(bad), "{bad:?} sollte ungültig sein");
    }
    assert!(!valid_branch(&"a".repeat(201)));
}

#[test]
fn advise_nennt_den_naechsten_handgriff() {
    let on = |branch: &str| Status {
        branch: branch.to_owned(),
        ..Status::default()
    };
    let tracked = || Status {
        upstream: "origin/f".to_owned(),
        ..on("f")
    };
    let cases = [
        (Status { ahead: 1, ..tracked() }, "Push: 1 Commits nicht gepusht"),
        (
            Status {
                upstream_gone: true,
                ..tracked()
            },
            "Remote-Branch gelöscht",
        ),
        (
            Status {
                base_ahead: 3,
                ..on("f")
            },
            "Push mit Upstream",
        ),
        (
            Status {
                upstream: "origin/main".to_owned(),
                base_behind: 2,
                ..on("main")
            },
            "main liegt 2 hinter",
        ),
        (
            Status {
                main_behind: 275,
                ..on("f")
            },
            "main liegt 275 hinter",
        ),
        (
            Status {
                changes: vec![change("a", "", false)],
                ..on("f")
            },
            "1 geänderte Dateien",
        ),
    ];
    for (status, want) in cases {
        let got = advise(&status).join(" | ");
        assert!(got.contains(want), "advise({status:?}) = {got:?}, erwartet {want:?}");
    }
    assert!(advise(&tracked()).is_empty(), "sauberer Stand");
}

#[test]
fn last_line_ueberspringt_hints() {
    assert_eq!(
        last_line("fatal: Not possible to fast-forward, aborting.\nhint: x\n"),
        "fatal: Not possible to fast-forward, aborting."
    );
}

# RP1 · INF · Repo-Hygiene: Branches aufräumen, Altlasten, Regeln

- **Status:** geplant
- **Projekt:** REL
- **Domäne:** INF
- **Prio:** mittel
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-302, B-288, B-094, B-058
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Branches und Worktrees bleiben liegen (B-302), die Spielstand-Regel steht nicht in der Arbeitsweise (B-288), Alt-Binaries und npm-Reste (B-094), `requirements.md` empfiehlt eine Execution-Policy ohne Entscheidung (B-058).

## Ziel

Das Repo enthält nur, was gebraucht wird, und die Regeln entsprechen den Beschlüssen. Am Ende sichtbar: Nur aktive Branches, keine Alt-Binaries, Regeln zu Spielstand und Execution-Policy angeglichen.

## Beteiligte und Zielgruppen

🧑 entscheidet B-058; Agent räumt auf.

## Anforderungen

B-302 › Anforderungen; B-288 › Anforderungen; B-094 › Anforderungen; B-058 › Anforderungen.

## Nicht-Ziele

Release-Ablauf (RL1).

## Regeln und Einschränkungen

Einschiebbar; Löschen entfernter Branches nur nach Rückfrage bei 🧑.

## Beispiele

`git branch -r` → nur `develop`, `main`, aktive `sprint/*`.

## Ausnahme- und Fehlerfälle

Branch mit ungemergten Commits → bleibt, Hinweis im Ergebnis.

## Akzeptanzkriterien

- **AC-01** Lokale Branches und Worktrees werden an festen Meilensteinen aufgeräumt (B-302/AC-01, B-302/AC-02, B-302/AC-03, B-302/AC-04, B-302/AC-05).
- **AC-02** Formatänderungen am Spielstand nehmen keine Rücksicht auf alte Stände (B-288/AC-01).
- **AC-03** Im Repo liegen keine Alt-Binaries und keine npm-Skripte mehr (B-094/AC-01, B-094/AC-02).
- **AC-04** requirements.md empfiehlt keine Sicherheitseinstellung ohne Entscheidung von 🧑 (B-058/AC-01).

## Offene Fragen

- B-058: Empfehlung zur Execution-Policy, entscheidet 🧑.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- RP1.1 Branch-Aufräumen an Meilensteinen (AC-01).
- RP1.2 Regeln und Altlasten (AC-02, AC-03, AC-04).
- RP1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–

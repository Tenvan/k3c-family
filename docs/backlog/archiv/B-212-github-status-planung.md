# B-212 · k3c-dev zeigt PR, CI und Merge-Konflikte je Sprint aus GitHub

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** M8
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑; umfasst B-210, B-211, B-212 (Sprint-Revision 2 bestätigt)

## Ausgangslage

Ob der PR eines Sprints (`sprint/<präfix>` → `develop`) offen ist, die CI grün läuft oder ein Merge-Konflikt besteht, sieht man nur auf GitHub oder per `gh` in der Shell. Die Planungsseite (B-211) kennt nur lokale Worktrees.

## Ziel

Planungsseite und Agenten sehen je Sprint den PR-Stand (offen, Entwurf, gemergt), CI-Ergebnis und Merge-Konflikt, dazu den letzten CI-Lauf auf `develop`.

## Beteiligte und Zielgruppen

🧑 (Überblick, Merge), Agenten (Review-Session, Blockade erkennen).

## Anforderungen

- Quelle ist die GitHub CLI `gh` (Anmeldung liegt bei 🧑, k3c-dev speichert kein Token): `gh pr list --state all --base develop --json …` und `gh run list --branch develop --json …`.
- Zuordnung PR → Sprint über den Branch `sprint/<präfix>` (Präfix = Sprint-ID klein).
- Je Sprint: PR-Nummer mit Link, Zustand (offen/Entwurf/gemergt/geschlossen), CI (grün/rot/läuft), Merge-Status (konfliktfrei/Konflikt/unbekannt).
- Kopfzeile der Planungsseite: letzter CI-Lauf auf `develop`.
- Zwischenspeicher mit kurzer Frist (60 s), Neu-laden-Knopf erzwingt Abruf.
- MCP-Tool `gh_status`: dieselben Daten knapp (eine Zeile je Sprint mit PR).

## Nicht-Ziele

PRs anlegen, mergen oder kommentieren; Webhooks; GitHub-REST mit eigenem Token (nur wenn `gh` nicht reicht, eigenes Ticket).

## Regeln und Einschränkungen

Domäne SRV, nur `tools/k3c-dev/`. Aufruf über `internal/proc` mit Timeout und festen Argumenten, keine Shell, kein Text aus Dateien in Argumenten. Keine neue Abhängigkeit.

## Beispiele

`sprint/m8` hat PR #110 offen, CI rot → Sprint-Karte M8 zeigt `#110 offen · CI rot`; `gh_status` → `M8 #110 offen · CI rot · konfliktfrei`.

## Ausnahme- und Fehlerfälle

`gh` fehlt oder nicht angemeldet → Seite zeigt einen Hinweis, alles andere funktioniert; Tool meldet denselben Hinweis. Kein Netz / Timeout → letzter Stand mit Alter, sonst Hinweis. Branch ohne Sprint → nicht angezeigt.

## Akzeptanzkriterien

- **AC-01** Go-Tests parsen aufgezeichnete `gh`-JSON-Ausgaben (`testdata/`) zu PR-, CI- und Merge-Stand je Sprint, auch für „gh fehlt“ und leere Liste.
- **AC-02** Die Planungsseite zeigt PR, CI und Merge-Status auf der Sprint-Karte und den letzten `develop`-Lauf in der Kopfzeile, im Mock mit allen Zuständen.
- **AC-03** Das MCP-Tool `gh_status` ist registriert, hat Instructions-Abschnitt und Roundtrip-Test.

## Offene Fragen

keine

## Notizen

–

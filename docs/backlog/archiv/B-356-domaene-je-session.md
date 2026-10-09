# B-356 · Eine Session gehört zu genau einer Domäne, ein Sprint darf mehrere Domänen nacheinander enthalten

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** PJ1
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1

## Ausgangslage

Jeder Sprint gehört genau einer Domäne und ändert nur deren Dateien (`docs/arbeitsweise.md` › Domänen); je Domäne ist ein Sprint aktiv (B-174). Ein Thema über mehrere Domänen zerfällt dadurch in eine Kette kleiner Sprints mit je eigenem Review und PR, z. B. Events: K3 (SIM) → K4 (SRV) → K5 (CLI) oder Skills: SK1 (SIM) → RM1 (SRV) → S9 (CLI).

## Ziel

Ein Thema über mehrere Domänen läuft als **ein** Sprint mit Sessions verschiedener Domänen nacheinander und einem PR; Kollisionen zwischen parallelen Läufen bleiben ausgeschlossen, weil die Domäne an der Session hängt.

## Beteiligte und Zielgruppen

Agenten (autonomer Ablauf, parallele Läufe auf zwei Accounts), 🧑 beim Planen und Freigeben.

## Anforderungen

- Jede Session trägt das Pflichtfeld `Domäne` und ändert nur Dateien dieser Domäne (Erlaubte Dateien, plus Tests und Doku dazu).
- Das Sprint-Feld `Domäne` wird eine Liste: die Domänen seiner Sessions in Reihenfolge des ersten Auftretens (z. B. `SIM, SRV, CLI`).
- **Sperre je Domäne auf Session-Ebene:** Eine Session `in Arbeit` sperrt ihre Domäne. Vor dem Beanspruchen prüft der autonome Ablauf die Sprint-Branches auf `origin` (`git ls-remote --heads origin 'sprint/*'`); läuft dort eine Session derselben Domäne, nimmt er die nächste passende Session. Ersetzt „je Domäne ein aktiver Sprint“.
- Sprint-Größe: 3–6 Sessions inklusive Review (statt 2–4). Die Review-Session prüft den Diff aller Domänen des Sprints.
- Commit-Titel tragen die Domäne der Session (`feat(sim): …`).
- Grenzfall Protokoll bleibt: eine eigene Session passt Protokoll und beide Enden an.
- Feature-Kette REG → SIM → CLI darf ein Sprint sein.
- Übergang: Bestehende Sessions bekommen die Domäne ihres Sprints (mechanisch).

## Nicht-Ziele

Projekt-Ebene und Rang (B-355), Werkzeug (B-357, B-358), Umbau bestehender Sprints zu Domänen-übergreifenden (geschieht bei Bedarf beim Bereitmachen, nicht rückwirkend).

## Regeln und Einschränkungen

- Domäne INF: `docs/arbeitsweise.md`, `docs/vorlagen/`, `docs/glossar.md`, `tests/planning.test.ts`; das Feld `Domäne` in allen Session-Dateien unter `docs/sprints/` (Planungsdateien).
- Domänen-Tabelle und Grenzfälle in `docs/arbeitsweise.md` bleiben inhaltlich, sie gelten nun je Session.
- Entschieden von 🧑 am 2026-10-07 im Chat.

## Beispiele

- Sprint „Events“: K.1 SIM (Vollmond, Blutmond), K.2 SRV (Protokoll), K.3 CLI (Banner), K.4 Review → ein Branch, ein PR, Sprint-Feld `Domäne: SIM, SRV, CLI`.
- Lauf A arbeitet an einer CLI-Session in Projekt GRA; Lauf B findet als nächste Session eine CLI-Session in BED → nimmt stattdessen eine Session anderer Domäne oder meldet „nichts frei“.

## Ausnahme- und Fehlerfälle

- Session ohne oder mit unbekannter Domäne → `tests/planning.test.ts` rot.
- Sprint-Feld `Domäne` passt nicht zu den Domänen seiner Sessions → rot.
- Zwei Sessions derselben Domäne `in Arbeit` im selben Checkout → rot.
- Review-Session: trägt die Domäne der letzten Umsetzungs-Session und darf schwere Befunde in allen Domänen des Sprints beheben.

## Akzeptanzkriterien

- **AC-01** `docs/vorlagen/session.md` hat das Feld `Domäne`; `docs/vorlagen/sprint.md` nennt `Domäne` als Liste.
- **AC-02** `docs/arbeitsweise.md` beschreibt Domäne je Session, die Sperre je Domäne auf Session-Ebene, Sprint-Größe 3–6 und das Review über alle Domänen des Sprints; `docs/glossar.md` (`Domäne`, `Sprint`, `Session`) ist angeglichen.
- **AC-03** `tests/planning.test.ts` prüft: jede Session hat eine gültige Domäne, Sprint-Domänen = Domänen seiner Sessions, höchstens eine Session je Domäne `in Arbeit`; je Regel ein Negativtest, `task test -- planning` grün.
- **AC-04** Alle bestehenden Session-Dateien tragen das Feld `Domäne`.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).

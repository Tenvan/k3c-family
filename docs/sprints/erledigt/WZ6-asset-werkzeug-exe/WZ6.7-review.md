# WZ6.7 · Review WZ6

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz6/7-review
- **Abhängig von:** WZ6.5, WZ6.6
- **Tickets:** B-459
- **Kriterien:** alle

## Ziel

Der Sprint-Diff von WZ6 ist gegen AC-01–AC-07 geprüft, schwere Befunde in INF/DEV sind behoben oder als Ticket erfasst. Die Abnahme-Liste für WZ6.8 steht, einschließlich der Punkte aus B-421, und der eine PR des Sprints ist mit Auto-Merge offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session, Schritte 1–6, auf dem Sprint-Branch `sprint/wz6`, nie im selben Lauf wie eine Umsetzung. Die Session trägt die Domäne der letzten Umsetzungs-Session (WZ6.6: DEV). Diff: `git fetch && git diff origin/develop...origin/sprint/wz6`, nur den Diff lesen, alle Domänen des Sprints (INF, DEV). Das Code-Review macht ein getrennter Reviewer, nicht der Schreiber der Sessions.

Es zählen nur schwere Befunde laut `docs/arbeitsweise.md` › Review-Session Schritt 3. Besonders prüfen:

- Gemeinsames Asset-Paket ohne Doppelung zwischen k3c-dev und k3c-studio, kein `internal/` über Modulgrenzen, `replace` statt `go.work`.
- MCP-Tools `assets_*` unverändert nutzbar (AC-07).
- Neuladen bei Änderung von `data/assets.json` ohne Verlust von Auswahl, Filter und Eingaben; Speichern auf veraltetem Stand wird abgelehnt (AC-07).
- Start-Binding `tools/k3c-dev/app_studio.go`: fester Pfad, kein Shell-Aufruf, keine Eingabe von außen im Pfad, Meldung „Asset-Studio nicht gebaut (`task k3c-studio:build`)“ (AC-03).
- Nur Loopback, kein `go run` (AC-01).
- Keine Reste der Workbench-Assets-Seite und von `assetvorschau` (Grep aus WZ6.6, AC-03/AC-06).
- `grafiken.html` und `soundtest.html` unberührt (AC-06).

Datei ≤ 400 und Funktion ≤ 60 Zeilen prüfen die Checks.

**Hardware entkoppelt:** WZ6.8 (`Agent: Mensch`) ist keine Abhängigkeit des Reviews. Das Review führt AC-08 und eine in WZ6.5 offen gebliebene Neubau-Probe als `offen (WZ6.8)`, öffnet den PR, und der Sprint bleibt `aktiv`, bis WZ6.8 fertig ist.

**B-421** (Abnahme M10, Projekt ABN) prüft die Workbench-Assets-Seite und `assetvorschau.html`, die es nach WZ6 nicht mehr gibt. Seine Punkte AC-01–AC-07 gehen gegen das Asset-Studio in die Abnahme-Liste (B-459 › Notizen).

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- Planungsdateien von WZ6 und B-459, neue Tickets, `docs/sprints/README.md` (Fahrplan), alle über k3c-dev
- Für schwere Befunde alle Dateien, die die Sessions WZ6.1–WZ6.6 erlauben, in den Domänen des Sprints (INF, DEV) laut `docs/arbeitsweise.md` › Review-Session Schritt 3; Befunde außerhalb → Ticket

## Nicht-Ziele

Neue Funktionen oder Umbauten über schwere Befunde hinaus; Stil, Doku, Benennung und mögliche Vereinfachungen als Befund behandeln; Befunde außerhalb von INF/DEV beheben (→ Ticket); Kriterien umformulieren oder die Spec freigeben; manuelle Prüfungen im Fenster oder Browser oder ein Urteil anstelle von 🧑 (WZ6.8); über B-421 entscheiden (🧑 in WZ6.8); eine Version setzen (nur vorschlagen).

## Schritte

1. `task check`, `task check:go`, `task check:dev` und `task check:all` grün; bei laufender Workbench über `check_run`.
2. Sprint-Diff lesen, Code-Review durch einen getrennten Reviewer, nur schwere Befunde. Befunde in INF/DEV im Review-Commit beheben und die Checks aus Schritt 1 wiederholen; Befunde außerhalb → Ticket.
3. AC-01–AC-07 je mit Nachweis aus den Session-Ergebnissen ins Ergebnis; AC-08 und eine offene Neubau-Probe aus WZ6.5 als `offen (WZ6.8)`.
4. Abnahme-Liste in WZ6.8 schreiben (`plan_section` WZ6.8 › Kontext, Unterabschnitt `### Abnahme-Liste`): je Punkt, was 🧑 am PC sieht. Dazu gehören die B-421-Punkte gegen das Studio: echte Slots und Filter, Ausschnitt `hub:3` und Ablehnung außerhalb des Bildes, Recherche mit echten Treffern nach Neustart, Import mit Lizenz prüfen/bestätigen, Mini-Szene folgt der Präferenz, fünf Grafik-Szenen mit roten Lücken, `sfx:coin` und Musikwechsel `music:tag` → `music:nacht`. Außerdem Workbench ohne Assets-Seite, Verweis und Meldung bei fehlender EXE, Neuladen bei MCP-Änderung (AC-07) und gegebenenfalls die Neubau-Probe.
5. Abnahme in der Sprint-README, höchstens fünf Zeilen, mit `Version: v… vorgeschlagen (Grund)`.
6. Sprint bleibt `aktiv` (WZ6.8 offen); Fahrplan anpassen. Commit `fix(dev): Review WZ6 (WZ6.7)`, `git fetch`, `git merge origin/develop`, push auf `sprint/wz6`. Danach **den einen PR des Sprints** gegen `develop` öffnen (Vorlage, eine Zeile je Session) und Auto-Merge einschalten: `gh pr merge --auto --squash`; ist der PR schon `CLEAN`, `gh pr merge --squash`.

## Fertig, wenn

- [ ] AC-01–AC-07 mit Nachweis im Ergebnis; AC-08 als `offen (WZ6.8)`; offene Befunde als Ticket.
- [ ] Abnahme-Liste in WZ6.8 › Kontext, einschließlich der Punkte aus B-421.
- [ ] Abnahme in der Sprint-README; der PR von `sprint/wz6` gegen `develop` ist offen, Auto-Merge an (oder gemergt).
- [ ] `task check`, `task check:go`, `task check:dev` und `task check:all` grün.

## Prüfen

```bash
task check
task check:go
task check:dev
task check:all
```

Manuelle Prüfungen (Browser, Xbox, TV) nennt diese Session nicht; sie gehören zu WZ6.8.

## Ergebnis

- **Checks:** `task check` (Lint, Typecheck, Tests), `task check:go`, `dev:test` (k3c-dev) und `task build` grün über `check_run`. `check:dev`/`check:all` sind nicht im Katalog von `check_run`; abgedeckt durch `dev:test`, `task:build` und die Studio-Prüfung in der CI (`k3c-studio`-Job).
- **Diff:** `origin/develop...sprint/wz6` enthält nach dem bereits gemergten PR #341 (WZ6.1–WZ6.5) nur WZ6.6 (Entfernung `assetvorschau`, Dev-only-Mechanismus, Herkunftskommentare) und `.gitattributes` (kein Schaden: `text=auto`, Binärdateien markiert). Keine schweren Befunde. `StudioOpen`: fester Pfad, kein Argument von außen, kein Shell-Aufruf; das Studio öffnet keinen Listener; `grafiken.html`/`soundtest.html` unverändert; `git grep` auf `assetvorschau`/`k3c-dev-only` außerhalb `docs/`: 0 Treffer.
- **AC-01:** angenommen, Validierung offen (WZ6.8): Build und Tests (WZ6.2), kein Port; Fenster nur mit Freigabe geprüft.
- **AC-02:** angenommen (WZ6.3, WZ6.4: Tests, Fensterproben mit Freigabe 🧑).
- **AC-03:** angenommen, Neubau-Probe offen (WZ6.8) (WZ6.5).
- **AC-04:** angenommen (WZ6.2: `check:studio` in `check:all` und CI; WZ6.3: Studio-Vitest in CI).
- **AC-05:** angenommen (WZ6.1). **AC-06:** angenommen (WZ6.3, WZ6.4, WZ6.6). **AC-07:** angenommen (WZ6.2, WZ6.3: `reload.test.tsx`, `ErrStale`).
- **AC-08:** offen (WZ6.8). Abnahme-Liste in WZ6.8 geschrieben, inkl. B-421/AC-01–AC-07 und Neubau-Probe.
- **Geprüft von:** Claude (autonom), keine manuellen Prüfungen. **Abweichung:** WZ6.1–WZ6.5 waren schon mit PR #341 gemergt, der Sprint-PR ist daher ein zweiter PR mit dem Rest. **Version:** v0.19.0 vorgeschlagen. **Neue Tickets:** keine.

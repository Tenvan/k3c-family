# WZ63 · DEV · Studio als eigene EXE mit eigenem MCP-Server, gemeinsamer Code in workkit

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** 3e99a4998
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** Ralf, 2026-11-22, Anweisung und Prompt-Freigabe im Chat (/promptize); Revision 2 (Balancing, Tabs links, Schreibschutz) ebenfalls im Chat

## Ausgangslage

Seit WB1 (Entscheidung 005) steckt das Weltdesign (Welten-Editor, Bereich Assets, MCP-Tools world_*, assets_*, hub_*, level_generate) in k3c-workbench. Ein Game-/Level-Designer muss die ganze Entwickler-Workbench (Planung, Git, Dienste, Balancing, 70+ MCP-Tools) starten und kennen. Die Studio-Pakete (`internal/world`, `assetsui`, `paperassets`, `review`, `studioview`, `hubplan`) hängen nur untereinander und an `mcpsrv`/`App`; trennbar.

## Ziel

Ein Designer startet **k3c-studio.exe** (eigene Wails-App, eigener MCP-Server `k3c-studio`) und kann Welten, Inseln, Chunks, Hub und Assets bearbeiten, ohne die Workbench zu kennen oder zu starten. Die Workbench behält Entwicklung und Balancing. Reiner Split: Seiten und Tools wandern 1:1, nichts wird neu gebaut.

Am Ende sichtbar: `task k3c-studio:build` erzeugt `k3c-studio.exe`; deren MCP-Server listet nur world_*, assets_*, hub_*, level_generate; die Workbench listet sie nicht mehr; beide `instructions.md` nennen ihre Tools eindeutig.

## Beteiligte und Zielgruppen

Zielgruppe: Game-/Level-Designer (Studio). Arbeitet: Claude. Prüft und nimmt manuell ab: Ralf.

## Anforderungen

Entscheidungen (Ralf, 2026-11-22): Umfang Welten + Assets; gemeinsamer Go-Code als neues Modul `tools/workkit` (per `replace`, wie assetkit); Workbench verliert die Studio-Seiten und -Tools ersatzlos (kein Lesezugriff).

**Nachtrag Revision 2 (Ralf, 2026-11-22, im Chat):** (1) Das Balancing (Seite, Pakete `balance`, `simrun`, `datadoc`, `gamedata`, Tools `sim_*`, `replay_run`, `gamedata`) gehört ins Studio. (2) Das Balancing darf nur Welten-Dateien (`data/worlds/*.json`) bearbeiten; Laufausgaben (`reports/`) sind keine Quelldateien. (3) Sind für das Balancing Code-Änderungen nötig, muss eine entsprechende Planung im Chat erfolgen (bei automatischem Balancing: erst Plan im Chat zeigen und Freigabe abwarten, dann ändern). (4) Die Seiten des Studios stehen wie früher als Tabs links.

- Struktur nach Best Practice: `tools/workkit` (gemeinsamer Code, importiert weder Workbench noch Studio), `tools/k3c-studio` (nur Studio-eigener Code), `tools/k3c-workbench` (nur Workbench-eigener Code), Frontend-Gemeinsames bleibt `tools/webkit`.
- Verschieben per `git mv`, danach nur Importpfade und Verdrahtung anpassen.

## Nicht-Ziele

Seiten neu entwerfen oder umbauen; Verhalten ändern (außer dem Schreibschutz aus WZ63.7); schreibender Zugriff der Workbench auf Welten/Assets/Balancing; Rust-Spiel; Sim-, Balance-, Gen-Läufe (bleiben gesperrt, auch das verschobene Balancing wird nur gebaut, nicht gefahren); Spielstand-Migration.

## Regeln und Einschränkungen

Verschieben, nicht neu schreiben. Datei ≤ 400 Zeilen, Funktion ≤ 60 (bestehende Ausnahmen bleiben). Kein Import studio→workbench oder workkit→(studio|workbench). Atomic-Design-Regeln und `tests/projectRules.test.ts` bleiben grün. Schreiben nur im Repo.

- **Bereich:** `tools/workkit/**`, `tools/k3c-studio/**`, `tools/k3c-workbench/**`, `tools/webkit/**`, `Taskfile.yml`, `.mcp.json`, `docs/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** taskfile, exe

## Beispiele

Designer startet nur `k3c-studio.exe`, legt per MCP `world_create` eine Welt an und sieht sie im Welten-Editor; die Workbench läuft dafür nicht. `workbench_status` der Workbench kennt `world_*` nicht mehr.

## Ausnahme- und Fehlerfälle

Laufen Workbench und Studio gleichzeitig, nutzen sie verschiedene Ports (Studio eigener Vorgabe-Port, Env-Override) und teilen sich nur Dateien im Repo. Ein Symbol, das beide brauchen, kommt nach workkit, nie als Kopie.

## Akzeptanzkriterien

- **AC-01** `tools/workkit` ist ein eigenes Go-Modul mit dem gemeinsamen Code (MCP-Gerüst, Checkout-Auflösung, Log/Konsole/Prozess-Basics); Workbench baut und testet darauf grün.
- **AC-02** `tools/k3c-studio` (eigenes go.mod) enthält `world`, `assetsui`, `paperassets`, `review`, `studioview`, `hubplan` und die Tools world_*, assets_*, hub_*, level_generate; kein Import auf die Workbench.
- **AC-03** Die Seiten Welten-Editor, Assets und Hub-Planer laufen im Studio-Frontend unverändert; `task k3c-studio:build` erzeugt `k3c-studio.exe`.
- **AC-04** Der MCP-Server `k3c-studio` hat eigenen Port, `.mcp.json`- und `services.json`-Eintrag; `instructions.md` beider Server nennen ihre Tools disjunkt.
- **AC-05** Die Workbench enthält die Studio-Seiten und -Tools nicht mehr und baut grün.
- **AC-06** `task check:diff` grün; Doku beschreibt zwei Apps. Manuelle Abnahme: Studio startet allein und bearbeitet eine Welt (🧑).
- **AC-07** Balancing-Pakete und -Tools (`balance`, `simrun`, `datadoc`, `gamedata`, sim_test, sim_run, replay_run, gamedata-Tools) liegen im Studio, die Workbench enthält sie nicht mehr; Tool-Kataloge bleiben disjunkt (WZ63.5).
- **AC-08** Das Studio zeigt alle Seiten (Welten, Assets, Hub, Balancing) als Tabs links, wie die Seiten früher in der Workbench; Balancing-Seite unverändert (WZ63.6).
- **AC-09** Die Balancing-Tools des Studios schreiben Quelldateien nur unter `data/worlds/`; jeder andere Schreibversuch in `data/`, `engine/`, `rust/`, `tools/` wird abgelehnt (Test); die Instructions und CLAUDE.md verlangen bei nötiger Code-Änderung einen Plan im Chat vor der Änderung (WZ63.7).

## Offene Fragen

Entschieden per Default (Claude, im Nachgang melden): hub_* (Hub-Planer) gehört zum Weltdesign und geht ins Studio; Studio-Vorgabe-Port wählt Session 2 nach dem nächsten freien Workbench-Port.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ63.1 | `WZ63.1-workkit-gemeinsamer-code.md` | Umsetzung | autonom | fertig |
| WZ63.2 | `WZ63.2-studio-modul-backend.md` | Umsetzung | autonom | fertig |
| WZ63.3 | `WZ63.3-studio-frontend.md` | Umsetzung | autonom | fertig |
| WZ63.4 | `WZ63.4-workbench-bereinigen-doku.md` | Umsetzung | autonom | fertig |
| WZ63.5 | `WZ63.5-balancing-backend-ins-studio.md` | Umsetzung | autonom | offen |
| WZ63.6 | `WZ63.6-balancing-seite-tabs-links.md` | Umsetzung | autonom | offen |
| WZ63.7 | `WZ63.7-balancing-schreibschutz-regeln.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.

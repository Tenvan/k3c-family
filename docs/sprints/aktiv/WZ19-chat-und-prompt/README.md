# WZ19 · DEV · Chat im Studio mit Claude Code CLI und Welt per Prompt füllen

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-488
- **Start-Commit:** 9c54de70 (origin/develop; Branch `sprint/wz19` setzt auf `sprint/wz18` b5cd145e auf, PR #385 offen)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-19, Ralf im Chat (Revision 1)

## Ausgangslage

`B-488 › Ausgangslage`: Das Asset-Studio hat keinen Chat und ruft kein Modell auf; Welten füllt 🧑 von Hand. Voraussetzung: WZ18 (Welt-Dateien und Welten Editor) ist erledigt. Sprint 2 von 2 der Spec B-488.

## Ziel

Ein allgemeiner Chat rechts im Asset-Studio (assistant-ui) spricht über die Claude Code CLI mit einem Modell, speichert den Verlauf eine Woche in `.chats`, und der Button „Prompt-Grundgerüst“ füllt eine Welt: Die Antwort wird geprüft und als Entwurf geladen (`B-488 › Ziel`, Teil Chat und Prompt).

**Am Ende sichtbar:** im Studio eine per Chat gefüllte Welt mit mindestens zwei Inseln als Entwurf; in der CI läuft die CLI-Anbindung nur gegen einen Test-Ersatz; von 🧑 abgenommen.

## Beteiligte und Zielgruppen

`B-488 › Beteiligte und Zielgruppen`: 🧑 gibt die neue Abhängigkeit (assistant-ui) frei, schreibt die Wünsche im Chat und nimmt ab; Agenten je Session; ein getrennter Reviewer.

## Anforderungen

`B-488 › Anforderungen`: Abschnitt „Chat und Welt füllen“. Nur für den Sprint: Die Abhängigkeit assistant-ui wird in der ersten Umsetzungssession mit Lizenz, Größe und React-Version belegt und von 🧑 freigegeben, bevor sie in `package.json` kommt; die CLI-Anbindung ist durch einen Test-Ersatz austauschbar.

## Nicht-Ziele

`B-488 › Nicht-Ziele`: API-Schlüssel und direkte API-Aufrufe, Werkzeuge oder Dateizugriff für das Modell, automatisches Speichern einer Modell-Welt, Änderungen an Welt-Format, Spielstand und Protokoll (WZ18).

## Regeln und Einschränkungen

`B-488 › Regeln und Einschränkungen` (Abschnitt Sicherheit): kein Schlüssel im Studio, Modell nur Text rein, Text raus, Zeitlimit und Abbruch; `.chats` von git ignoriert, Gültigkeit eine Woche; kein echter Modellaufruf in Tests oder CI; Datei ≤ 400, Funktion ≤ 60 Zeilen; Sessions je eine Domäne (DEV, Go-Teil für den Prozess-Start im Studio).

## Beispiele

`B-488 › Beispiele`: Prompt-Grundgerüst, Wünsche im Chat, Prüfung nennt „Insel 3: kein Endboss in der tiefsten Stufe 2“; Antwort ohne JSON → „Antwort nicht lesbar“.

## Ausnahme- und Fehlerfälle

Die CLI fehlt oder ist nicht angemeldet → verständliche Meldung im Chat, kein Absturz; Zeitlimit oder Abbruch → Antwort bleibt als abgebrochen markiert; `.chats` nicht beschreibbar → Chat läuft ohne Verlauf mit Hinweis; Antwort mit unbekannten Feldern oder ungültigen Werten → die Prüfung benennt sie, nichts wird still verworfen.

## Akzeptanzkriterien

Aus `B-488`, Sprint 2:
- **AC-01** `B-488/AC-06` Go-Test mit Test-Ersatz für die CLI: Prompt senden, Antwort fortlaufend, Zeitlimit, Abbruch, keine Schreibrechte und keine Werkzeuge.
- **AC-02** `B-488/AC-07` Chat-Verlauf in `.chats` (von git ignoriert); Einträge älter als eine Woche werden gelöscht, jüngere bleiben.
- **AC-03** `B-488/AC-08` Prompt-Grundgerüst aus den Daten, Antwort geprüft und als Entwurf geladen, Fehler am Entwurf; Chat auf allen Seiten ein- und ausblendbar.
- **AC-04** `B-488/AC-09` (Teil Chat) 🧑 nimmt eine per Chat gefüllte Welt mit zwei Inseln am PC als Entwurf ab.
- **AC-05** `B-488/AC-10` Das Review weist jedes Kriterium nach; neue Abhängigkeit belegt und freigegeben; `task check:all` und `task check:go` grün; kein Chat-Verlauf und keine Welt eingecheckt.

## Offene Fragen

Entschieden: `B-488 › Offene Fragen` (alle). Freigegeben von 🧑 am 2026-10-19 (Chat, Revision 1) mit dem Auftrag, den Sprint autonom umzusetzen; die neue Abhängigkeit assistant-ui ist damit freigegeben (Lizenz, Größe und React-Version belegt WZ19.2 trotzdem). Aktiviert wird WZ19, sobald WZ18 erledigt ist (Branch `sprint/wz19`). Offen für die Umsetzung: genauer Aufruf der CLI (`claude -p` mit Ausgabeformat) und Zuschnitt des Prompt-Textes, beides in WZ19.1 und WZ19.3 zu klären.

**Studio getrennt vom Server (2026-10-20, TS1, B-507):** Das Studio ist nur noch ein eigenes Fenster, keine Seite des Spielservers; Backend und Chat aus WZ19.1 liegen in `tools/k3c-studio/backend` (früher `engine/studio`), `task build` baut kein Studio mehr (Prüfung über `task check:studio`). Siehe `docs/decisions/004-studio-getrennt-datei-watch.md`. Neu offen für 🧑: Chat global rechts in allen Studio-Seiten, Provider nur über CLI und ACP (Notiz in WZ19.2, Frage in `B-488 › Offene Fragen`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ19.1 | `WZ19.1-cli-anbindung-verlauf.md` | Umsetzung | autonom | fertig |
| WZ19.2 | `WZ19.2-chat-im-studio.md` | Umsetzung | autonom | fertig |
| WZ19.3 | `WZ19.3-prompt-grundgeruest.md` | Umsetzung | autonom | fertig |
| WZ19.4 | `WZ19.4-review.md` | Review | autonom | fertig |
| WZ19.5 | `WZ19.5-abnahme.md` | Review | Mensch | offen |

- WZ19.1 (DEV) Go: Anbindung der Claude Code CLI als RPC-Methode des Studios (früher Wails-Binding) mit Test-Ersatz, Zeitlimit, Abbruch, `.chats` mit Löschung nach einer Woche (AC-01, AC-02).
- WZ19.2 (DEV) Abhängigkeit assistant-ui belegen (Lizenz, Größe, React-Version; Freigabe durch 🧑 am 2026-10-19), dann Chat rechts im Studio auf allen Seiten (AC-03).
- WZ19.3 (DEV) Button „Prompt-Grundgerüst“ aus den Daten, Antwort prüfen und als Entwurf im Welten Editor laden (AC-03).
- WZ19.4 (DEV) Review: `task check:all`, Diff, PR (AC-05, alle).
- WZ19.5 (DEV) Abnahme durch 🧑 am PC, Agent Mensch (AC-04).

## Abnahme

- 2026-10-20 Review WZ19.4: AC-01, AC-02, AC-03 und AC-05 geprüft (Nachweise in WZ19.1–WZ19.3 und WZ19.4 › Ergebnis), `task check:all` und `task check:go` grün; AC-04 offen (WZ19.5, Abnahme 🧑 am PC).
- Getrennter Reviewer: keine schweren Befunde, Sicherheit der CLI-Anbindung bestätigt; neues Ticket B-514 (Nachbesserungen, nicht schwer).
- Version: v0.22.0 vorgeschlagen (Minor: neues Chat- und Prompt-Werkzeug im Asset-Studio; nach TS1 v0.21.0, letzter Tag v0.20.0), gesetzt erst nach Bestätigung durch 🧑.

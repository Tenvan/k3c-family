# WZ47 · DEV · Ticket-Relationen, schlanker Index und Sprint-Zusammenfassungen

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-696
- **Start-Commit:** cab18b488
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12 (Gruppe von Ralf zur Umsetzung freigegeben)

## Ausgangslage

Der Ticket-Index `docs/backlog/README.md` enthält auch alle erledigten und verworfenen Tickets und wächst; Duplikate und Ersatz stehen nur als Prosa in Notizen (z. B. die Bereinigung nach Entscheidung 009 am 2026-11-12). B-696 beschreibt drei Teile: Relationen (A), Index-Verdichtung (B), Sprint-Zusammenfassungen (C).

## Ziel

Tickets verweisen maschinenlesbar auf Duplikate und Ersatz, der Index enthält nur offene und eingeplante Tickets, und erledigte Sprints haben eine kurze Zusammenfassung.

Am Ende sichtbar: `plan_get` zeigt „ersetzt durch“/„Duplikate“; `docs/backlog/README.md` < 30 KB; `plan_digest` erzeugt `ZUSAMMENFASSUNG.md` je erledigtem Sprint.

## Beteiligte und Zielgruppen

Claude (Umsetzung WZ47.1–WZ47.3, Review WZ47.4); planende Agenten und 🧑 lesen weniger.

## Anforderungen

`B-696 › Anforderungen` (Teile A, B, C).

## Nicht-Ziele

Alte Duplikate im Archiv nachträglich verknüpfen (B-696 › Offene Fragen 2); Fahrplan-Archiv für Sprints (Frage 5).

## Regeln und Einschränkungen

`B-696 › Regeln und Einschränkungen`. Die Sammeldatei `docs/backlog/README.md` ändert Teil B nur über den Werkzeug-Umbau und im letzten Commit des Sprints nach `git merge origin/develop` (`docs/arbeitsweise.md` › Sammeldateien). Vorlagen-Änderung `docs/vorlagen/ticket.md` mit optionalen Feldern (keine Migration). Neubau der Workbench-EXE (Sperre `exe`).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/k3c-workbench/internal/mcpsrv/tools_planning*.go`, `tools/k3c-workbench/internal/mcpsrv/instructions.md`, `tools/k3c-workbench/app_ticket*.go`, `tools/assetkit/ticket/**`, `tests/planning.test.ts`, `tests/planningDocs.ts`, `docs/vorlagen/ticket.md`, `docs/backlog/archiv/README.md`, `docs/arbeitsweise.md`, `CLAUDE.md`
- **Abhängig von:** WZ44
- **Braucht 🧑:** –
- **Sperre:** exe

## Beispiele

`plan_set B-741 {Duplikat von: B-823}` → B-741 verworfen und archiviert, `plan_get B-823` nennt B-741 unter „Duplikate“.

## Ausnahme- und Fehlerfälle

Zyklus oder unbekannte ID → Ablehnung ohne Dateiänderung. Ähnlichkeitsprüfung schlägt fehl → Ticket wird trotzdem angelegt.

## Akzeptanzkriterien

- **AC-01** Relationen (Teil A): `B-696/AC-01` bis `B-696/AC-06`.
- **AC-02** Index-Verdichtung (Teil B): `B-696/AC-07` bis `B-696/AC-10`.
- **AC-03** Sprint-Zusammenfassungen (Teil C): `B-696/AC-11` bis `B-696/AC-13`.

## Offene Fragen

keine blockierenden. Defaults (Claude, 2026-11-12, Meldung im Nachgang) zu `B-696 › Offene Fragen`: 1 drei Sessions A → B → C in diesem Sprint, je Teil ein Commit; 2 keine nachträgliche Verknüpfung; 3 Stoppwörter weg, Mindestlänge 4, Schwelle 0,4, höchstens fünf Treffer, per Test kalibriert; 4 neue Felder optional am Ende des Kopfs; 5 nicht Teil des Sprints.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ47.1 | `WZ47.1-relationen.md` | Umsetzung | autonom | fertig |
| WZ47.2 | `WZ47.2-index-verdichtung.md` | Umsetzung | autonom | fertig |
| WZ47.3 | `WZ47.3-sprint-zusammenfassungen.md` | Umsetzung | autonom | fertig |
| WZ47.4 | `WZ47.4-review.md` | Review | autonom | fertig |

Keine eigenen Sim- oder Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-14 (WZ47.4, Claude): AC-01 bis AC-03 erfüllt (Ergebnisse WZ47.1–WZ47.3, AC-07 mit Umbau in WZ47.4); `task check`, `task check:go`, `go:test` grün.
Behoben im Review: doppelte Ticket-Nummern im Archiv-Index, Zwischenzustand ohne Archiv-Index, `checkout` für `plan_digest`, strikte Trennung bei `Ersetzt`.
Neue Tickets: keine angelegt (Wellen-Auftrag); Vorschläge im Bericht der Welle (Abnahme-Ticket WZ47, Index-Wechsel über zwei Dateien, Nummern ab B-1000).
Abnahme-Ticket für Sim, Balance und Gen: noch anzulegen (keine Gen-relevanten Änderungen). Nach Merge: Workbench-EXE neu bauen (Sperre `exe`), sonst lehnt die alte EXE Archivierungen ab.

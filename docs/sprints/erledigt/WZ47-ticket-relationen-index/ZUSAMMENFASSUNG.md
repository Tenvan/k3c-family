# WZ47 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Ticket-Relationen, schlanker Index und Sprint-Zusammenfassungen
- **Projekt:** WZG

## Ziel

Tickets verweisen maschinenlesbar auf Duplikate und Ersatz, der Index enthält nur offene und eingeplante Tickets, und erledigte Sprints haben eine kurze Zusammenfassung. Am Ende sichtbar: `plan_get` zeigt „ersetzt durch“/„Duplikate“; `docs/backlog/README.md` < 30 KB; `plan_digest` erzeugt `ZUSAMMENFASSUNG.md` je erledigtem Sprint.

## Ergebnis

2026-11-14 (WZ47.4, Claude): AC-01 bis AC-03 erfüllt (Ergebnisse WZ47.1–WZ47.3, AC-07 mit Umbau in WZ47.4); `task check`, `task check:go`, `go:test` grün. Behoben im Review: doppelte Ticket-Nummern im Archiv-Index, Zwischenzustand ohne Archiv-Index, `checkout` für `plan_digest`, strikte Trennung bei `Ersetzt`. Neue Tickets: keine angelegt (Wellen-Auftrag); Vorschläge im Bericht der Welle (Abnahme-Ticket WZ47, Index-Wechsel über zwei Dateien, Nummern ab B-1000). Abnahme-Ticket für Sim, Balance und Gen: noch anzulegen (keine Gen-relevanten Änderungen). Nach Merge: Workbench-EXE neu bauen (Sperre …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Relationen (Teil A): `B-696/AC-01` bis `B-696/AC-06`. |
| AC-02 | erfüllt | Index-Verdichtung (Teil B): `B-696/AC-07` bis `B-696/AC-10`. |
| AC-03 | erfüllt | Sprint-Zusammenfassungen (Teil C): `B-696/AC-11` bis `B-696/AC-13`. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-696 | verworfen | AC-01 erfüllt, AC-02 erfüllt, AC-03 erfüllt | Gebäude liegen in allen vorgesehenen Auflösungen als Grafik vor |

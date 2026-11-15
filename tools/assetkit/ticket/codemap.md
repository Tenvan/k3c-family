# tools/assetkit/ticket/

## Responsibility
Paket `ticket` (B-595): legt Tickets nach `docs/vorlagen/ticket.md` an (nächste freie Nummer, gefüllte Vorlage, Zeile im Index `docs/backlog/README.md`) und stellt dieselben Bausteine dem Ticket-Teil von `plan_create` in k3c-workbench bereit. Funktionen nehmen `root` explizit und arbeiten direkt auf den Dateien, ohne laufende Workbench.

## Design
- `ticket.go`:
  - Typen `Context` (Seite, von der das Ticket kommt), `Input` (Titel, Beschreibung, Typ, Prio, optional Projekt, Kontext), `Created` (ID, Pfad relativ zur Wurzel mit `/`).
  - `Create(root, in)`: `validate` → `lockBacklog` → Nummer über `LocalHighest` → `fill` (Vorlage mit Kopf, Feldern und Ausgangslage) → `store` → Sperre lösen.
  - `lockBacklog`/`releaseLock` (B-615): exklusive Sperrdatei im Backlog über Prozesse hinweg; verwaiste Sperren (älter als `lockStale`) werden übernommen, gelöscht wird nur die eigene.
  - `store`: erst Ticket, dann Index; scheitert der Index, wird das Ticket wieder entfernt. `Validate` prüft wie `Create`, ohne zu schreiben (B-679).
- `draft.go`: `Defaults(today)` (Kopf ohne Domäne, Typ, Prio), `Optional` (Relationsfelder B-696, nur gesetzt, wenn die Vorlage sie kennt), `Draft(tpl, nr, title, values)` (Überschrift `# <nr> · <title>`, unbekanntes Feld = Fehler), `setSection`, `situation` (Beschreibung mit maskierten `#`, Kontextliste), `Row`/`IndexRow`/`AddToIndex` (Zeile unter `## Offen`, `tableEnd`), `Slug` (klein, Umlaute umschrieben, höchstens 50 Zeichen).
- `files.go`: `Highest(names)` (größte Nummer aus `B-<n>-…`), `LocalHighest(root)` (Backlog und `archiv/`), `Recent(root, n)` (neueste Tickets ohne Archiv), `readText` (merkt CRLF), `writeAtomic` (Temp-Datei im selben Ordner).
- `rename.go`: `Rename(from, to)` wiederholt `os.Rename` bei Zugriffssperren unter Windows (Virenscanner, Suchindex) und löscht `from`, wenn es scheitert.
- Texte intern mit `\n`; die Zeilenenden der Datei setzt erst das Schreiben.

## Flow
1. Ticket-Button der Workbench-Oberfläche → Binding `App.TicketCreate` (`app_ticket.go`) → `ticket.Create(root, in)` → `docs/backlog/B-<n>-<slug>.md` + Index-Zeile → `Created`.
2. `plan_create` (Ticket) → `internal/planning/create.go` → `Defaults` → `Draft` → `IndexRow`/`AddToIndex`; Nummern über `Highest` (Bäume entfernter Branches) und `LocalHighest` (eigener Checkout und Worktrees, `ticketnr.go`), Schreiben über `Rename` (`store.go`).
3. Liste der letzten Tickets: `Recent(root, n)`.

## Integration
- Teil des Go-Moduls `k3c/tools/assetkit`, Import `k3c/tools/assetkit/ticket`.
- Konsumenten: `tools/k3c-workbench/app_ticket.go` (`Create`, `Recent`) und `tools/k3c-workbench/internal/planning/` (`create.go`, `index.go`, `tables.go`, `store.go`, `ticketnr.go`).
- Dateien: `docs/vorlagen/ticket.md`, `docs/backlog/`, `docs/backlog/archiv/`, `docs/backlog/README.md`.
- Tests: `ticket_test.go`, `draft_test.go`.

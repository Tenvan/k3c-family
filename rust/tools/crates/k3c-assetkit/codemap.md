# rust/tools/crates/k3c-assetkit/

## Responsibility

Asset-Logik: Sound-Register, Katalog über `public/`, Lizenzen und Credits, Web-Recherche, Import und Vorrat der Vorschlagsliste; dazu Tickets, Fensterstand und Browser-Start.

## Design

- Wurzel-Modul (`lib.rs`): `register`, `stamp`, `target`, `catalog`, `credits`, `research`, `suggestions*`, `import*`, `vorrat*`, `candidates*`.
- Eigene Module: `ticket` (Tickets nach `docs/vorlagen/ticket.md`), `windowstate`, `browser`, `net` (HTTP-Clients ohne eigene Weiterleitungen).

## Flow

Eine App ruft die Funktionen mit der Repo-Wurzel; geschrieben wird in `data/`, `public/` und für Tickets in `docs/backlog`, Temp unter `.work/tmp`.

## Integration

- Konsumenten: `k3c-studio`, `k3c-workbench`.
- Tests neben den Modulen; `task tools-rust:test -- -p k3c-assetkit`.

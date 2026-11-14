# engine/devrun/

## Responsibility

Dev-Supervisor (Process Supervisor) der Tasks `<app>:dev`: Eine EXE mit `-dev` baut eine Kopie von sich als Kind,
startet sie und baut sie bei jeder Änderung an ihren Go-Quellen neu. Ersetzt `wails dev` und einen externen Watcher.

## Design

- `devrun.go`: `Config` (Name, `Dir`, `Package`, fester Kind-Pfad `Child`, `Args`, `Env`, `Watch`, `Extra`,
  `RestartCode`, `Grace`, `Log`), öffentliche API `Run(ctx, Config)`, `IsChild()` und `Listen(network, addr, wait)`
  (wie `net.Listen`, wiederholt bei belegtem Port, solange das alte Kind ihn noch hält). Interner `runner` mit
  Polling (`pollEvery`), Entprellung (`quiet`), Tausch der EXE (`tempPath`, `swap` mit Wiederholung bei Sperre) und
  Schleifenbremse für Neustarts per `RestartCode` (`loopWindow`, `loopMax`).
- `EnvRebuild` (`K3C_DEV_REBUILD`) erzwingt den ersten Build; `K3C_DEV_CHILD` markiert das Kind und wird beim Laden aus
  der Umgebung entfernt (`takeChildEnv`).
- `child.go`: `child` (laufender Prozess) und Runner-Schritte `build` (`go build` an temporären Pfad), `start`,
  `shutdown`, `stop`, `await`, `kill`.
- `snapshot.go`: `fileSig` und `snapshot`/`watched`/`fresh`/`changed` über die beobachteten Dateien (`*.go` ohne Tests,
  `go.mod`, `go.sum`, `Extra`; übersprungen werden `frontend/`, `node_modules/`, `bin/`, `build/`, versteckte Ordner).
- Plattform-Adapter: `proc_windows.go` (Job-Objekt mit `KILL_ON_JOB_CLOSE` je Kind, `bindChild`, `closeJob`,
  `killTree`, `requestStop` per CTRL_BREAK, `addrInUse`), `proc_other.go` (Prozessgruppe, `SIGINT`, `EADDRINUSE`).

## Flow

1. `Run`: `newRunner` → erster Build (übersprungen, wenn `fresh` und kein `EnvRebuild`) → Kind starten.
2. Schleife: `snapshot` alle `pollEvery`, `settle` wartet bis `quiet` ohne Änderung → `rebuild`.
3. `rebuild`: bauen an `tempPath`; Fehler → altes Kind läuft weiter (❌-Log); Erfolg → Kind sauber beenden (`Grace`,
   danach `killTree`), `swap`, neu starten.
4. Endet das Kind mit `RestartCode`: Neustart ohne Neubau (`onExit`); endet `ctx`: `shutdown`.

## Integration

- Abhängigkeiten: nur Standardbibliothek (`os/exec`, `syscall`, `log/slog`, `net`).
- Konsumenten: `tools/k3c-workbench/headless.go` (`-dev`-Modus, `Listen` für den Backend-Port).
- Aufgerufen über die Tasks `k3c-workbench` bzw. `k3c-workbench:dev`.

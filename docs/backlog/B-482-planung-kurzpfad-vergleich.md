# B-482 · Planungs-Tools erkennen die Repo-Wurzel auch bei Kurzpfaden (8.3) und die CI-Tests des k3c-dev-Jobs sind grün

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der CI-Job „k3c-dev · Frontend · Tests · Lint · Wails (Windows)“ ist auf `develop` seit mindestens `b868ba97` rot. Betroffen sind Tests in `tools/k3c-dev/internal/planning`: `TestTicketNummerAusRemoteRef`, `TestTicketNummerAusAnderemWorktree`, `TestTicketNummerWorktreeNichtLesbar`, `TestCheckWorktreesMergeBase`, `TestCheckWorktreesArchiviertesTicket`. Ursache (2026-10-18 lokal nachgestellt mit `TMP` auf einen 8.3-Kurzpfad): Der Windows-Runner liefert den Temp-Ordner als Kurzpfad (`C:\Users\RUNNER~1\…`), `git worktree list` und `git rev-parse --show-toplevel` geben den Langpfad zurück. `samePath` (`ticketnr.go`) und `parseWorktreePaths` (`wtdiff.go`) vergleichen Pfade als Text und halten dieselbe Wurzel für verschieden: „Ordner ist keine Repo-Wurzel“, die Wurzel erscheint zusätzlich in `CheckWorktrees`.

## Ziel

Die Planungs-Tools vergleichen Ordner nach ihrer Identität (`os.SameFile`), nicht nach dem Text des Pfades; der CI-Job „k3c-dev“ wird grün.

## Beteiligte und Zielgruppen

Entwickler und Agenten, die die Planungs-Tools in Worktrees benutzen; 🧑 für den Release (`task check:all` grün).

## Anforderungen

- Ein Helfer vergleicht zwei Ordner mit `os.SameFile(os.Stat(a), os.Stat(b))`; ist einer nicht lesbar, gilt der bisherige Textvergleich (Groß- und Kleinschreibung egal, `/` und `\` gleich).
- `samePath` (`ticketnr.go`) und `parseWorktreePaths` (`wtdiff.go`) benutzen ihn; weitere Stellen mit Pfadvergleich im Paket prüfen.
- Test unter Windows mit Kurzpfad: Der Test erzeugt einen Ordner mit langem Namen, ermittelt dessen 8.3-Namen (Win32 `GetShortPathName`) und prüft `samePath` sowie `CheckWorktrees`/`nextTicket` mit der Wurzel in Kurzform; ohne 8.3-Namen (abgeschaltet) wird er übersprungen.

## Nicht-Ziele

Keine Änderung der Planungs-Logik außer dem Pfadvergleich; kein Eingriff in `tools/k3c-studio` oder das Spiel.

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-dev/`). Der Kurzpfad-Test gehört in eine Datei mit Build-Tag `windows`; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

Wurzel `C:\Users\RUNNER~1\AppData\Local\Temp\x` und Git-Pfad `C:\Users\runneradmin\AppData\Local\Temp\x` → gleicher Ordner; `CheckWorktrees` listet die Wurzel nicht.

## Ausnahme- und Fehlerfälle

Ein Pfad existiert nicht oder ist nicht lesbar → Textvergleich wie bisher, kein Fehler.

## Akzeptanzkriterien

- **AC-01** Ein Test unter Windows mit Wurzel in Kurzform ist rot ohne und grün mit dem Fix (`samePath`, `CheckWorktrees`, Ticket-Nummer aus anderem Worktree).
- **AC-02** Die fünf genannten Tests sind mit `TMP` auf einen Kurzpfad grün.
- **AC-03** Der CI-Job „k3c-dev · Frontend · Tests · Lint · Wails (Windows)“ ist grün.

## Offene Fragen

keine

## Notizen

Gefunden bei der Release-Prüfung für v0.19.0 (2026-10-18). Auftrag von 🧑 im Chat: Fix mit `os.SameFile` und Kurzpfad-Test. Bezug: B-480 (anderes Problem, nur lokal).

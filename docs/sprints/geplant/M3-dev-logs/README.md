# M3 · SRV · k3c-dev III: Oberfläche und Logs-Seite

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-064
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach M1 und M2 läuft `k3c-dev` ohne Fenster im Terminal; Läufe und Logs sehen nur Agenten.

## Ziel

`k3c-dev` ist ein Windows-Programm mit Fenster, das den MCP-Server hostet und auf der Logs-Seite Läufe und JSON-Logs
live zeigt. Am Ende sichtbar: `k3c-dev.exe` startet, ein Agent ruft `check_run`, der Lauf erscheint in der
Quellenleiste und seine Ausgabe läuft in der Konsole mit.

## Beteiligte und Zielgruppen

Entwickler am Entwickler-PC; 🧑 gibt Abhängigkeiten und Ausnahmen frei und nimmt die Oberfläche ab.

## Anforderungen

B-064 › Anforderungen.

## Nicht-Ziele

B-064 › Nicht-Ziele. Die MCP-Seite folgt in M4.

## Regeln und Einschränkungen

B-064 › Regeln und Einschränkungen. Einschiebbar nach M2. Mit der Freigabe zu genehmigen:

1. **Abhängigkeiten:** `github.com/wailsapp/wails/v2` (v2.16.0 geprüft 2026-09-30), im Frontend `react` und `react-dom`
   19, `@radix-ui/themes` 3, `@vitejs/plugin-react`, `@types/react`, `@types/react-dom` (Versionen beim Umsetzen prüfen);
   Vite und TypeScript wie im Hauptprojekt.
2. **Ausnahmen außerhalb der Domäne** (INF): `package.json` (`check:dev` um Frontend-Typecheck erweitern),
   `.github/workflows/ci.yml` (Job `k3c-dev` baut zusätzlich Frontend und `wails build`), `.oxlintrc.json`
   (`tools/k3c-dev/frontend/dist` und `wailsjs` ignorieren), `requirements.md` (Wails-CLI, WebView2-Laufzeit).

## Beispiele

B-064 › Beispiele.

## Ausnahme- und Fehlerfälle

B-064 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Fenster, Einzelinstanz, gemerkte Größe, CI-Build (B-064/AC-01).
- **AC-02** Kopfzeile, Farbmodus nach der Landingpage und Mock (B-064/AC-02).
- **AC-03** Quellenleiste und Konsole live (B-064/AC-03).
- **AC-04** Reiter `Log` und `Fehler (verdichtet)` (B-064/AC-04).
- **AC-05** `check:dev` und CI grün, Grenzen eingehalten (B-064/AC-05).
- **AC-06** Abnahme durch 🧑: Oberfläche im Fenster gesehen, Beispiel aus B-064 nachgestellt (Beobachtung).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- M3.1 Wails-Gerüst: Fenster, Einzelinstanz, Größe merken, Bindings, Ereignisse, Frontend-Gerüst mit Kopfzeile,
  Farben, Bausteinen, Backend-Vertrag und Mock; Ausnahmen (AC-01, AC-02, AC-05).
- M3.2 Logs-Seite: Quellenleiste und Konsole (ANSI, Einsortieren, Mitlaufen) (AC-03).
- M3.3 Reiter `Log` und `Fehler (verdichtet)`; Prüfung gegen den Mock im Browser-Pane, falls 🧑 sie freigibt (AC-04).
- M3.4 🔍 Review, 🧑 prüft das Fenster (AC-06, alle).

## Abnahme

–

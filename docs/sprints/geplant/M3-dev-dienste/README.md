# M3 · SRV · k3c-dev III: Dienste führen

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-067
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach M1 und M2 hat `k3c-dev` MCP-Kern, Statistik und Spieldaten; Vite-Dev- und Heimnetz-Server startet man weiter von
Hand oder per freiem Shell-Befehl.

## Ziel

`k3c-dev` startet, überwacht und stoppt die Entwicklungs-Dienste; Agenten nutzen dafür `svc_*`-Tools. Am Ende sichtbar:
`svc_start Vite` startet den Dev-Server, `svc_status` zeigt ihn mit PID, CPU und Speicher, `console_tail Vite` seine
Ausgabe, `svc_stop Vite` beendet ihn samt Kindprozessen.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten; 🧑 gibt die Abhängigkeit frei. Die Dienste-Seite folgt in M4 (B-068).

## Anforderungen

B-067 › Anforderungen.

## Nicht-Ziele

B-067 › Nicht-Ziele. Keine Oberfläche (M4).

## Regeln und Einschränkungen

B-067 › Regeln und Einschränkungen. Einschiebbar nach M2. Mit der Freigabe zu genehmigen: Abhängigkeit
`github.com/shirou/gopsutil/v4` (Version beim Umsetzen prüfen), nur im Modul `tools/k3c-dev`.

## Beispiele

B-067 › Beispiele.

## Ausnahme- und Fehlerfälle

B-067 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Konfiguration und Zustandsmaschine mit Health-Prüfung und Auto-Restart (B-067/AC-01, B-067/AC-02).
- **AC-02** Übernahme, `Port belegt` und Metriken (B-067/AC-03, B-067/AC-04).
- **AC-03** Konsole und Log-Level-Zähler je Dienst (B-067/AC-06).
- **AC-04** `svc_*`-Tools und Beenden (B-067/AC-05).
- **AC-05** Von Hand geprüft, `check:dev` grün (B-067/AC-07).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- M3.1 `services.json`, Konfiguration mit Prüfung, Controller und Zustandsmaschine mit Health-Prüfung, Auto-Restart,
  Prozessbaum über `internal/proc`; dort unter Windows ein Job Object statt nur `taskkill /T` prüfen, damit auch
  verwaiste Enkel enden (Hinweis aus dem Review M1.4) (AC-01).
- M3.2 Übernahme per Port, `Port belegt`, Metriken mit `gopsutil`, Dienst-Konsolen in `logs_sources`, Log-Level-Zähler (AC-02, AC-03).
- M3.3 `svc_status`, `svc_start`, `svc_stop`, `svc_restart`, Beenden, Instructions; Prüfung von Hand (AC-04, AC-05).
- M3.4 🔍 Review (alle).

## Abnahme

–

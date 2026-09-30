# M3 · SRV · k3c-dev III: Dienste führen

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-067
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (M3 Revision 1 mit B-067, gopsutil v4.26.8 und x/sys)

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

B-067 › Regeln und Einschränkungen. Einschiebbar nach M2. Mit der Freigabe genehmigt, nur im Modul `tools/k3c-dev`:

1. `github.com/shirou/gopsutil/v4` **v4.26.8** (aktuell, geprüft 2026-09-30) für Port → PID und Metriken.
2. `golang.org/x/sys` als direkte Abhängigkeit (heute schon indirekt über das MCP-SDK) für das Windows Job Object.

Keine Ausnahmen außerhalb der Domäne; der README-Satz in M3.3 ist Doku des Werkzeugs.

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

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M3.1 | `M3.1-controller.md` | Umsetzung | autonom | offen |
| M3.2 | `M3.2-uebernahme-metriken.md` | Umsetzung | autonom | offen |
| M3.3 | `M3.3-svc-tools.md` | Umsetzung | autonom | offen |
| M3.4 | `M3.4-review.md` | Review | autonom | offen |

## Abnahme

–

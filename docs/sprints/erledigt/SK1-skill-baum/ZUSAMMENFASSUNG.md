# SK1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Skill-Baum mit Tank und Zauberer, Respec-Regeln abfragbar
- **Projekt:** SKL

## Ziel

Tank und Zauberer lassen sich über den Baum lernen und umskillen, der Server fragt die Regeln seiteneffektfrei ab. Am Ende sichtbar: Skill-Baum spielbar, Respec und Lernen ohne Seiteneffekt prüfbar.

## Ergebnis

2026-10-18, Review SK1.4 (Diff `origin/develop...origin/sprint/sk1`, `task check` und `task check:go` grün). Keine schweren Befunde; `CanLearn`/`CanRespec` sind die einzige Kopie der Regeln, Fehlertexte und Golden-Daten unverändert. AC-01 SK1.2 (B-007/AC-02 angenommen, Validierung offen: S3.4 verworfen, Abnahme in B-337/AC-05; B-007 bleibt offen) · AC-02 SK1.1 · AC-03 SK1.1, SK1.2 · AC-04 verschoben nach S9.1a (SK1.3 verworfen, B-321 erledigt). Neues Ticket: B-495 (`respec` in der Aktionsliste, SRV). B-270 archiviert. Version: v0.20.1 vorgeschlagen (Patch: neue Prüffunktionen und Tests in der …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Skill-Baum mit Tank und Zauberer ist spielbar: ein Sim-Test zeigt Tank und Zauberer zweier … |
| AC-02 | – | Die Sim prüft Respec und Lernen ohne Seiteneffekt (B-270/AC-01). |
| AC-03 | geprüft | `task check:go` ist grün, Golden-Daten unverändert außer den begründeten neuen Ereignissen … |
| AC-04 | verschoben | Schlag ohne Treffer und Skill ohne Ziel erzeugen ein Ereignis (B-321/AC-01, B-321/AC-02); … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-007 | erledigt | AC-01 geprüft | Skill-Baum mit Tank und Zauberer ist spielbar |
| B-270 | erledigt | AC-02 – | Die Sim prüft Respec und Lernen ohne Seiteneffekt |

# B-270 · Die Sim prüft Respec und Lernen ohne Seiteneffekt

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Aktionsliste im Protokoll v4 (`engine/net/actions.go`, S2.1, B-123) soll `respec` nennen, wenn ein Respec erlaubt
wäre. `engine/sim/monarch.go` bietet nur `Respec(w, p)`, das bei Erfolg die Skills sofort leert, und `LearnSkill`, das
sofort lernt; eine Prüfung ohne Seiteneffekt gibt es nicht. Deshalb fehlt `respec` in der Liste, und `learn` nennt nur
„Punkte frei“, nicht welche Skills lernbar sind.

## Ziel

Der Server kann die Regeln für Respec und Lernen abfragen, ohne sie nachzubauen; die Aktionsliste wird vollständig.

## Beteiligte und Zielgruppen

Entwickler (Sim und Server), Spieler über das Aktionen-Overlay (S3).

## Anforderungen

- Exportierte Prüfungen in `engine/sim` (z. B. `CanRespec(w, p) error`, `CanLearn(w, p, id) error`), die `Respec` und `LearnSkill` selbst nutzen.
- Keine Änderung am Spielverlauf, deterministisch, Golden-Hash unverändert.

## Nicht-Ziele

Darstellung im Client (S3), Protokolländerung (eine eigene SRV-Session nimmt danach `respec` in die Liste auf).

## Regeln und Einschränkungen

Domäne SIM; Funktion ≤ 60 Zeilen, Datei ≤ 400. Regeln aus `docs/rules/monarch.md`.

## Beispiele

Spieler am Tag an der Burg mit gelernten Skills → `CanRespec` nil; in der Nacht → Fehler, Skills unverändert.

## Ausnahme- und Fehlerfälle

Unbekannte Skill-ID → Fehler wie bei `LearnSkill`.

## Akzeptanzkriterien

- **AC-01** Tests zeigen: `CanRespec`/`CanLearn` liefern dieselben Fehler wie `Respec`/`LearnSkill` und ändern den Spieler nicht.

## Offene Fragen

Soll Respec ohne gelernte Skills als erlaubt gelten (heute: ja, ohne Wirkung)? Entscheidet 🧑.

## Notizen

Aus S2.1 (B-123 › Offene Fragen: Umfang der Aktionsliste).

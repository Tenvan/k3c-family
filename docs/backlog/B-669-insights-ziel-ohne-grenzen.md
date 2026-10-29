# B-669 · Die Auswertung eines Berichts stürzt nicht ab, wenn ein Ziel weder Unter- noch Obergrenze hat

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`tools/k3c-workbench/internal/balance/insights_plain.go` › `TargetInsight.target()` greift im letzten Zweig ungeprüft auf `*t.Upper` zu. Hat ein Ziel weder `lower` noch `upper`, stürzt die Auswertung mit einem nil pointer ab (Zeile 173, gefunden 2026-10-29 bei WZ32.1). Echte Ziele in `data/balance-targets.json` haben heute immer eine Grenze, der Fall kam nur in Tests vor.

## Ziel

Die Auswertung (Seite „Balancing“, `balance.Insights`, HTML-Bericht) bleibt bei jedem Bericht stabil.

## Beteiligte und Zielgruppen

Ralf beim Balancing; Agent DEV.

## Anforderungen

- Ziel ohne Grenzen → Satz „Kein Sollwert hinterlegt.“ statt Absturz; Status bleibt so, wie `balance` ihn bewertet.
- Gleiche Prüfung an allen Stellen, die `Lower`/`Upper` dereferenzieren (`grep` in `internal/balance/insights*.go`).

## Nicht-Ziele

Ziele ohne Grenzen fachlich erlauben oder in `balance-targets.json` prüfen.

## Regeln und Einschränkungen

Komplexitäts-Budget; keine Änderung der Berichts-JSON.

## Beispiele

Bericht mit Ziel `{name: "x", lower: null, upper: null}` → Auswertung zeigt „Kein Sollwert hinterlegt.“

## Ausnahme- und Fehlerfälle

nicht relevant (das Ticket ist selbst der Fehlerfall).

## Akzeptanzkriterien

- **AC-01** Go-Test: `balance.Insights` mit einem Ziel ohne Grenzen liefert ohne Panic einen Text mit „Kein Sollwert hinterlegt“.

## Offene Fragen

keine

## Notizen

Gefunden vom Agenten der Session WZ32.1 (B-662).

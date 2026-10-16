# GEB4.4 · Review und Abnahme

- **Status:** in Arbeit
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb4/4-review
- **Abhängig von:** GEB4.3
- **Tickets:** B-435
- **Kriterien:** AC-03

## Ziel

Der Diff von GEB4 ist geprüft, 🧑 hat Burg- und Tor-Stufenreihe in der Vorschau und im Spiel abgenommen, der eine PR des Sprints ist offen.

## Kontext

- Leichter Review: nur schwere Befunde im Diff; besonders ältere PNGs unverändert, gelöschte Grafik nirgends mehr genutzt, Burg wechselt mit `hubLevel`, Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Sichtprüfung (AC-03):** nur mit Freigabe von 🧑 für diesen Lauf. Vite und Spielserver über `svc_start`, Adressen über `get_urls` (seit #309 Spiel-Vite auf Port 5183, sobald k3c-dev neu gebaut ist; sonst 5173 unter `127.0.0.1`). Vorschau `assetvorschau.html?szene=gebaeude`; Spiel nur, wenn 🧑 auch das freigibt, sonst prüft 🧑 selbst.

## Erlaubte Dateien

- Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 verlangte Nachbesserungen
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit ohne Auftrag von 🧑, GEB5-Gebäude, TV-Test.

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/geb4` lesen, Befunde behandeln.
3. Mit Freigabe von 🧑: Vorschau (und ggf. Spiel) zeigen, Abnahme im Ergebnis festhalten.
4. Nachweis je Kriterium AC-01 bis AC-06; Abschnitt „Abnahme“.
5. Sprint nach `erledigt/`, `Status: erledigt`; PR `sprint/geb4` → `develop` öffnen.

## Fertig, wenn

- [ ] AC-03: Abnahme durch 🧑 steht im Ergebnis.
- [ ] AC-01, AC-02, AC-04–AC-06: Nachweise aus GEB4.1–GEB4.3 geprüft.
- [ ] `task check` und `task check:go` grün; PR offen.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

2026-10-16, Claude (Ralf im Chat), Zwischenstand.
- `task check` und `task check:go` grün; Sprint-Diff gegen `origin/sprint/geb3` ohne schwere Befunde (keine älteren PNGs geändert, kein Zufall, alle Dateien ≤ 400 Zeilen; Reviews GEB4.1–GEB4.3 eingearbeitet).
- Sichtprüfung: Ralf hat für diesen Lauf nur die Vorschau freigegeben. Claude hat `assetvorschau.html?szene=gebaeude` (Port 5183 seit #309) mit Burg 1–5 und Tor 1–5 gezeigt; Ralf: **Vorschau abgenommen**, Spiel offen (prüft Ralf selbst; Spielserver und Vite laufen).
- Offen: AC-03 Teil Spiel, danach Abnahme-Abschnitt, Sprint abschließen, PR. Hinweis: Die Burg ist mit 8 Kacheln schmaler als der alte Aufbau (etwa 10).

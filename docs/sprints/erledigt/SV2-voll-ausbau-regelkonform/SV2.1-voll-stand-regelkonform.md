# SV2.1 · Voll-Stand aus der Welt-Datei und über sim.DevBuildAll

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** sv2/1-voll-stand-regelkonform
- **Abhängig von:** –
- **Tickets:** B-500, B-425
- **Kriterien:** AC-01, AC-02

## Ziel

Ein bis zwei Sätze: welches Ergebnis nach dieser Session auf `develop` liegt.

## Kontext

Alles, was ein Agent **ohne Vorwissen** braucht: Stand des Codes, relevante Dateien mit Pfad, Entscheidungen
(`docs/decisions/…`), Fallstricke. Keine Verweise auf Chat-Verläufe.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- `pfad/…`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Konkrete, nummerierte Schritte in sinnvoller Reihenfolge.

## Fertig, wenn

- [ ] AC-01: Jeder Punkt ist prüfbar: ein Befehl mit erwartetem Ergebnis, ein Test, eine Datei, eine Beobachtung.
- [ ] Punkte ohne Kriterium sind technische Voraussetzungen (z. B. CI grün).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 und AC-02: umgesetzt. Der Voll-Stand nimmt die Tiefen aus der Welt-Datei und baut je Inselstufe über `sim.DevBuildAll(isl, hub-i)` aus. `islandDepths` und die eigene Stufen-Logik sind entfernt.
- Geprüft mit `go test ./engine/... ./cmd/...` und golangci-lint (grün). Neu ist `TestVollAusgebautHaeltAusbauregeln` (Tiefen, Stufe ≤ Hub, Voraussetzungen, Tore), dazu der Test mit echtem `store.Saves`. Golden ist byte-gleich.
- Abweichung: Den SV1.3-Test „jeder Platz gebaut“ hat Claude gelockert, weil er B-425/AC-01 widerspricht. Auf Tiefe 4 (Hub 1) bleibt `stairsUp` ungebaut, ein regulärer Zustand (Review: kein Befund).

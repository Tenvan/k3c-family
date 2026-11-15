# B-1005 · golangci-lint meldet in native/k3c-game/menu/licensefile kein QF1001 mehr

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NA1
- **Projekt:** NAT
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`golangci-lint run ./menu/...` im Modul `native/k3c-game` meldet (geprüft 2026-11-15) einen Befund: `menu/licensefile/main.go:82:19: QF1001: could apply De Morgan's law (staticcheck)` an der Bedingung `e.IsDir() || !(strings.HasPrefix(up, "LICENSE") || …)`.

## Ziel

Der Lint-Lauf über `native/k3c-game/menu` ist ohne Befund.

## Beteiligte und Zielgruppen

Agenten (`task check:diff`); 🧑 gibt frei.

## Anforderungen

- Die Bedingung ist ohne Verhaltensänderung umgeformt (Hilfsfunktion `isLicenseName(up)` oder De Morgan), kein `//nolint`.
- Die erzeugte Lizenzdatei bleibt byte-gleich.

## Nicht-Ziele

Andere Lint-Befunde außerhalb von `menu/licensefile`; Änderungen am Lizenztext.

## Regeln und Einschränkungen

Funktion ≤ 60 Zeilen.

- **Bereich:** `native/k3c-game/menu/licensefile/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

`golangci-lint run ./menu/...` in `native/k3c-game` → `0 issues`.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Umformung.

## Akzeptanzkriterien

- **AC-01** `golangci-lint run ./menu/...` in `native/k3c-game` meldet 0 Befunde; `go test ./menu/...` grün.
- **AC-02** Der Generator liefert dieselbe Ausgabe wie vorher (`git diff --stat` zeigt nur `menu/licensefile/main.go`, ggf. dessen Test).

## Offene Fragen

keine

## Notizen

Gefunden beim Planen 2026-11-15.

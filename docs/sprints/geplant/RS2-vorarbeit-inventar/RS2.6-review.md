# RS2.6 · Review RS2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/rs2
- **Abhängig von:** RS2.1, RS2.2, RS2.3, RS2.4
- **Tickets:** B-736, B-726, B-731
- **Kriterien:** alle

## Ziel

Sprint RS2 wird abgenommen: alle Kriterien geprüft, Befunde behoben, Folgesprint RS3 (Orakel und Skelett) nach Ralfs Entscheidung zu G1 bereit gemacht oder Projekt RST pausiert.

## Kontext

Review-Session nach `docs/arbeitsweise.md` › Review-Session. Hardware-Session RS2.5 ist keine Abhängigkeit (entkoppelt): Das Review führt AC-05 als `angenommen, Validierung offen (RS2.5)`, der Sprint bleibt `aktiv`, bis RS2.5 fertig ist. Zu prüfen: B-726, B-731, 007-Entwurf, Zahlen in 006.

## Erlaubte Dateien

- `docs/**`
- Behebungen kleiner Befunde (< ~30 Zeilen) in den vom Sprint geänderten Dateien

## Nicht-Ziele

Kein Rust-Code, keine Freigabe von Folgesprints (Ralf).

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander (race lokal ohne gcc: als übersprungen nennen).
2. Je Sprint-Kriterium Nachweis aus den Session-Ergebnissen prüfen.
3. Unabhängiger Reviewer-Agent über den Sprint-Diff (SRV/DEV, Protokollnahe).
4. Abnahme-Abschnitt der Sprint-README füllen, PR nach `develop` öffnen (erst nach vollständigem Push).
5. RS3 (Orakel und Skelett) aus B-736 als Sprint anlegen und bereit machen, wenn Ralf an G1 »weiter« entschieden hat.

## Fertig, wenn

- [ ] Alle Sprint-Kriterien (AC-01 bis AC-05) mit Nachweis; AC-05 ggf. `angenommen, Validierung offen (RS2.5)`.
- [ ] CI-Voraussetzung: `task check`, `task check:go` grün.

## Prüfen

```bash
task check
task check:go
task check:race
```

Nacheinander (Review-Session).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.

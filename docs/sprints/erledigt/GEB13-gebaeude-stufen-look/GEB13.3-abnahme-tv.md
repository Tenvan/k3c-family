# GEB13.3 · Abnahme der Looks am TV

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** geb13/3-abnahme-tv
- **Abhängig von:** –
- **Tickets:** B-568, B-570, B-571
- **Kriterien:** AC-04

## Ziel

🧑 nimmt Pappel, Camp und Heilplatz am TV ab.

## Kontext

B-568, B-570, B-571.

## Erlaubte Dateien

- keine

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Spiel starten, Plantage, Camp in drei Biomen und Heilplatz in Stufe 1–5 ansehen.

## Fertig, wenn

- [x] AC-04: 🧑 hat die Looks abgenommen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-04 geprüft** (Ralf, 2026-10-24): Plantage-Pappel (B-568), Camp in drei Biomen (B-570) und Heilplatz Stufe 1–5 (B-571) ok. Gerät nicht genannt (angelegt als TV-Abnahme); Abweichungen auf anderen Geräten kommen als Ticket.

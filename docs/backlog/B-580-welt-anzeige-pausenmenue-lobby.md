# B-580 · Die Welt steht groß im Pausenmenü und bei jedem Spielstand in der Lobby

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei der Abnahme BD5.4 (2026-10-24, PC) fiel auf: Die Welt-Anzeige im Pausenmenü ist klein, und in der Lobby steht die Welt nicht bei jedem Spielstand.

## Ziel

Die Welt ist im Pausenmenü gut lesbar und in der Lobby bei jedem Spielstand sichtbar.

## Beteiligte und Zielgruppen

Spieler; 🧑 nimmt ab.

## Anforderungen

- Welt-Anzeige im Pausenmenü größer (Mindest-Schrift laut `docs/rules/bedienung.md` § 2 einhalten).
- Jeder Spielstand in der Lobby zeigt seine Welt.

## Nicht-Ziele

Neue Welt-Auswahl.

## Regeln und Einschränkungen

Nur zeichnen (ADR 001), Texte über `t()`, Datei ≤ 400 Zeilen.

## Beispiele

Lobby mit drei Spielständen → jede Zeile nennt ihre Welt.

## Ausnahme- und Fehlerfälle

Spielstand ohne bekannte Welt → Platzhalter „–“.

## Akzeptanzkriterien

- **AC-01** Pausenmenü zeigt die Welt in größerer Schrift (Beobachtung am PC).
- **AC-02** Jeder Spielstand der Lobby nennt seine Welt (Beobachtung am PC).

## Offene Fragen

keine

## Notizen

Aus BD5.4 (Ralf, 2026-10-24). Spec: Entwurf.

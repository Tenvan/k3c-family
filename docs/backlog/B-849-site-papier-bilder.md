# B-849 · Die Präsentationsseite site/ zeigt Papierschnitt-Bilder statt Pixel-Grafik

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Teil von B-823 (Entscheidung 009, Punkt 4). `site/` bleibt, zeigt aber noch Pixel-Grafik (`site/index.html:52-113`, Bilder in `site/img/`).

## Ziel

`site/` zeigt Papierschnitt-Bilder des nativen Spiels.

## Beteiligte und Zielgruppen

🧑 Ralf (Freigabe, Auswahl der Bilder); Claude (Umsetzung).

## Anforderungen

- Bilder aus den Papierschnitt-Assets oder Aufnahmen des nativen Spiels nach `site/img/`.
- Texte nennen das native Spiel am PC.

## Nicht-Ziele

Neues Seitenlayout.

## Regeln und Einschränkungen

Domäne PLAT: Freigabe durch 🧑.

- **Bereich:** `site/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Startseite zeigt Monarch und Burg im Papierschnitt.

## Ausnahme- und Fehlerfälle

nicht relevant: statische Seite.

## Akzeptanzkriterien

- **AC-01** `site/` ohne Pixel-Grafik, Sichtprüfung durch 🧑.

## Offene Fragen

Welche Bilder? Entscheidet 🧑 bei der Freigabe.

## Notizen

Zerlegung B-823 am 2026-11-13 (Claude).

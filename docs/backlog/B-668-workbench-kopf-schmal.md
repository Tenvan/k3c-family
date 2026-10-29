# B-668 · Der Kopf der Workbench bricht in schmalen Fenstern um, statt die Seite waagrecht zu verbreitern

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

Der gemeinsame Kopf der Workbench (Reiter Dienste & Logs bis Balancing, „Asset-Studio öffnen“, MCP-Marke, Mock-Marke, Dark-Schalter, Ticket) ist bei 600 px Fensterbreite 694 px breit (gemessen 2026-10-29 im Browser-Pane mit `?mock=1`, Balancing-Seite). Die Seite darunter passt (Balancing-Rahmen 560 px, WZ32.2), der Kopf verbreitert aber die ganze Seite und erzeugt einen waagrechten Scrollbalken.

## Ziel

Die Workbench läuft in schmalen Fenstern (halbes Fenster, Web-Client auf dem Notebook) ohne waagrechten Überlauf.

## Beteiligte und Zielgruppen

Ralf am PC und im Web-Client (B-614); Agent DEV.

## Anforderungen

- Unter etwa 760 px rollen die Reiter des Kopfs in sich (waagrecht scrollbar oder als Auswahl), die Marken und Knöpfe rechts brechen um oder wandern in ein Menü.
- `document.documentElement.scrollWidth` = Fensterbreite auf jeder Seite ab 600 px.

## Nicht-Ziele

Inhalt der einzelnen Seiten (eigene Tickets, z. B. B-658, B-659).

## Regeln und Einschränkungen

Kopf liegt in `tools/webkit/organisms/AppHeader.tsx` und gilt für Workbench, Studio und Starter; Atomic Design über `@webkit`.

## Beispiele

Workbench bei 600 px → Kopf zweizeilig oder mit rollenden Reitern, kein waagrechter Scrollbalken.

## Ausnahme- und Fehlerfälle

Unter 400 px: nicht relevant (kein Ziel-Gerät).

## Akzeptanzkriterien

- **AC-01** Beobachtung im Browser-Pane mit `?mock=1` bei 600 px auf Balancing, Planung und Tasks: `scrollWidth` = `innerWidth`.
- **AC-02** Vitest des Kopfs: unter der Grenze sind die Reiter in einem eigenen Scroll-Container.

## Offene Fragen

keine

## Notizen

Gefunden bei der Prüfung von WZ32.2 (Seitenrahmen der Balancing-Seite).

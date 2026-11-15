# tools/webkit/organisms/

## Responsibility
Organismen (`@webkit/organisms`): eigenständige Bereiche einer Seite aus Atomen und Molekülen – Kopfzeile, Facetten-Leiste, Abspielleiste und der Ticket-Baustein (Knopf und Dialog). Radix Themes liefert dazu Dialoge, Menüs, Reiter und Tabellen.

## Design
- `index.ts`: Re-Export von Radix `AlertDialog`, `ContextMenu`, `Dialog`, `DropdownMenu`, `TabNav`, `Table`, `Tabs` und der eigenen Organismen.
- `AppHeader.tsx`: Kopfzeile eines Wails-Programms (B-064): Titel, Reiter (`HeaderTabs<P>`: `pages` in Anzeige-Reihenfolge und `labels`; steht in einem `Tabs.Root`, das `AppFrame` setzt), rechte Leiste; zeigt `TicketButton`, wenn `useTicketOptional()` einen Provider findet.
- `TicketButton.tsx`: Knopf „Ticket“, öffnet `TicketDialog` (braucht `TicketProvider`).
- `TicketDialog.tsx`: Dialog „Ticket anlegen“: Eingabe (Titel, Beschreibung, Typ, Prio, Domäne aus den Listen in `lib/ticket.ts`; Projekt aus `defaultProject` des Providers), sichtbarer Kontext der Seite, letzte Tickets; ruft `create`/`recent` aus `useTicket`; bei Fehler (`errorText`) bleibt die Eingabe stehen, Schließen setzt Ergebnis und Fehler zurück.
- `FacetRail.tsx` (B-677): Facetten links, je Gruppe Markierfelder mit Zähler; lange Listen zeigen erst die ersten und gewählten Optionen, „Zurücksetzen“ erscheint bei Auswahl.
- `TransportBar.tsx` (B-677): eine Abspielleiste je Seite mit Stopp, Überblenden und Lautstärke; zeigt, was gerade spielt (leer = Stille); Tastenkürzel hängt die Seite an.

## Flow
`templates/AppFrame` → `AppHeader` → (mit Provider) `TicketButton` → `TicketDialog` → `useTicket().create(input)` → Go-Binding `TicketCreate` der Workbench → Ergebnis und `recent()` im Dialog.

## Integration
- Importiert `../atoms`, `../molecules`, `../lib`; Konsument `templates/AppFrame.tsx` und Seiten der Workbench (`FacetRail`/`TransportBar` in `src/assets/catalog/CatalogPage.tsx` und `src/assets/review/index.tsx`).
- Abhängigkeiten: `react`, `@radix-ui/themes`.
- Test: `TicketDialog.test.tsx`.

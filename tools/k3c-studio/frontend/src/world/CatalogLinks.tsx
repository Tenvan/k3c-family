import { Flex, LinkButton, Text } from '@webkit/atoms';
import { navigateTo } from '@webkit/lib/navigate';
import { formatWeltHash, WELT_VORGABE, type WeltModus } from '@webkit/lib/weltFilter';

/** Katalog der Welt öffnen: setzt den Welt-Filter in der Adresse und wechselt auf die Seite Katalog (WZ62.4, B-1171). */
export function openCatalog(welt: string, modus: WeltModus): void {
  try {
    window.location.hash = formatWeltHash({ ...WELT_VORGABE, welt, modus }, window.location.hash);
  } catch {
    // Adresse nicht beschreibbar: der Katalog öffnet ohne Filter
  }
  navigateTo('katalog');
}

/** Zwei Schnell-Links im Kopf des Welten-Editors: Assets nur dieser Welt (exklusiv) oder samt geerbten (gesamt). */
export function CatalogLinks({ welt }: { welt: string }) {
  return (
    <Flex gap="2" align="center" wrap="wrap" role="group" aria-label="Assets der Welt im Katalog">
      <Text size="1" color="gray">Katalog:</Text>
      <LinkButton size="1" data-katalog="exklusiv" onClick={() => openCatalog(welt, 'exklusiv')}>nur diese Welt (exklusiv)</LinkButton>
      <LinkButton size="1" data-katalog="gesamt" onClick={() => openCatalog(welt, 'gesamt')}>mit geerbten (gesamt)</LinkButton>
    </Flex>
  );
}

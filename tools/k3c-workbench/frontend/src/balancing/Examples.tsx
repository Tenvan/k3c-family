import { Button, Flex, Text } from '@webkit/atoms';
import { Dialog } from '@webkit/organisms';
import { EXAMPLES, type Example } from './help';

/** Knopf „Beispiele“ mit modalem Popup (WZ28.10, B-609): typische Läufe; „Übernehmen“ setzt sie in die Konfiguration
 *  und schließt das Popup. */
export function Examples({ onPick }: { onPick: (e: Example) => void }) {
  return (
    <Dialog.Root>
      <Dialog.Trigger>
        <Button size="1" variant="soft">Beispiele …</Button>
      </Dialog.Trigger>
      <Dialog.Content maxWidth="640px" className="bal-modal" aria-describedby={undefined}>
        <Dialog.Title>Beispiele</Dialog.Title>
        <section className="bal-files bal-modal-body" aria-label="Beispiele">
          {EXAMPLES.map((e) => (
            <div key={e.name} className="bal-example">
              <div>
                <Text size="2" weight="medium">{e.name}</Text>
                <Text as="p" size="1" color="gray">{e.text}</Text>
              </div>
              <Dialog.Close>
                <Button size="1" variant="soft" aria-label={`Beispiel übernehmen: ${e.name}`} onClick={() => onPick(e)}>Übernehmen</Button>
              </Dialog.Close>
            </div>
          ))}
        </section>
        <Flex justify="end" mt="4">
          <Dialog.Close>
            <Button variant="soft" color="gray">Schließen</Button>
          </Dialog.Close>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}

import { Button, Flex, Text } from '@webkit/atoms';
import { Dialog } from '@webkit/organisms';
import type { BalanceBaseline } from '../api';
import { EXAMPLES, type Example } from './help';

interface Props {
  onPick: (e: Example) => void;
  /** Gesicherte Basis-Läufe (B-640), nach Name. */
  baselines: BalanceBaseline[];
  /** Basis-Läufe übernehmen: setzt Konfiguration und Baseline. */
  onPickBaseline: (b: BalanceBaseline) => void;
  /** Das Popup wurde geöffnet (Basis-Läufe neu laden). */
  onOpen?: () => void;
}

const dateOf = (iso: string) => (iso.length >= 10 ? iso.slice(0, 10) : iso);

function baselineText(b: BalanceBaseline): string {
  const parts = [`${b.run}, ${dateOf(b.saved)}`, `${b.seeds} Seed${b.seeds === 1 ? '' : 's'} (${b.seedMode})`, b.bots.join(', '), b.worlds.join(', ')];
  const frozen = b.frozen.length > 0 ? ` · eingefroren: ${b.frozen.join(', ')}` : '';
  return `${parts.filter(Boolean).join(' · ')} · ${b.pass ? 'durchgelaufen' : 'nicht erreicht'}${frozen}`;
}

/** Abschnitt „Basis-Läufe“ (B-640): gesicherte, durchgelaufene Läufe; „Übernehmen“ setzt Konfiguration und Baseline. */
function BaselineList({ baselines, onPick }: { baselines: BalanceBaseline[]; onPick: (b: BalanceBaseline) => void }) {
  return (
    <div aria-label="Basis-Läufe" role="group">
      <Text as="p" size="2" weight="bold">Basis-Läufe</Text>
      {baselines.length === 0 && <Text as="p" size="1" color="gray">Noch keine Basis-Läufe. Sichern mit sim_test (action baseline) nach einem durchgelaufenen Lauf.</Text>}
      {baselines.map((b) => (
        <div key={b.name} className="bal-example">
          <div>
            <Text size="2" weight="medium">{b.name}</Text>
            <Text as="p" size="1" color="gray">{baselineText(b)}</Text>
          </div>
          <Dialog.Close>
            <Button size="1" variant="soft" aria-label={`Basis-Lauf übernehmen: ${b.name}`} onClick={() => onPick(b)}>Übernehmen</Button>
          </Dialog.Close>
        </div>
      ))}
    </div>
  );
}

/** Knopf „Beispiele“ mit modalem Popup (WZ28.10, B-609): Basis-Läufe und typische Läufe; „Übernehmen“ setzt sie in die
 *  Konfiguration und schließt das Popup. */
export function Examples({ onPick, baselines, onPickBaseline, onOpen }: Props) {
  return (
    <Dialog.Root onOpenChange={(open) => open && onOpen?.()}>
      <Dialog.Trigger>
        <Button size="1" variant="soft">Beispiele …</Button>
      </Dialog.Trigger>
      <Dialog.Content maxWidth="640px" className="bal-modal" aria-describedby={undefined}>
        <Dialog.Title>Beispiele</Dialog.Title>
        <section className="bal-files bal-modal-body" aria-label="Beispiele">
          <BaselineList baselines={baselines} onPick={onPickBaseline} />
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

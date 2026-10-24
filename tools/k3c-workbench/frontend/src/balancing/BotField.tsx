import { Flex, SelectField, Text } from '@webkit/atoms';
import { HelpField } from './HelpButton';
import { CHOICES, isMix, MIX_SEP, mixCount, PER_PLAYER, resizeMix, SOLO_BOTS } from './help';
import type { BalanceForm } from './spec';

const SOLO = SOLO_BOTS.map((b) => ({ value: b, label: b }));

/** Feld „Bot“ (WZ28.12, B-611): ein Profil für alle Spieler oder „je Spieler“ mit einer Auswahl je Spieler. */
export function BotField({ form, onForm }: { form: BalanceForm; onForm: (f: BalanceForm) => void }) {
  const n = mixCount(form.players);
  const mix = isMix(form.bot);
  const known = CHOICES.bot.some((o) => o.value === form.bot);
  const options = CHOICES.bot.map((o) => (o.value === PER_PLAYER ? { ...o, disabled: n < 2 } : o));
  if (!mix && !known) options.push({ value: form.bot, label: form.bot });
  const pick = (v: string) => onForm({ ...form, bot: v === PER_PLAYER ? resizeMix('saver', n) : v });
  const parts = mix ? resizeMix(form.bot, n).split(MIX_SEP) : [];
  const setPart = (i: number) => (v: string) => onForm({ ...form, bot: parts.map((p, j) => (j === i ? v : p)).join(MIX_SEP) });
  return (
    <HelpField name="bot" hint={false}>
      <SelectField aria-label="Bot" value={mix ? PER_PLAYER : form.bot} options={options} onChange={pick} />
      {mix && (
        <Flex direction="column" gap="1" className="bal-mix">
          {parts.map((b, i) => (
            <Flex key={i} align="center" gap="2">
              <Text size="1" color="gray">Spieler {i + 1}</Text>
              <SelectField aria-label={`Bot Spieler ${i + 1}`} size="1" value={b} options={SOLO} onChange={setPart(i)} />
            </Flex>
          ))}
        </Flex>
      )}
    </HelpField>
  );
}

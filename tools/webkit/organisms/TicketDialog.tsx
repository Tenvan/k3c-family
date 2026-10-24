import { Dialog } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { ActionButton, Button, Flex, SelectField, Text, TextArea, TextField, plainOptions } from '../atoms';
import { errorText } from '../lib/errors';
import { TICKET_DOMAINS, TICKET_PRIOS, TICKET_TYPES, type TicketCreated, type TicketDomain, type TicketPrio, type TicketType } from '../lib/ticket';
import { useTicket } from '../lib/ticketContext';
import { NoticeCard } from '../molecules';

/** Dialog „Ticket anlegen“: Eingabe, sichtbarer Kontext, letzte Tickets. Bei einem Fehler bleibt die Eingabe stehen. */
export function TicketDialog({ open, onOpenChange }: { open: boolean; onOpenChange: (open: boolean) => void }) {
  const { app, version, defaultDomain, defaultProject, create, recent, page } = useTicket();
  const [titel, setTitel] = useState('');
  const [beschreibung, setBeschreibung] = useState('');
  const [typ, setTyp] = useState<TicketType>('Problem');
  const [prio, setPrio] = useState<TicketPrio>('mittel');
  const [domaene, setDomaene] = useState<TicketDomain>(defaultDomain);
  const [error, setError] = useState('');
  const [done, setDone] = useState<TicketCreated | null>(null);
  const [latest, setLatest] = useState<TicketCreated[]>([]);

  useEffect(() => {
    if (open) return;
    setDone(null);
    setError('');
  }, [open]);

  useEffect(() => {
    if (!open || !recent) return;
    recent().then(setLatest, () => setLatest([]));
  }, [open, recent, done]);

  const send = async () => {
    setError('');
    try {
      const made = await create({
        titel: titel.trim(),
        beschreibung,
        typ,
        prio,
        domaene,
        projekt: defaultProject,
        kontext: { app, seite: page.page, auswahl: page.selection, version, zeit: new Date().toISOString() },
      });
      setDone(made);
      setTitel('');
      setBeschreibung('');
    } catch (e) {
      setDone(null);
      setError(errorText(e));
    }
  };

  const context = [app, page.page, page.selection, version].filter(Boolean).join(' · ');
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Content maxWidth="560px" aria-describedby={undefined}>
        <Dialog.Title>Ticket anlegen</Dialog.Title>
        <Flex direction="column" gap="3">
          <Text size="1" color="gray" data-testid="ticket-context">
            Kontext: {context}
          </Text>
          <TextField.Root aria-label="Titel" placeholder="Titel" value={titel} onChange={(e) => setTitel(e.target.value)} />
          <TextArea aria-label="Beschreibung" placeholder="Beschreibung" rows={5} value={beschreibung} onChange={(e) => setBeschreibung(e.target.value)} />
          <Flex gap="3">
            <SelectField aria-label="Typ" value={typ} options={plainOptions(TICKET_TYPES)} onChange={setTyp} />
            <SelectField aria-label="Priorität" value={prio} options={plainOptions(TICKET_PRIOS)} onChange={setPrio} />
            <SelectField aria-label="Domäne" value={domaene} options={plainOptions(TICKET_DOMAINS)} onChange={setDomaene} />
          </Flex>
          {error && <NoticeCard title="Fehler" tone="error">{error}</NoticeCard>}
          {done && <NoticeCard title="Angelegt" tone="ok">{done.id} · {done.titel}</NoticeCard>}
          {latest.length > 0 && (
            <Flex direction="column" gap="1" aria-label="Letzte Tickets">
              <Text size="1" weight="bold">Letzte Tickets</Text>
              {latest.map((t) => (
                <Text key={t.id} size="1">{t.id} · {t.titel}</Text>
              ))}
            </Flex>
          )}
          <Flex gap="3" justify="end">
            <Button variant="soft" color="gray" onClick={() => onOpenChange(false)}>Schließen</Button>
            <ActionButton disabled={titel.trim() === ''} onClick={send}>Anlegen</ActionButton>
          </Flex>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}

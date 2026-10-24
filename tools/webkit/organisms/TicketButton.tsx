import { useState } from 'react';
import { Button } from '../atoms';
import { TicketDialog } from './TicketDialog';

/** Knopf „Ticket“: öffnet den Dialog zum Anlegen eines Tickets (braucht TicketProvider). */
export function TicketButton() {
  const [open, setOpen] = useState(false);
  return (
    <>
      <Button variant="soft" size="1" onClick={() => setOpen(true)}>
        Ticket
      </Button>
      <TicketDialog open={open} onOpenChange={setOpen} />
    </>
  );
}

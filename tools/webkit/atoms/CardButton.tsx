import { Card } from '@radix-ui/themes';
import type { ComponentProps } from 'react';

type Props = Omit<ComponentProps<'button'>, 'type'> & { variant?: ComponentProps<typeof Card>['variant'] };

/** Anklickbare Karte (Zeile einer Liste, Kachel, Hinweis): Radix Card als Knopf, mit Hover und Fokus aus dem Theme. */
export function CardButton({ variant = 'surface', ...rest }: Props) {
  return (
    <Card asChild variant={variant}>
      <button type="button" {...rest} />
    </Card>
  );
}

import { Link } from '@radix-ui/themes';
import type { ComponentProps } from 'react';

type Props = Omit<ComponentProps<'button'>, 'type'> & Pick<ComponentProps<typeof Link>, 'size' | 'color' | 'underline'>;

/** Knopf im Fließtext, der wie ein Link aussieht (z. B. „erneut prüfen“, „alle einklappen“). */
export function LinkButton({ size = '2', color, underline, ...rest }: Props) {
  return (
    <Link asChild size={size} color={color} underline={underline}>
      <button type="button" {...rest} />
    </Link>
  );
}

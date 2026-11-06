import { Slider as RadixSlider } from '@radix-ui/themes';
import type { ComponentProps } from 'react';

type RootProps = Omit<ComponentProps<typeof RadixSlider>, 'value' | 'defaultValue' | 'onValueChange' | 'onChange' | 'min' | 'max' | 'step'>;

interface Props extends RootProps {
  value: number;
  min: number;
  max: number;
  step?: number;
  onChange: (value: number) => void;
}

/** Schieber für genau einen Zahlenwert (Radix Slider), z. B. Frame-Wahl oder Überblendung im Asset-Review.
 *  `aria-label` sitzt an der Wurzel um den Griff (Radix Themes reicht es nicht an den Griff weiter); Änderungen kommen
 *  als Zahl, nie außerhalb von min … max. Der rohe Radix-`Slider` (Werte als Liste) bleibt daneben ein Atom. */
export function ValueSlider({ value, min, max, step = 1, onChange, 'aria-label': label, ...rest }: Props) {
  return (
    <RadixSlider {...rest} value={[value]} min={min} max={max} step={step} aria-label={label}
      onValueChange={(v) => {
        if (v[0] !== undefined) onChange(v[0]);
      }} />
  );
}

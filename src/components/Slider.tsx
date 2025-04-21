import { useId } from "react";

export interface Props {
  min: number;
  max: number;
  step?: number;
  value: number;
  onChange: (value: number) => void;
  unit?: string;
  children: any;
}

export default function Slider({
  min,
  max,
  step,
  value,
  onChange,
  children,
  unit,
}: Props) {
  const id = useId();
  return (
    <div className="mb-4 grid grid-cols-3">
      <label
        htmlFor={id}
        className="mb-2 text-sm col-span-2 font-medium text-gray-900 dark:text-white"
      >
        {children}
      </label>
      <span className="mb-2 text-sm col-span-1 font-medium text-gray-900 dark:text-white text-right">
        {value} {unit && `${unit}`}
      </span>
      <input
        id={id}
        type="range"
        min={min}
        max={max}
        step={step}
        className="col-span-3 h-3 bg-gray-200 rounded-lg cursor-pointer range-lg dark:bg-gray-700 accent-blue-600"
        value={value}
        onChange={(evt) => {
          const v = parseFloat(evt?.target?.value);
          if (typeof v == 'number') onChange(v)
        }}
      />
    </div>
  );
}

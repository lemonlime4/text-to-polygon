import { readable } from "svelte/store";

const counter = (() => {
  let count = 1;
  return () => (count++).toString();
})();
export const defaultExpressions = [
  {
    type: "folder",
    id: "points-folder",
    readonly: true,
    title: "P (keep closed to reduce lag)",
    collapsed: true,
  },
  {
    type: "expression",
    id: "points",
    folderId: "points-folder",
    readonly: true,
    latex: "P=\\left(\\left[0\\infty\\right],\\left[0\\infty\\right]\\right)",
    pointSize: "3",
    color: Desmos.Colors.RED,
  },
  {
    type: "folder",
    id: "metrics-folder",
    readonly: true,
    title: "Metrics",
  },
  {
    type: "text",
    id: counter(),
    folderId: "metrics-folder",
    readonly: true,
    text: "advance",
  },
  {
    type: "expression",
    id: "advance",
    folderId: "metrics-folder",
    readonly: true,
    latex: "P_a=1\\cdot\\left(0,0\\right)",
    color: Desmos.Colors.BLUE,
  },
  {
    type: "text",
    id: counter(),
    folderId: "metrics-folder",
    readonly: true,
    text: "font size",
  },
  {
    type: "expression",
    id: "font-size",
    folderId: "metrics-folder",
    readonly: true,
    latex: "P_s=1\\cdot0",
  },
  {
    type: "expression",
    id: counter(),
    latex: "P\\left[P.x=P.x\\right].\\operatorname{count}",
  },
  {
    type: "expression",
    id: counter(),
    latex: "\\operatorname{polygon}\\left(P\\right)",
    color: Desmos.Colors.BLACK,
    fillOpacity: ".8",
    lines: false,
  },
  {
    type: "expression",
    id: counter(),
    latex: "y=P_s",
    color: Desmos.Colors.BLUE,
  },
];

export function setExpressions(
  calc: Desmos.Calculator,
  list: Record<string, unknown>[],
) {
  calc.setState({
    ...(calc.getState() as object),
    expressions: { list },
  });
}

export function toLatex(x: number): string {
  if (Number.isNaN(x)) return "0\\infty";
  if (x === Infinity) return "\\infty";
  if (x === -Infinity) return "-\\infty";
  let arr = x.toString().split("e", 2);
  if (arr.length === 1) return arr[0] as string;
  let [base, exp] = arr as [string, string];
  return `${base}\\cdot10^{${exp.slice(+exp.startsWith("+"))}}`;
}

export function numberToLatex(
  strings: TemplateStringsArray,
  ...values: number[]
) {
  return String.raw({ raw: strings }, ...values.map(toLatex));
}

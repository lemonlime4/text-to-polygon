export const defaultExpressions = [
  {
    readonly: true,
    type: "folder",
    id: "points-folder",
    title: "Points",
    collapsed: true,
  },
  {
    readonly: true,
    folderId: "points-folder",
    type: "expression",
    id: "P",
    latex: "P=\\left(\\left[0\\infty\\right],\\left[0\\infty\\right]\\right)",
    pointSize: "4",
  },
  {
    type: "expression",
    id: "1",
    latex: "P\\left[P.x=P.x\\right].\\operatorname{count}",
  },
  {
    type: "expression",
    id: "2",
    latex: "\\operatorname{polygon}\\left(P\\right)",
    color: "#000000",
    fillOpacity: ".8",
    lines: false,
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

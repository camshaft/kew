import cytoscape from "cytoscape";
import dagre from "cytoscape-dagre";

dagre(cytoscape);

export default cytoscape;

export function render(cy) {
  const style = cy.style();

  if (style) {
    style
      .selector(".queue")
      .style({
        shape: "square",
        label(el) {
          return el.data("label");
        },
      })
      .selector(".actor")
      .style({
        label(el) {
          return el.data("label");
        },
      })

      .selector("edge")
      .style({
        "target-arrow-color": "red",
        "target-arrow-shape": "triangle",
      })

      .update();
  }

  cy.layout({
    name: "dagre",
    rankDir: "RL",
  }).run();
}

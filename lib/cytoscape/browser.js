import { default as cytoscape, render } from "./index.js";

async function getElements(url) {
  const res = await fetch(url);
  const json = await res.json();
  return json;
}

async function select() {
  for (let el of document.querySelectorAll("[data-cytoscape]")) {
    const container = document.createElement("div");
    container.style.height = "100%";
    el.appendChild(container);

    const cy = cytoscape({
      container: container,
      userZoomingEnabled: false,
    });

    cy.add(await getElements(el.dataset.elements));
    render(cy);
  }
}

select();

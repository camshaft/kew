import embed from "vega-embed";

async function select() {
  for (let el of document.querySelectorAll("[data-vega]")) {
    const url = el.dataset.vega;
    const container = document.createElement("div");
    el.after(container);

    const options = {
      width: 700,
      patch: (config) => {
        config.data.forEach((data) => {
          if (!data.url.match(/^https?\:/)) {
            data.url = `${window.location.origin}${data.url}`;
          }
        });
        return config;
      },
    };

    try {
      await embed(container, url, options);
      el.remove();
    } catch (err) {
      console.error(err);
      container.remove();
    }
  }
}

select();

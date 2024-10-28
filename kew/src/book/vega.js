let vega = "https://cdn.jsdelivr.net/npm/vega@5";
let embed = "https://cdn.jsdelivr.net/npm/vega-embed@6";

import(vega)
  .then(() => import(embed))
  .then(() => {
    document.querySelectorAll("[data-vega]").forEach((el) => {
      const url = el.dataset.vega;
      const container = document.createElement("div");
      el.after(container);

      const options = {
        width: 700,
        patch: (config) => {
          config.data.forEach((data) => {
            if (!data.url.match(/^https?\:/)) {
              data.url = `${window.location.origin}/${data.url}`;
            }
          });
          return config;
        },
      };

      vegaEmbed(container, url, options)
        .then(() => {
          el.remove();
        })
        .catch((err) => {
          console.error(err);
          container.remove();
        });
    });
  });

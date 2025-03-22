import { createHash } from "crypto";
import { existsSync, writeFileSync, mkdirSync } from "fs";
import { parse as parseSql, astVisitor as visitSql } from "pgsql-ast-parser";

function writeIfNeeded(path, data) {
  if (!existsSync(path)) {
    writeFileSync(path, data);
  }
}

function sha1(contents) {
  return createHash("sha1").update(contents).digest("hex");
}

const root = "src/data/kew";

export default function keyPlugin(md) {
  wrapCodeRenderer(md.renderer, "render");
  wrapCodeRenderer(md.renderer, "renderInline");
  wrapCodeRenderer(md.renderer, "renderInlineAsText");

  if (!existsSync(root)) mkdirSync(root);
  writeIfNeeded(`${root}/.gitignore`, "*\n");
}

function parseInfo(info) {
  const [_lang, ...args] = info.split(" ");
  const out = new Map();
  for (let arg of args) {
    if (!arg) continue;
    const [key, value] = arg.split("=");
    out.set(key, value);
  }
  return out;
}

function replaceSqlToken(token) {
  const query = token.content;
  const ast = parseSql(query);
  const tables = [];
  const visitor = visitSql((_v) => ({
    tableRef: (t) => tables.push(t.name),
  }));

  for (let stmt of ast) {
    visitor.statement(stmt);
  }

  const info = parseInfo(token.info);
  const hash = sha1(query);
  const contentName = info.get("id") || `_sql_${hash}`;

  const updateName = `_sql_${hash}_update`;
  const updateTables = `_sql_${hash}_tables`;
  const sqlName = `_sql_${hash}_sql`;

  const tokens = [];

  tokens.push({
    ...token,
    info: "js",
    content: `
    import {Mutable} from "observablehq:stdlib";

    // TODO prerender query
    const ${contentName} = Mutable();

    const ${updateName} = (arrow) => {
      ${contentName}.value = arrow;
    };
    `,
  });

  tokens.push({
    ...token,
    info: "js",
    content: `
    import {Mutable} from "observablehq:stdlib";
    import {DuckDBClient} from "npm:@observablehq/duckdb";
    import * as Arrow from "npm:apache-arrow";

    const ${sqlName} = Mutable();

    let client = await DuckDBClient.of({});
    let prevTables = {};
    const ${updateTables} = async (tables) => {
      let loads = Object.entries(tables).map(async ([name, table]) => {
        await insertArrowTable(client._db, name, table, { create: true });
      });

      async function insertArrowTable(database, name, table, options) {
        const connection = await database.connect();
        try {
          if (prevTables[name]) await connection.send(\`DROP TABLE IF EXISTS \${name}\`);
          await connection.insertArrowTable(table, {
            name,
            schema: "main",
            ...options
          });
          prevTables[name] = true;
        } finally {
          await connection.close();
        }
      }

      await Promise.all(loads);

      ${sqlName}.value = client.sql.bind(client);
    };
    `,
  });

  tokens.push({
    ...token,
    info: "js",
    content: `
    ${updateTables}({ ${tables.join(", ")} });
    `,
  });

  tokens.push({
    ...token,
    info: "js",
    content: `
    let sql = ${sqlName};
    if (sql) ${updateName}(await sql\`${query}\`);
    `,
  });

  maybePushDisplay(tokens, token, contentName, info);

  return tokens;
}

function replaceSim(token, info) {
  const config = token.content;
  const hash = sha1(config);
  const path = `${root}/${hash}.js`;
  writeIfNeeded(path, config);

  const attachment = `FileAttachment('./data/sim-${hash}.arrow').arrow()`;

  const setupName = `_sim_${hash}_instance`;

  const updateName = `_sim_${hash}_update`;

  const contentName = info.get("id") || `_sim_${hash}`;

  const tokens = [];

  tokens.push({
    ...token,
    info: "js",
    content: `
    let sim = ${setupName};
    ${config};
    `,
  });

  tokens.push({
    ...token,
    info: "js",
    content: `
    import {Mutable} from "observablehq:stdlib";

    const ${contentName} = Mutable(await ${attachment});

    const ${updateName} = (arrow) => {
      ${contentName}.value = arrow;
    };
    `,
  });

  tokens.push({
    ...token,
    info: "js",
    content: `
    import { Kew } from './components/kew.js';
    import * as Arrow from "npm:apache-arrow";

    let kew = new Kew();

    const ${setupName} = (config) => {
      config = typeof config == 'function' ? config() : config;
      kew.update(config);
      ${updateName}(Arrow.tableFromIPC(kew.to_arrow()));
    };
    `,
  });

  maybePushDisplay(tokens, token, contentName, info);

  return tokens;
}

function maybePushDisplay(tokens, token, name, info) {
  if (!info.has("id") || info.has("display")) {
    tokens.push({
      ...token,
      info: "js",
      content: `
      display(Inputs.table(${name}))
      `,
    });
  }
}

function replaceToken(token) {
  if (token.type != "fence" || token.tag != "code") return token;

  if (token.info.startsWith("sql")) return replaceSqlToken(token);

  if (token.info.startsWith("js")) {
    let info = parseInfo(token.info);
    if (info.has("sim")) return replaceSim(token, info);
  }

  return token;
}

function wrapCodeRenderer(ctx, name) {
  if (!ctx[name]) return;

  const renderer = ctx[name].bind(ctx);

  ctx[name] = function (tokens, ...args) {
    const newTokens = [];
    for (let token of tokens) {
      try {
        const t = replaceToken(token);
        if (Array.isArray(t)) newTokens.push(...t);
        else newTokens.push(t);
      } catch (error) {
        const content = `\nthrow new Error(${JSON.stringify("" + error)})\n`;
        newTokens.push({
          ...token,
          type: "fence",
          info: "js",
          content,
          toString() {
            return content;
          },
        });
      }
    }
    const out = renderer(newTokens, ...args);
    return out;
  };
}

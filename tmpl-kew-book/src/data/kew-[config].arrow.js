import { readFile, writeFile } from "fs/promises";
import { Kew } from "../components/kew.js";
import * as _duckdb from "duckdb";
const { Database } = _duckdb.default;

const configHash = process.argv[2].replace("--config=", "");

const configFile = await readFile(`src/data/kew/${configHash}.json`);
const { sql: sqlParts, configs } = JSON.parse(configFile);

let sql = sqlParts[0];
const tables = [];
const toLoad = [];

let kew;
for (let i = 0; i < configs.length; i++) {
  const config = JSON.stringify(configs[i]);
  if (!kew) kew = new Kew(config);
  else kew.update(config);

  const data = kew.to_parquet();

  const table = `tbl${i}`;
  tables.push(table);
  sql += ` ${table} `;
  sql += sqlParts[i + 1];

  toLoad.push({ table, data });
}

async function asyncdb(cb) {
  return await new Promise((resolve, reject) => {
    cb((err, value) => {
      err ? reject(err) : resolve(value);
    });
  });
}

let db = new Database(":memory:");

await asyncdb((cb) => db.exec(`INSTALL arrow; LOAD arrow;`, cb));

for (let { data, table } of toLoad) {
  // TODO use this once they fix it
  // await asyncdb((cb) => db.register_buffer(table, [data], true, cb));

  const path = `src/data/kew/${configHash}_${table}.parquet`;
  await writeFile(path, data);
  await asyncdb((cb) =>
    db.exec(
      `CREATE TEMPORARY VIEW ${table} AS SELECT * FROM read_parquet('${path}');`,
      cb
    )
  );
}

const stream = await asyncdb((cb) => db.arrowIPCAll(sql, cb));
for (let chunk of stream) {
  process.stdout.write(chunk);
}

import { DuckDBClient } from "npm:@observablehq/duckdb";

export default async function* query(q, generator) {
  const tbl = "tbl";
  let client;
  for await (let res of generator) {
    if (!client) {
      client = await DuckDBClient.of({ [tbl]: res });
    } else {
      console.log("updating table");
      await client._db.insertArrowFromIPCStream(tbl, res);
    }

    const sql = q.join(` ${tbl} `);
    const out = await client.query(sql);

    console.log("query", out, out.numRows);

    yield out;
  }
}

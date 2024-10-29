use super::*;

pub fn render<P: AsRef<Path>>(p: P) {
    let p = p.as_ref();
    let elements = element_json(p);

    // TODO make this a plugin
    md("");
    md(format_args!(
        "<div data-cytoscape data-elements=\"{}\"></div>",
        elements.file_name().unwrap().to_str().unwrap()
    ));
    md("");
}

pub fn element_json<P: AsRef<Path>>(p: P) -> PathBuf {
    let p = p.as_ref().display();

    sql_json(format_args!(
        "
    SELECT DISTINCT
        'nodes' AS \"group\",
        to_json({{
            id: 'actor_' || attr_actor
        }}) AS data,
        to_json([ 'actor' ]) AS classes
    FROM read_parquet('{p}')
    WHERE
        name == 'pop' OR name == 'push'

    UNION

    SELECT DISTINCT
        'nodes' AS \"group\",
        to_json({{
            id: 'queue_' || attr_queue_name
        }}) AS data,
        to_json([ 'queue' ]) AS classes
    FROM read_parquet('{p}')
    WHERE
        name == 'pop' OR name == 'push'

    UNION

    SELECT DISTINCT
        'edges' AS \"group\",
        to_json({{
            source: 'queue_' || attr_queue_name,
            target: 'actor_' || attr_actor
        }}) AS data,
        to_json([ name ]) AS classes
    FROM read_parquet('{p}')
    WHERE
        name == 'push'
    
    UNION

    SELECT DISTINCT
        'edges' AS \"group\",
        to_json({{
            source: 'actor_' || attr_actor,
            target: 'queue_' || attr_queue_name
        }}) AS data,
        to_json([ name ]) AS classes
    FROM read_parquet('{p}')
    WHERE
        name == 'pop'
    "
    ))
}

use std::sync::LazyLock;

use redb::{ReadableTable, ReadableTableMetadata, TableDefinition};

static TABLE_NAME: &str = "INDEX";

static DB: LazyLock<redb::Database> = LazyLock::new(|| {
    let _ = std::fs::create_dir_all(".cache");
    let _ = std::fs::remove_file(".cache/index.redb");
    redb::Database::builder()
        .create(".cache/index.redb")
        .unwrap()
});

pub type CacheValue = (String, String);

fn table_definition() -> redb::TableDefinition<'static, std::string::String, CacheValue> {
    TableDefinition::new(TABLE_NAME)
}

pub fn index_clear() -> anyhow::Result<()> {
    let trans = DB.begin_write()?;
    let table = trans.open_table(table_definition())?;
    trans.delete_table(table)?;
    trans.commit()?;

    Ok(())
}

pub fn index_insert(k: String, v: CacheValue) -> anyhow::Result<()> {
    let trans = DB.begin_write()?;
    {
        let mut table = trans.open_table(table_definition())?;
        table.insert(k, v)?;
    }
    trans.commit()?;
    Ok(())
}

pub fn index_list() -> anyhow::Result<Vec<(String, CacheValue)>> {
    let mut res = vec![];
    let trans = DB.begin_read()?;
    let table = trans.open_table(table_definition())?;
    res.reserve(table.len()? as usize);
    let v = table.iter()?;
    for i in v {
        let i = i?;
        let k = i.0.value();
        let v = i.1.value();
        res.push((k, v));
    }

    Ok(res)
}

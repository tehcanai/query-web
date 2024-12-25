use datafusion::arrow::{record_batch, util};
use datafusion::prelude::*;

use record_batch::RecordBatch;

pub async fn query_csv(query: &str) -> datafusion::error::Result<String> {
    let ctx = SessionContext::new();
    ctx.register_csv("uploaded", "tmp/uploaded.csv", CsvReadOptions::new())
        .await?;

    let df = ctx.sql(query).await?;

    let results: Vec<RecordBatch> = df.collect().await?;

    let pretty_results = util::pretty::pretty_format_batches(&results)?.to_string();

    Ok(pretty_results)
}

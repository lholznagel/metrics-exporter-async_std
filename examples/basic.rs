use metrics_exporter_http_async_std::HttpExporter;
use metrics_runtime::{Receiver, observers::YamlBuilder};

#[async_std::main]
async fn main() -> std::io::Result<()> {

    let receiver = Receiver::builder().build().expect("failed to create receiver");
    let exporter = HttpExporter::new(
        receiver.controller(),
        YamlBuilder::new(),
        "127.0.0.1:9999".parse().unwrap()
    );

    let mut sink = receiver.sink();
    sink.update_gauge("MyGauge", 0);

    // or as a task:
    // use async_std::task;
    // task::spawn(async { exporter.async_run().await.unwrap(); });
    exporter.async_run().await.unwrap();

    Ok(())
}
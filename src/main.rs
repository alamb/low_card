// (arrow_dev) alamb@MacBook-Pro-6:~/Software/influxdb_iox$ LOG_FILTER=ingester=debug,info cargo run    -- run all-in-one --max-http-request-size=1000000000 --persist-partition-size-threshold-bytes=200000000

use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};

use futures::{stream::FuturesUnordered, StreamExt};
use reqwest::Response;

/// number of concurrent clients sending data
const NUM_CLIENTS: usize = 20;

/// Number of low cardinality tags written
const NUM_TAGS: usize = 10;

/// Number of distinct measurements written
const NUM_MEASUREMENTS: usize = 1;

/// Number lines of line protocol in each request
const LINES_PER_REQUEST: usize = 100000;

#[tokio::main]
async fn main() {
    println!("Hello, starting....");

    // use one generator, results in no duplicates
    let generator = Arc::new(LineProtoGenerator::new());

    // fire it up
    println!("starting clients...");

    let tasks = (0..NUM_CLIENTS)
        .map(|_| {
            // sending dupliated data results means the ingester can keep up
            //let generator = Arc::new(LineProtoGenerator::new());
            tokio::task::spawn(write_task(Arc::clone(&generator)))
        })
        .collect::<FuturesUnordered<_>>();

    println!("waiting for clients...");
    let results = tasks.collect::<Vec<_>>().await;

    for res in results {
        if let Err(e) = res {
            println!("Error, client task panic'd: {}", e)
        }
    }

    println!("done");
}

// endless loop that sends data from the generator to the ingester
async fn write_task(generator: Arc<LineProtoGenerator>) {
    let client = WriteClient::new();

    for _ in 0..1000 {
        let res = client.post(generator.make_lines(LINES_PER_REQUEST)).await;

        if res.status().is_success() {
            //println!("Success!");
        } else {
            println!("Failure");
            println!("{:#?}", res);
        }
    }
}

/// wrapper that send data to IOx
#[derive(Debug)]
struct WriteClient {
    client: reqwest::Client,
}

impl WriteClient {
    fn new() -> Self {
        let client = reqwest::Client::new();

        Self { client }
    }

    async fn post(&self, data: Vec<u8>) -> Response {
        let org = "26f7e5a4b7be365b";
        let bucket = "917b97a92e883afc";
        let params = vec![("org", org), ("bucket", bucket)];

        //println!("Sending {} bytes of data...", data.len());
        self.client
            .post("http://localhost:8080/api/v2/write")
            .query(&params)
            .body(data)
            .send()
            .await
            .expect("request failed")
    }
}

/// Makes pathalogical line protocol
#[derive(Debug, Default)]
struct LineProtoGenerator {
    timestamp_generator: AtomicI64,
}

impl LineProtoGenerator {
    fn new() -> Self {
        Self::default()
    }

    /// Returns num_lines of line protocol
    fn make_lines(&self, num_lines: usize) -> Vec<u8> {
        let mut bytes = vec![];

        for i in 0..num_lines {
            self.gen_line(&mut bytes, i)
        }

        bytes
    }

    /// write a single line of output to w
    ///
    /// example output:
    /// ```text
    /// m tag0=A_FAIRLY_LONG_TAG_VALUE,tag1=A_FAIRLY_LONG_TAG_VALUE field=4 122
    ///```
    fn gen_line<W: std::io::Write>(&self, w: &mut W, i: usize) {
        let measurement_number = i % NUM_MEASUREMENTS;

        let ts = self.timestamp_generator.fetch_add(1, Ordering::Relaxed);
        write!(w, "m{}", measurement_number).unwrap();
        for tag_num in 0..NUM_TAGS  {
            write!(w, ",tag{}=A_FAIRLY_LONG_TAG_VALUE", tag_num).unwrap();
        }
        write!(w, " field=4 {}\n", ts).unwrap();
    }
}

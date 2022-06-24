// (arrow_dev) alamb@MacBook-Pro-6:~/Software/influxdb_iox$ LOG_FILTER=ingester=debug,info cargo run    -- run all-in-one --max-http-request-size=1000000000 --persist-partition-size-threshold-bytes=200000000


use std::sync::atomic::{AtomicI64, Ordering};



#[tokio::main]
async fn main() {
    println!("Hello, starting....");

    let client = reqwest::Client::new();

    let org = "26f7e5a4b7be365b";
    let bucket = "917b97a92e883afc";

    let params = [("org", org),("bucket", bucket)];

    let generator = LineProtoGenerator::new();

    let res = client.post("http://localhost:8080/api/v2/write")
        .query(&params)
        .body(generator.make_lines(1))
        .send()
        .await
        .expect("request");

    if res.status().is_success() {
        println!("Success!");
    }
    else {
        println!("Failure");
        println!("{:#?}", res);
    }


    println!("done");
}


/// Makes pathalogical line protocol
#[derive(Debug, Default)]
struct LineProtoGenerator
{
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
    fn gen_line<W: std::io::Write>(&self, w: &mut W, i: usize) {
        let ts = self.timestamp_generator.fetch_add(1, Ordering::Relaxed);
        write!(w, "m,tag=A field=4 {}", ts).expect("write failed");

    }
}




// basic plan is to feed in data with high volumne with low cardinality tags (that thus compresses very well)
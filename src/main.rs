// (arrow_dev) alamb@MacBook-Pro-6:~/Software/influxdb_iox$ LOG_FILTER=ingester=debug,info cargo run    -- run all-in-one --max-http-request-size=1000000000 --persist-partition-size-threshold-bytes=200000000


#[tokio::main]
async fn main() {
    println!("Hello, starting....");

    let client = reqwest::Client::new();

    let org = "26f7e5a4b7be365b";
    let bucket = "917b97a92e883afc";

    let params = [("org", org),("bucket", bucket)];

    let res = client.post("http://localhost:8080/api/v2/write")
        .query(&params)
        .body("the exact body that is sent")
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





// basic plan is to feed in data with high volumne with low cardinality tags (that thus compresses very well)

# rust-s3-example

An example of using the Rust AWS SDK to interact with an S3 bucket.

To run, first start the `localstack` container by running:

```bash
$ docker-compose up
```

Then run the code itself with:

```bash
$ cargo run
```

Check that files have been written to the local S3 bucket using:

```bash
$ docker exec -it localstack awslocal s3api list-objects-v2 --bucket test-bucket
```

Finally, to teardown the stack:

```bash
$ docker-compose down -v
```

## Rust api mongoDB

Actix + MongoDB Rust Driver


version lib 
```
actix-web = "4"
actix-cors = "0.6.4"
serde = "1.0.147"
dotenv = "0.15.0"
futures = "0.3.25"
mongodb = "2.3.0"
```


set file .env befor connect Mongodb 

```
MONGOURI=mongodb://[USER]:[PASSWORD]@[127.0.0.1]:27017/admin?authSource=admin&readPreference=primary&directConnection=true&ssl=false
```


run for Build
```
cargo build
```

run PRD
```
./target/debug/rust_api_mongodb
```
or
```
cargo run
```
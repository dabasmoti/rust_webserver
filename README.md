###To reproduce the results run
####unoptimized version 
```
cargo run
```

###Optimized version 
```
cargo run --release
```

###Make post request
```
curl -X POST 0.0.0.0:8080/predict \
     -H 'Content-Type: application/json' \
     -d '{"instances": [{"_id": "7608657044"}]}'
```

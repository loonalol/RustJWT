## RustJWT Generator

* Getting The Team ID
    1. Step 1
        Open The [Apple Developer Account](https://developer.apple.com/account)
        Get the Team ID under Membership Details,
        Copy and paste it into the example.json
    2. Getting The Key
        Open The [Apple Developer Account](https://developer.apple.com/account) (Again)
        Navigate to Keys
        Click on the + to create a new Key (Name it whatever)
        https://r2.e-z.host/a8d205e2-c05b-47a5-88de-e2b8c7916dbb/05hgo1fe.png
        Choose whatever services you need and then Continue
        Copy and paste that AuthKey_XXXXXXXXXX.p8 into the src folder
        edit the main.rs to use your key
        ```Rust
           let private_key = fs::read_to_string("AuthKey_XXXXXXXXXX.p8").expect("Failed to read private key");
        ```
    3. Editing the json file
        run this command inside the src folder
        ```bash
         cp example.json jwt.json
        ```
        afterwards open the jwt.json and copy and paste your Key ID into apns_key_id
        ```json
        {
            "apns_key_id": ""
        }
        ```
    4. Completed!
        You should be able to run 
        ```bash
        cargo run
        ```
        and the output should look like this
        ```
        Generated token:
        ```

    <iframe id='kofiframe' src='https://ko-fi.com/luna_foss/?hidefeed=true&widget=true&embed=true&preview=true' style='border:none;width:100%;padding:4px;background:#f9f9f9;' height='712' title='luna_foss'></iframe>
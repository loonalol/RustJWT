## RustJWT Generator

* Getting The Team ID
    1. Step 1
        Open The [Apple Developer Account](https://developer.apple.com/account)
        Get the Team ID under Membership Details,
        Copy and paste it into the example.json
       
    3. Getting The Key
        Open The [Apple Developer Account](https://developer.apple.com/account) (Again)
        Navigate to Keys
        Click on the + to create a new Key (Name it whatever)
        ![image](https://github.com/lunathefur/RustJWT/assets/85907829/26d9295a-6cd4-4a3a-8bb7-451cef42834c)
        Choose whatever services you need and then Continue
        Copy and paste that AuthKey_XXXXXXXXXX.p8 into the src folder
        edit the main.rs to use your key
        ```Rust
           let private_key = fs::read_to_string("AuthKey_XXXXXXXXXX.p8").expect("Failed to read private key");
        ```
        
    4. Editing the json file
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
        
    5. Completed!
        You should be able to run 
        ```bash
        cargo run
        ```
        and the output should look like this
        ```
        Generated token:
        ```

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/L4L8Q0NIF)

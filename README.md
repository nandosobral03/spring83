# Readme

This repository contains an implementation of both a Client and a Server for the speculative protocol [Spring '83](https://github.com/robinsloan/spring-83), first ideated by Robin Sloan on his [blog](https://www.robinsloan.com/lab/specifying-spring-83/) and later specified in the [Spring '83 specification Draft](https://github.com/robinsloan/spring-83/blob/main/draft-20220629.md). I recommend reading the specification before as to understand the protocol better.

This is an implementation written in Rust of most of the protocol as well as a companion client written with SvelteKit. You can interact with the client [here](https://spring83.aornum.xyz) and create your own board, all you need is a valid keypair which you can generate anyway you want, including my [Spring '83 Key Generator](https://github.com/nandosobral03/spring83-keygen). And the server is hosted in [spring83-server.aornum.xyz](https://spring83-server.aornum.xyz) as well.

## How to run

### Server

You will first need to specify the following environment variables:
```bash
DATABASE_URL = mongodb://
PASSWORD = 
JWT_SECRET =
PORT =  
```

The mongodb url is the url to your mongodb database, the password is the password for the admin user used to generate blacklisted keypairs and the jwt secret is the secret used to sign the jwt tokens that can be used for authentication for things like getting a user's followed boards and key custody. (Note that both the client and the server are fully functional without authentication, but you will not be able to follow boards or have key custody)
Lastly, the port is the port the server will run on (defaults to 3000).

After that you can run the server with:
```bash
    cargo run --release
```
Or by building it and running the binary:
```bash
    cargo build --release
    ./target/release/spring83
```

### Client 

You will first need to specify the following environment variables:
```bash
API_URL = 
PUBLIC_API_URL = 
```

The API_URL is the url to the server's api and the PUBLIC_API_URL is the url to the server's api that will be used by the client. Both can be the the server's external url, but if both are running on the same machine API_URL can be changed for http://localhost:{PORT} to avoid DNS resolution. Since it will only be ran server-side for SSR, while PUBLIC_API_URL is used client-side.

After that you can run the client with:
```bash
    npm run dev
```
or alternatively build with the desired SvelteKit adapter and run the binary:
```bash
    npm run build
    npm run preview
```

## How to use

As to how to interact with the server, please refer to the [Spring '83 specification Draft](https://github.com/robinsloan/spring-83/blob/main/draft-20220629.md), since this implementation is compliant with it. To use the client you can see the recently updated boards and upload your own either anonymously with a keypair or with your username and password if you have an account and a keypair assigned. You can also follow boards and see them in your followed feed. Boards are deleted 21 days their last modification and are not recoverable, so be careful.

## License

MIT License

Copyright (c) 2023 Fernando Sobral

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

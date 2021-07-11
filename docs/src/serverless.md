# Going Serverless

Diana's most unique feature is its ability to bridge the serverless and serverful gap all in one system, and it's about time we covered how to use the serverless system! As for serverful systems, Diana uses integrations to support different serverless platforms. Currently, the only integration is for AWS Lambda and its derivatives, like Netlify and Vercel, so that's what we'll use here!

Crucially, **no part of your schemas or options** should have to change to go serverless, it should be simply a different way of using them.

## Coding it

First off, install `diana-aws-lambda` by adding the following to your `Cargo.toml` for the `serverless` crate under `[dependencies]` (notice that versions of integrations and the core library are kept in sync deliberately):

```toml
diana-aws-lambda = "0.2.8"
```

```rust
use diana_aws_lambda::{
    netlify_lambda_http::{
        lambda, lambda::Context as LambdaCtx, IntoResponse as IntoLambdaResponse,
        Request as LambdaRequest,
    },
    run_aws_req, AwsError,
};
use lib::get_opts;

#[lambda(http)]
#[tokio::main]
async fn main(req: LambdaRequest, _: LambdaCtx) -> Result<impl IntoLambdaResponse, AwsError> {
    let res = run_aws_req(req, get_opts()).await?;
    Ok(res)
}
```

This example also expects you to have [tokio](https://crates.io/crates/tokio) installed, you'll need a version above v1.0.0 for the runtime to work.

The serverless system is quite a bit simpler than the serverful system actually, because it just runs a query/mutation directly, without any need to run a server for a longer period. This handler is now entirely complete.

One thing to remember that could easily stump you for a while is environment variables. if you're reading from an environment variable file in your configuration setup, don't do that when you're in the serverless environment! And don't forget to add your environment variables to the serverless provider so they're available to your code!

## Deploying it

This page will only cover deploying this to [Netlify](https://netlify.com), since that's arguably the most convenient service to set up for Rust serverless functions quickly right now. The rest of this section will assume you have a Netlify account and that you've installed the Netlify CLI. The process is however relatively similar for other services.

Firstly, you'll need to set up a few basic things for Netlify deployment to work. Create a file named `rust-toolchain` in the `serverless` crate at its root (next to `Cargo.toml`). Then put the following in that file:

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-musl"]
```

This tells Netlify how to prepare the environment for Rust. Next, you'll need some static files to deploy as a website (you may already have a frontend to use, otherwise just a basic `inde.html` is fine). Put these in a new directory in the `serverless` crate called `public`. Also create another new empty directory called `functions` next to it.

Now we'll create a basic Netlify configuration. Create a `netlify.toml` file in the root of the `serverless` crate and put the following in it:

```toml
[build]
publish = "public"
functions = "functions"
```

This tells Netlify where your static files and functions are. But we haven't actually got any compiled functions yet, so we'll set those up now! Your final function will be the compiled executable of your code in `src/main.rs`.

Now we'll create a build script to prepare your function automatically. Create a new file in the `serverless` crate called `build.sh` and fill it with the following:

```bash
#!/bin/bash

cargo build --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/serverless functions
```

This will compile your binary for production and copy it to the `functions` directory, where Netlify can access it. Note that we're compiling for the `x86_64-unknown-linux-musl` target triple, which is the environment on Netlify's servers. To be able to compile for that target (a variant of Linux), you'll need to add it with `rustup target add x86_64-unknown-linux-musl`, which will download what you need.

There's one more thing we have to do before we can deploy though, and that's minimizing the size of the binary. Rust by default creates very large binaries, optimizing for speed instead. Diana is large and complex, which exacerbates this problem. Netlify does not like large binaries. At all. Which means we need to slim our release binary down significantly. However, because Netlify support for Rust is in beta, certain very powerful optimizations (like running `strip` to halve the size) will result in Netlify being unable to even detect your binary. Add the following to your _root_ `Cargo.toml` (the one for all your crates):

```toml
[profile.release]
opt-level = "z"
codegen-units = 1
panic = "abort"
```

This changes the compiler to optimize for size rather than speed, removes extra unnecessary optimization code, and removes the entire panic handling matrix. What this means is that your binary becomes smaller, which is great! However, if your program happens to `panic!` in production, it will just abort, so if you have _any_ custom panic handling logic, you'll need to play around with this a bit. Netlify will generally accept binaries under 15MB. Now there are more optimizations we could apply here to make the binary tiny, but then Netlify can't even detect it, so this is the best we can do (if you have something else that works better, please [open an issue](https://github.com/diana-graphql/diana/issues/new)).

Finally, you can run `sh build.sh` to build your function! Now we just need to send it to Netlify!

1. Log in to Netlify from the terminal with `netlify login`.
2. Create a new site for this project for manual deployment with `netlify init --manual` (run this in `serverless`).
3. Deploy to production with `netlify deploy --prod`!

You should now be able to query your live production GraphQL serverless function with any GraphQL or HTTP client! If you're having problems, Netlify's docunmentation may help, and don't forget to look at your site's logs!

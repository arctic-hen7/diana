# Netlify Example

This example is not entirely intuitive. The Rust code will compile, but it shouldn't be run locally. Optimally, it would be run with `netlify dev` on your local machine, but that tool doesn't yet support Rust functions (hopefully it soon will!). For now, the function actually has to be deployed to see the results. There is a build script at `build.sh` which will compile the binary, optimising for size (explained below). You can then upload that directly to a Netlify site. You should configure that site using the Netlify CLI.

If you're using another serverless platform that isn't based on AWS Lambda (this should work for it and all its derivatives), you'll need to consult the documentation for `run_serverless_req`, which takes a request body and a raw `Authorization` header and runs a request. That should work basically anywhere. See the book for an example of how to do that.

## Size Optimisations

Netlify does not like large files, and Rust produces very large binaries by default. So, we need to optimise explicitly for size rather than speed, which means including these settings in `Cargo.toml` at the project root (applies to everything, but only needed for this example).

```toml
[profile.release]
opt-level = "z"
codegen-units = 1
panic = "abort"
```

Further optimisations can be applied, but as of right now Netlify won't actually detect a super-optimised Rust binary (go figure), so we just use this for now.

## What's in `public` and `functions`?

`public` contains a bare-bones HTMl file because we have to serve some static content over Netlify.
`functions` is where the compiled function will go.

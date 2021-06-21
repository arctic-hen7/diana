# Contributing

First off, thanks so much for taking the time to contribute to Diana, it's greatly appreciated!

## I just want to propose something

If you just want to let us know about a bug, or propose a new feature, please open an issue on this repository. We'll take a look as soon as possible!

If you have a question about Diana, open a new issue with the `question` tag and we'll get to it as soon as possible!

## What should I work on?

You can check out the [roadmap](./README.md#Roadmap) or the [issues](https://github.com/arctic-hen7/diana/issues) to see what's currently needing to be done. If you want to work on something that's not on there, please file an issue and we'll take a look it as soon as possible!

## How do I contribute?

Contributing to a project on Github is pretty straight forward. If this is you're first time contributing to a project, all you need to do is fork this repository to your own GitHub account, add then change the code you want to (usually on your local machine, you'd pull your fork down). Commit your changes as necessary, and when you're done, submit a pull request on this repository and we'll review it as soon as possible!

Make sure your code doesn't break anything existing, that all tests pass, and, if necessary, add tests so your code can be confirmed to work automatically.

After you've submitted a pull request, a maintainer will review your changes. Unfortunately, not every pull request will be merged, but we'll try to request changes so that your pull request can best be integrated into the project.

## Building and Testing

- `cargo build`
- `cargo test`

Diana exposes three major components -- the dedicated subscriptions server, the serverful GraphQL system, and the serverless GraphQL system. The first two are what you'll most likely be working with, and these are run from within Docker. To work on Diana, you'll need both Docker and Docker Compose installed.

There's a series of Bonnie scripts in the root of the project, and `bonnie up` will allow you to bring up the whole system (including a MongoDB database for testing). That will make all systems react to your changes, though they can be a bit slow to do so (re-compiling Rust takes a while), so you may want to open two terminals and then run `bonnie sh-server` in one of them and `bonnie sh-subscriptions-server` in the other. Those will give you fully-fledged ZSH prompts in the containers from which you can work. `cargo watch -w . -w ../lib -x "run"` will build the containers reactively and let you see errors and the like.

If you're building something that's likely to impact serverless functionality, you'll need to test the deployment of the system on Netlify. Right now, Diana's serverless systems haven't been built, so instructions on how to do this will come once that's been done.

## Documentation

If the code you write needs to be documented in the help page, the README, or elsewhere, please do so! Also, please ensure your code is commented, it makes everything so much easier for everyone.

## Committing

We use the Conventional Commits system, but you can commit however you want. Your pull request will be squashed and merged into a single compliant commit, so don't worry about this!

# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.2.8](https://github.com/diana-graphql/diana/compare/v0.2.7...v0.2.8) (2021-07-11)


### Features

* ✨ exposed auth types and added method to get token claims ([e9f66f5](https://github.com/diana-graphql/diana/commit/e9f66f51d8988ebba8cc7deeb139793286b20984))

### [0.2.7](https://github.com/diana-graphql/diana/compare/v0.2.6...v0.2.7) (2021-07-11)


### Features

* 🗑 added `is_authed!` to replace `if_authed!` ([35e8a8f](https://github.com/diana-graphql/diana/commit/35e8a8fa872d1e2a624f95beb5c3c1caa1ca0d93))

### [0.2.6](https://github.com/diana-graphql/diana/compare/v0.2.5...v0.2.6) (2021-07-10)


### Bug Fixes

* 🐛 allowed `if_authed` to work regardless of return result ([9d2fe98](https://github.com/diana-graphql/diana/commit/9d2fe986af5d6f9614f7856c9fd3ee01bcaac2df))

### [0.2.6](https://github.com/diana-graphql/diana/compare/v0.2.5...v0.2.6) (2021-07-10)


### Bug Fixes

* 🐛 allowed `if_authed` to work regardless of return result ([9d2fe98](https://github.com/diana-graphql/diana/commit/9d2fe986af5d6f9614f7856c9fd3ee01bcaac2df))

### [0.2.5](https://github.com/diana-graphql/diana/compare/v0.2.4...v0.2.5) (2021-07-10)


### Features

* ✨ added `Options::builder()` method ([23454d9](https://github.com/diana-graphql/diana/commit/23454d922f21113facd0809e62dae4af765c9cc6)), closes [#1](https://github.com/diana-graphql/diana/issues/1)


### Bug Fixes

* 🚑 made `JWTSecret` `Clone`able ([61b8de6](https://github.com/diana-graphql/diana/commit/61b8de60f375e0821f5d9474202315e6233737d4))

### [0.2.4](https://github.com/diana-graphql/diana/compare/v0.2.3...v0.2.4) (2021-07-10)


### Bug Fixes

* 🚑 exposed hidden jwt types ([ff4a766](https://github.com/diana-graphql/diana/commit/ff4a76607184d11b47c2eb8c3ee06e5333b9dfa5))


### Documentation Changes

* **book:** 📝 updated book intro ([8234ff0](https://github.com/diana-graphql/diana/commit/8234ff00db4b1eec26d28482d883451a819b8b28))
* **book:** 📝 updated book to require installation of `async_graphql` directly ([2577738](https://github.com/diana-graphql/diana/commit/2577738552be068abc5c5d5e8d2f57d43119465a))

### [0.2.3](https://github.com/diana-graphql/diana/compare/v0.2.2...v0.2.3) (2021-07-09)


### Documentation Changes

* **book:** 📝 wrote book ([10739d5](https://github.com/diana-graphql/diana/commit/10739d5cb609ead61bb2f720253225df3e64c73e))
* 📝 updated readme ([45cca35](https://github.com/diana-graphql/diana/commit/45cca3553f43c6af3d037c7bb93e5ab03f12fb87))

### [0.2.2](https://github.com/diana-graphql/diana/compare/v0.2.1...v0.2.2) (2021-07-02)


### Bug Fixes

* 🐛 fixed type inference failure after `DianaHandler` changes ([a8a854c](https://github.com/diana-graphql/diana/commit/a8a854c96e808080f3a0e3fce4cb193acb609e04))
* 🐛 support binary bodies that can be serialized to strings in lambda integration ([b55ce88](https://github.com/diana-graphql/diana/commit/b55ce88dbbd69d4856ef96ad517fa9e2f7110dc5))


### Code Refactorings

* ♻️ made `DianaHandler.is_authed()` accept `Option<Into<String>>` ([c76aee0](https://github.com/diana-graphql/diana/commit/c76aee08a220dcef51fac94c1561afd16b3de732))

### [0.2.1](https://github.com/diana-graphql/diana/compare/v0.2.0...v0.2.1) (2021-06-30)


### Bug Fixes

* 🔧 fixed bonnie publish script ([a522249](https://github.com/diana-graphql/diana/commit/a522249576f0f29e476b27c1cec537301178d9df))
* **cargo:** 🔧 added diana versions to integration crates ([1373667](https://github.com/diana-graphql/diana/commit/1373667d6855bbcdc86961e7766002cb2545432c))


### Code Refactorings

* 🔧 split up a few bonnie scripts ([3056b89](https://github.com/diana-graphql/diana/commit/3056b89e08a9b1b83b5bd475bcd9a3783bad9172))


### Documentation Changes

* 📝 added readmes for integration crates ([cfdabda](https://github.com/diana-graphql/diana/commit/cfdabda4edbd0fe5e359130643fb07bbf73beb56))
* 📝 added versioning docs ([e265015](https://github.com/diana-graphql/diana/commit/e265015a7f7d7798daf16b3afc5e414ce26384fd))

## [0.2.0](https://github.com/diana-graphql/diana/compare/v0.1.1...v0.2.0) (2021-06-30)


### ⚠ BREAKING CHANGES

* renamed `AuthCheckBlockState` to `AuthBlockLevel`
* modules now fully re-exported rather than electively
* original serverless interface no longer supported
* radical changes with new integrations model (see the book)

### Features

* ✨ added integration for aws lambda and derivatives ([6b6ef32](https://github.com/diana-graphql/diana/commit/6b6ef324d2423617b78163846e9f7b16cb640e01))
* ✨ switched to integrations model with core logic ([40721eb](https://github.com/diana-graphql/diana/commit/40721eb2938d9b887437a28f9498981266d97ba5))


### Code Refactorings

* 🚚 refactored re-exports ([5ede923](https://github.com/diana-graphql/diana/commit/5ede9236d80b362da28bfade7e7ce4121b23bd0a))
* 🚚 renamed `AuthCheckBlockState` to `AuthBlockLevel` ([d34bfdd](https://github.com/diana-graphql/diana/commit/d34bfdd0af5d7566c0677827aba678db7b6e749c))


### Documentation Changes

* 📝 added documentation for integration crates ([99608c6](https://github.com/diana-graphql/diana/commit/99608c6a9e3fe0347617dbd13d0815ab5ac2e3d5))
* 📝 removed useless section of core crate docs ([6aedfac](https://github.com/diana-graphql/diana/commit/6aedfacd0334b792d6e2629d37414505aa32c91a))
* 📝 updated docs ([0e734a8](https://github.com/diana-graphql/diana/commit/0e734a852a127feb1542cd84cf66e3efa23cebaa))
* 📝 updated readme ([fef8ba6](https://github.com/diana-graphql/diana/commit/fef8ba638805286b90ada9dd740025ced83cf890))

### [0.1.1](https://github.com/diana-graphql/diana/compare/v0.1.0...v0.1.1) (2021-06-28)


### Bug Fixes

* 🔧 fixed incorrect compose target in playground ([84d926e](https://github.com/diana-graphql/diana/commit/84d926ea95756a6f77390d8799e755e5ccde7812))
* 🔧 updated crate name ([18ac391](https://github.com/diana-graphql/diana/commit/18ac3912d48b31e5b49c4819fb618ea1ab940a16))


### Code Refactorings

* 🚚 switched to workspace structure for examples ([ad330f2](https://github.com/diana-graphql/diana/commit/ad330f2abf5d5f14ad99fb5be6c39b316ae725ec))


### Documentation Changes

* ✏️ fixed typo in `if_authed` docs ([43c2fe4](https://github.com/diana-graphql/diana/commit/43c2fe4ad52a33ffd29ccde4c0315eb6cb013c8e))
* 📝 added mdbook basics ([ee475cd](https://github.com/diana-graphql/diana/commit/ee475cd601b7df09917dfba8676b502bc5565e8c))
* 📝 made trivial docs change to test book deployment ([76260da](https://github.com/diana-graphql/diana/commit/76260da8abc4a606afb55330908ac412d3f4477b))
* 📝 updated documentation examples ([d3477a2](https://github.com/diana-graphql/diana/commit/d3477a2f21d6c2b8756e77cbd71deac9e21597d6))

## 0.1.0 (2021-06-27)


### Features

* ✨ added aws-specific serverless function invoker ([2616733](https://github.com/arctic-hen7/diana/commit/26167331bae4bfb7afcbf8fbb84b2092a253aad4))
* ✨ added full serverless system ([96825bb](https://github.com/arctic-hen7/diana/commit/96825bbd501738684abbf40cc5f7da11d55bb221))
* ✨ added option to disable subscriptions server entirely ([234cd10](https://github.com/arctic-hen7/diana/commit/234cd10b5083751330ddbaf7e142a2e44482a298))
* ✨ modularised the query/mutation systems ([2a50470](https://github.com/arctic-hen7/diana/commit/2a50470109132c1b2a960f2b5f579842091e879e))
* ✨ modularised the subscriptions server ([a508e81](https://github.com/arctic-hen7/diana/commit/a508e812d8ba2ac07d9dd5699ba2cd458c48df1b))
* 🎉 imported code from elm-rust-boilerplate ([a1835f0](https://github.com/arctic-hen7/diana/commit/a1835f08b48abcf13ee157e51670f22b6d76c819))
* 🚧 added hello world serverless function ([60e608b](https://github.com/arctic-hen7/diana/commit/60e608b2ae15fed2716562f0490bfc2452522138))
* 🚧 set up basics for serverless setup ([c600f36](https://github.com/arctic-hen7/diana/commit/c600f36ef1416f0588b96077209231ada02524a7))


### Bug Fixes

* 🐛 fixed publisher error lockout ([36e1df4](https://github.com/arctic-hen7/diana/commit/36e1df41175246511ea6f262c18c7bae74767c94))
* 🥅 added error handling for tokio broadcasts ([9cb0fe6](https://github.com/arctic-hen7/diana/commit/9cb0fe61411ee09988435296b7c654a048c3240e))


### Code Refactorings

* ♻️ broke out serverless handler into separate function ([7ffa3ac](https://github.com/arctic-hen7/diana/commit/7ffa3ac77a55c1e57762febe5f8d6539175db05a))
* ♻️ broke out serverless handler into separate function ([c1a08fc](https://github.com/arctic-hen7/diana/commit/c1a08fc3d107ec4b27fe9575bd8381962293b629))
* ♻️ fixed convoluted error management and made publishes clearer ([c53aa8b](https://github.com/arctic-hen7/diana/commit/c53aa8b16cc5a65af92b9d2b4d185913c509bc44))
* 🚚 made gigantic infrastructure changes ([4525f04](https://github.com/arctic-hen7/diana/commit/4525f04398181c8d1c9065e3f41348f22a7e334b))
* 🚚 moved dev utilities into a separate sub-crate ([915ed72](https://github.com/arctic-hen7/diana/commit/915ed7229b5c85baef4054d70b3cd043fa2df12e))


### Documentation Changes

* 📝 added crate-level rustdoc ([2b2d9d8](https://github.com/arctic-hen7/diana/commit/2b2d9d8314d506c79875fddd7b6bdde7bb67ce64))
* 📝 documented everything with rustdoc ([938b4bd](https://github.com/arctic-hen7/diana/commit/938b4bdea0640941149929f840d404cd23269513))
* 📝 fixed rustdoc links ([594c04a](https://github.com/arctic-hen7/diana/commit/594c04a8207cc8f228205873da1351d6605114d1))
* 🔥 removed erroneous examples readme ([62e8a93](https://github.com/arctic-hen7/diana/commit/62e8a93d474d68b306ddc504148c73cf67e4539e))

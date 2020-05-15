# fuzzysearch-web

[![Build Status](https://drone.huefox.com/api/badges/Syfaro/fuzzysearch-web/status.svg)](https://drone.huefox.com/Syfaro/fuzzysearch-web)

The (WIP) web interface for [FuzzySearch](https://fuzzysearch.net).

It is written in Rust using [Yew](https://github.com/yewstack/yew).

The backend can be found [here](https://github.com/Syfaro/fuzzysearch).

## Why Rust/WASM?

I really enjoy writing things in Rust. It also makes it easier to offload
search work onto the client because images can be hashed in-browser and don't
require the bandwidth of sending images.

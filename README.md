# Kez

[![crates.io](https://img.shields.io/crates/v/kez)](https://crates.io/crates/kez)
[![docs.rs](https://img.shields.io/docsrs/kez)](https://docs.rs/kez)

A Rust crate for interacting with Valve's Dota 2 Web API. Named after the newest hero added to Dota 2 as of this crate's creation date (2025-01-09).

## Motivation

Existing Rust crates like `dota2_api` and `dota2_webapi_bindings` are no longer active maintained and lack support for newer API features. While building a Dota 2 match finder, I needed an API they didn't provide, so I created Kez.

## Supported APIs

 - get_match_history_by_seq_num: Fetch up to 100 sequential matches. Ideal for bulk data collection.
 - get_match_history: Search for matches by player, hero, or other criteria (limited filtering support).
 - get_heroes: Retrieve metadata for all heroes.
 - history: This is a thin wrapper around get_match_history_by_seq_num, which returns a list of Matches with more detailed information.

 ## TODOs

  - [X] add more dota2 related types, LobbyType/GameMode/Item/...
  - [ ] add more APIs? GetTopLiveGame?
  - [ ] add more documents and comments about varies fields of response
  - [ ] other things...

## Disclaim

Don't use this crate for anything serious, it is fundamentally unstable since every game update could potentially break this crate.

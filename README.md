# Kez

Kez is a rust crate for valve's dota2 web api.

## Motivation

As of 2025/01/09, there are two rust crate for the same functionality, namely `dota2_api` and `dota2_webapi_bindings`, however they are both outdated. In my attempt to write a dota2 match finder in rust, I have to manually write these equivelent code, so I decided to extract those code into a separate crate.

## Naming

Kez is the newest hero added to dota2 as of the time this crate is created(2025/01/09).

## Supported APIs

 - get_match_history_by_seq_num: this is the primary APIs used by varies dota2 data websites now, you can requests at most 100 matches with this APIs in one request, and it gives back the most detailed match information like get_match_detail(this is not working right now).
 - get_match_history: this is the API for you if you want to find some specific matches, for example with specific hero or player. However this API is very hard to use IMO.
 - get_heroes: request a list of current available heroes.

#!/bin/bash

bindgen src/wrapper.h --allowlist-item "^FT_.*$" -- -Ithird_party/freetype-2.13.3/include/ > src/freetype.rs

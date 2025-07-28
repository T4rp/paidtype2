#!/bin/bash

bindgen src/wrapper.h --allowlist-item "^FT_.*$" -- -Ifreetype/include/ > src/freetype.rs

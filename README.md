# read_nef — reverse‑engineering Nikon NEF in Rust

This is a hobby project where I reverse‑engineered parts of Nikon's proprietary NEF raw format and wrote a minimal decoder in Rust. It parses the TIFF/IFD structure, finds Nikon MakerNote data containing Huffman configuration, locates the raw strip(s), performs Huffman decompression with Nikon‑specific trees and predictors and does minimal transformations to produce a JPEG preview from the decoded data.

It’s not a full raw converter. The focus is on understanding and documenting the data path from NEF to a viewable image using as little “magic” as possible. Tested with Nikon D7500.

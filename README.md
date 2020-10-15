# Arch DB Meta Exporter

This utility can help you extract all packages and their metadata from a pacman synced db file.

## Get Started

`cargo build` on your own or download the prebuilt binary from `releases` page.

``` shell script
$ arch-db-meta-rs core.db core_meta.json
$ cat core_meta.json | jq
[
  {
    "arch": "x86_64",
    "name": "acl",
    "version": "2.2.53-3",
    "description": "Access control list utilities, libraries and headers",
    "last_built": 1588833464,
    "filename": "acl-2.2.53-3-x86_64.pkg.tar.zst",
    "size": 139908
  }, ...
]
```

## License
This project is licensed under MIT License - see the [LICENSE.md](LICENSE.md) file for details.
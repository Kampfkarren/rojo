<div align="center">
    <img src="assets/rojo-logo.png" alt="Rojo" height="217" />
</div>

<div>&nbsp;</div>

<div align="center">
    <a href="https://travis-ci.org/LPGhatguy/rojo">
        <img src="https://api.travis-ci.org/LPGhatguy/rojo.svg?branch=master" alt="Travis-CI Build Status" />
    </a>
    <a href="https://crates.io/crates/rojo">
        <img src="https://img.shields.io/crates/v/rojo.svg?label=version" alt="Latest server version" />
    </a>
    <a href="https://lpghatguy.github.io/rojo">
        <img src="https://img.shields.io/badge/documentation-website-brightgreen.svg" alt="Rojo Documentation" />
    </a>
</div>

<hr />

**Rojo** is a flexible multi-tool designed for creating robust Roblox projects.

It lets Roblox developers use industry-leading tools like Git and VS Code, and crucial utilities like Luacheck.

Rojo is designed for **power users** who want to use the **best tools available** for building games, libraries, and plugins.

## Features
Rojo lets you:

* Work on scripts from the filesystem, in your favorite editor
* Version your place, library, or plugin using Git or another VCS
* Sync JSON-format models from the filesystem into your game

Later in 2018, Rojo will be able to:

* Sync scripts from Roblox Studio to the filesystem
* Compile MoonScript and sync it into Roblox Studio
* Sync `rbxmx` models between the filesystem and Roblox Studio
* Package projects into `rbxmx` files from the command line

## [Documentation](https://lpghatguy.github.io/rojo)
You can also view the documentation by browsing the [docs](https://github.com/LPGhatguy/rojo/tree/master/docs) folder of the repository, but because it uses a number of Markdown extensions, it may not be very readable.

## Inspiration and Alternatives
There are lots of other tools that sync scripts into Roblox or provide other tools for working with Roblox places.

Here are a few, if you're looking for alternatives or supplements to Rojo:

* [rbxmk by Anaminus](https://github.com/anaminus/rbxmk)
* [Rofresh](https://github.com/osyrisrblx/rofresh) and [RbxRefresh](https://github.com/osyrisrblx/RbxRefresh) by [Osyris](https://github.com/osyrisrblx)
* [Studio Bridge by Vocksel](https://github.com/vocksel/studio-bridge)
* [RbxSync by evaera](https://github.com/evaera/RbxSync)
* [CodeSync](https://github.com/MemoryPenguin/CodeSync) and [rbx-exteditor](https://github.com/MemoryPenguin/rbx-exteditor) by [MemoryPenguin](https://github.com/MemoryPenguin)

If you use a plugin that _isn't_ Rojo for syncing code, open an issue and let me know why! I'd like Rojo to be the end-all tool so that people stop reinventing solutions to this problem.

## Contributing
The `master` branch is a rewrite known as **Epiphany**. It includes a breaking change to the project configuration format and an infrastructure overhaul.

Building Rojo requires the latest Rust beta in order to use **2018 Edition** features. Once Rust 1.31 is stable on **December 6, 2018**, Rojo `master` will compile on Rust stable again.

Pull requests are welcome!

All pull requests are run against a test suite on Travis CI. That test suite should always pass!

## License
Rojo is available under the terms of the Mozilla Public License, Version 2.0. See [LICENSE](LICENSE) for details.
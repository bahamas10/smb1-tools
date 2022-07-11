SMB1 Tools
==========

*NOTE:* This software is pre-alpha.  I'll try my best to not break anything but
don't expect it to do anything really. I don't even know what the final product
will look like yet, but the current goal is to just parse a Super Mario Bros 1
(NES) rom file as bytes and understand and map it all out in rust.

---

You need an SMB1 rom with the following checksum:

    MD5 (smb1.nes) = 811b027eaf99c2def7b933c5208636de

Validate Rom
------------

Place the rom file into the root of this repo with the name:

    smb1.nes

This can be validated with:

    $ cargo test --test validate-rom
    Finished test [unoptimized + debuginfo] target(s) in 0.04s
    Running tests/validate-rom.rs (target/debug/deps/validate_rom-46322348c82600a9)

    running 2 tests
    test test_rom_valid ... ok
    test test_level_data_valid ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Extract Data
------------

    $ cargo run -q --bin extract -- ./smb1.nes
    level = Level {
        level_header: LevelHeader {
            time: T400,
            start_position: FallFromSkyAlternate,
            start_autowalk: false,
            background: DayTime,
            scenery: Mountains,
            platform: GreenAndTrees,
            ground: BasicFloor,
        },
        object_data: LevelObjectData {
            objects: [
                LevelObject {
                    kind: QuestionBlockCoin,
                    x_coordinate: 112,
                    y_coordinate: 7,
                    new_page_flag: true,
                },
                LevelObject {
                    kind: HorizontalBrick(
                        5,
                    ),
                    x_coordinate: 112,
                    y_coordinate: 7,
                    new_page_flag: false,
                },
    ....

Credits and Documentation Used
------------------------------

- [Super Mario Bros. Hacking Document](https://www.romhacking.net/documents/76/)
- [Super Mario Bros. Level Generation Using
- Torch-RNN](https://medium.com/@justin_michaud/super-mario-bros-level-generation-using-torch-rnn-726ddea7e9b7)
- [SMB1 Level Format (All
- Stars)](https://github.com/bonimy/MushROMs/blob/master/doc/SMB1%20Level%20Format.md)

License
-------

MIT License

# Mirl Buffer (0.0.0-alpha)

### Muffel - A crate defining `Buffer` and `ConstBuffer` for graphical purposes

<details>
<summary>Flags</summary>

### Default:

**Core**

- `std` (Default)
- `c_compatible`
- `all`

**Codec**

- `all_codecs`
- `serde`
- `bitcode`
- `wincode` (bitcode recommended)
- `zerocopy`
- `compactly`

</details>

### Entry Points

> `prelude` provided

- `Buffer` and `ConstBuffer` structs accessible under `mirl_buffer`.

- `mirl_buffer::traits::*` for all the functionality these 2 provide

### Purpose

Store raw color data to safely and quickly access entries

### Disclaimer

The only drawing functions provided in this lib are `set_pixel_safe` and `set_pixel_unsafe`. For more advanced drawing; use [`mirl_rendering`](https://github.com/Miner3D-Gamer/mirl_rendering). 

### Origin

`Buffer` is used almost everywhere throughout mirl so putting it _anywhere_ logically meant looping dependency cycles. 
The fix? Just create a standalone crate!
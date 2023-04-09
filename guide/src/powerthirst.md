# Powerthirst Edition

The [Powerthirst Edition](https://www.buildandshoot.com/forums/viewtopic.php?f=13&t=3445) is a mod of the version 0.75 that adds a few new features, such as:

- Longer user names
- 64 players
- Support for [AngelScript](http://www.angelcode.com/angelscript/) scripts

This version adds 4 new packets, extends 2 packets, and duplicate maps 1 packet over another.

The World Update packet has up to 64 fields now instead of 32.

## Map StartColor

`Server->Client`

Sent when a client connects, or a map is advanced for already existing connections.

Should be the first packet received when a client connects.

The version must exist and be >= 1 (and &lt;= the client's Powerthirst proto version or otherwise the client will refuse to connect) to enable certain Powerthirst features such as long-name support.

|||
| ----------: | -------- |
| Packet ID   | 18       |

### Fields

| Field Name | Field Type | Example | Notes |
| ---------- | ---------- | ------- | ----- |
| Map size   | Uint32     | `4567`  |       |
| PT version | UInt32     | `4`     |       |

## Map ChunkColor

`Server->Client`

This is just a remapping of the [Map Chunk](protocol.md#map-chunk) packet to 2 packets back to stop vanilla clients from connecting.

|||
| ----------: | -------- |
| Packet ID   | 17       |
| Total Size: | 9 bytes  |

### Fields

| Field Name | Field Type | Example | Notes                                                                                                                               |
| ---------- | ---------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Map Data   | UByte      | `0`     | [DEFLATE/zlib](http://en.wikipedia.org/wiki/DEFLATE) encoded [AOS map data](http://silverspaceship.com/aosmap/aos_file_format.html) |

## Script BeginColor

`Server->Client`

|||
| ----------: | --------       |
| Packet ID   | 31             |
| Total Size: | (varies) bytes |

### Fields

| Field Name  | Field Type                                                 | Example | Notes |
| ----------- | ---------------------------------------------------------- | ------- | ----- |
| Script size | Uint32                                                     | `4567`  |       |
| Module name | [CP437](http://en.wikipedia.org/wiki/Code_page_437) String |         |       |

## Script ChunkColor

`Server->Client`

This is just a remapping of the [Map_Chunk](protocol.md#map-chunk) packet to 2 packets back to stop vanilla clients from connecting.

|||
| ----------: | --------       |
| Packet ID   | 32             |
| Total Size: | (varies) bytes |

### Fields

| Field Name  | Field Type | Example | Notes                                                                                                                         |
| ----------- | ---------- | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Script Data | UByte      | `0`     | [DEFLATE/zlib](http://en.wikipedia.org/wiki/DEFLATE) encoded [AngelScript source code](http://www.angelcode.com/angelscript/) |


## Script EndColor

`Server->Client`

Once this is sent, the script is loaded.

|||
| ----------: | --------       |
| Packet ID   | 33             |
| Total Size: | (varies) bytes |

### Fields

| Field Name  | Field Type                                                 | Example | Notes |
| ----------- | ---------------------------------------------------------- | ------- | ----- |
| Module name | [CP437](http://en.wikipedia.org/wiki/Code_page_437) String |         |       |

## Script CallColor

`Server->Client`

|||
| ----------: | --------       |
| Packet ID   | 34             |
| Total Size: | (varies) bytes |

### Fields

| Field Name    | Field Type                                                              | Example       | Notes                                                      |
| ------------- | ----------------------------------------------------------------------- | ------------- | ---------------------------------------------------------- |
| Function name | 0-terminated [CP437](http://en.wikipedia.org/wiki/Code_page_437) String | `void main()` | Must be an AngelScript prototype, not just the name itself |
| Parameters    | See below                                                               |               |                                                            |

### Script Parameters

Start from after the 0-byte in the Function name string. Then, loop through these IDs:

* 0: `ASP_TERM`: End of parameter list.
* 1: `ASP_INT`: Read a 32-bit little-endian int. AngelScript type: "int"
* 2: `ASP_FLOAT`: Read a 32-bit little-endian single-precision float. AngelScript type: "float"
* 3: `ASP_PSTRING`: Read an 8-bit uint, then read that many bytes as a string (do NOT add in a terminating NUL). AngelScript type: "const string &in"

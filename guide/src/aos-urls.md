# AoS URLs

AoS uses urls like "aos://1124169524:43887" to encode the ip and port of a server. This is a guide on how to parse them.

## Parsing Algorithm

1. strip protocol prefix (e.g. "aos://" -> "1124169524:43887")
1. search for colon (e.g. "*:*" -> "1124169524" ":" "43887")
    1. if none found
        1. port is default port (32887)
    1. if found
        1. split at colon (e.g. "*:*" -> "1124169524" ":" "43887")
        1. port is second part
        1. continue with first part
1. search for dot (e.g. "*.*" -> none found in "1124169524")
    1. if at least one (or better, exactly four) found (e.g. "192.168.1.1")
        1. URL is IP
    1. if none found
        1. URL is an integer address, parse it as explained [here](#decoding-integer-addresses)

## Decoding integer addresses

The integer address is a 32-bit integer that represents the IP address. The IP address is encoded in big-endian. It can be decoded by using the following algorithm:

1. to get the four IP parts:
    1. apply bitwise AND with 255 (e.g., "1124169524" & 255 -> 52)
    1. right-shift the URL by 8 bits and then apply bitwise AND with 255 (e.g., ("1124169524" >> 8) & 255 -> 119)
    1. right-shift the URL by 16 bits and then apply bitwise AND with 255 (e.g., ("1124169524" >> 16) & 255 -> 1)
    1. right-shift the URL by 24 bits and apply bitwise AND with 255 (e.g., ("1124169524" >> 24) & 255 -> 67)
1. the four IP parts are 52, 119, 1, 67
1. join the parts with dots (e.g. "52.119.1.67")



Example JS code:

```js
let ip = ${url & 255}.${(url >> 8) & 255}.${(url >> 16) & 255}.${(url >> 24) & 255}
```

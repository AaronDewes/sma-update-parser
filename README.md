# SMA up2 parser

This is a simple parser for up2 files, often used by SMA devices.

The goal is to eventually allow full dumping of their update files.

Currently, the format can be fully parsed.

Also, while the raw firmware data can be extracted, it is not yet possible to understand the firmware binary/executable itself.

### About the format

The up2 format seems to be designed to make it harder to reverse engineer the firmware.

Another goal of the format seems to be to provide a simple set of instructions on how an update is transmitted from one central control device to other, connected devices.

It contains instructions on how often to send data, how to verify it, ....

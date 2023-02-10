# Changelog

### Latest/Nightly (this branch)

### v0.9.1
- Features
    - Added as_u64() for ProtobufMessageEnumTraits which returns the messages id.
- Bug Fixes
    - Fixed bug in Vector3F64::from_readable() that would cause a crash with partial/broken demos.
- Changes
    - Add some new (but unidentified) NetMessages.
- Internal
    - set source-demo-tool-impl-proc-macros version to 0.4.1

### v0.9.0
- Features
    - Partial/broken demos can now be opened, if a frame cannot be parsed, the parsing will end early and the last error will be placed in DemoFile::last_index_error.
    - DemoFile::{ get_data_tables(), get_server_info(), get_game_event_list() } now additionally search main demo frames for data (seen in partial/broken demos).
- Changes
    - FromProtobufMessageWarnings::sub_warnings is now a tuple that includes the sub-field name.
    - indexing errors propagate up better now
    - Add some new (but unidentified) NetMessages and UserMessages.
- Internal
    - set source-demo-tool-impl-proc-macros version to 0.4.0

### v0.8.0
- Changes
    - Changed NetMessage::SendTable::SendProp::{ low_value, high_value } from Float32 to Fixed32, closes #3.

### v0.7.3
- Changes
    - Added (made public) get_game_event_list for DemoFile.
    - Added get_id_map for ProtobufMessageEnumTraits.
- Internal
    - set source-demo-tool-impl-proc-macros to 0.3.4

### v0.7.2
- Changes
    - Added Sync + Send for ProtobufMessageEnumTraits.

### v0.7.1
- Bug Fix
    - Fixed missing proc-macros upgrade in v0.7.0.
- Internal
    - set source-demo-tool-impl-proc-macros to 0.3.3

### v0.7.0
- Changes
    - moved impl ToString to enum traits as to_str, changed return signature to &'static str.

### v0.6.3
- Changes
    - added Self: Sized bound for method type_count.

### v0.6.2
- Changes
    - made type_count a static function.
- Internal
    - set source-demo-tool-impl-proc-macros to 0.3.2

### v0.6.1
- Features
    - Added type_count for protobuf messages, returns the number of messages in the enum.
- Internal
    - set source-demo-tool-impl-proc-macros to 0.3.1

### v0.6.0
- Features
    - FullGameEvent now includes frame and message index.

### v0.5.0
- Features
    - ParsedUserMessage now contains the frame tick for convenience.

### 0.4.2
- Features
    - added Clone to various structs

### 0.4.0/0.4.1
- Features
    - get_user_messages now returns the message index in addition to the frame index for each message.
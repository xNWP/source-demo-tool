# Changelog

### Latest/Nightly (this branch)
- Changes
    - Added (made public) get_game_event_list for DemoFile.
    - Added get_id_map for ProtobufMessageEnumTraits.

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
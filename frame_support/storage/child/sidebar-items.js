initSidebarItems({"enum":[["ChildInfo","Information related to a child state."],["ChildType","Type of child. It does not strictly define different child type, it can also be related to technical consideration or api variant."],["KillStorageResult","The outcome of calling `storage_kill`. Returned value is the number of storage items removed from the backend from making the `storage_kill` call."],["StateVersion","Different possible state version."]],"fn":[["exists","Check to see if `key` has an explicit entry in storage."],["get","Return the value of the item in storage under `key`, or `None` if there is no explicit entry."],["get_or","Return the value of the item in storage under `key`, or `default_value` if there is no explicit entry."],["get_or_default","Return the value of the item in storage under `key`, or the type’s default if there is no explicit entry."],["get_or_else","Return the value of the item in storage under `key`, or `default_value()` if there is no explicit entry."],["get_raw","Get a Vec of bytes from storage."],["kill","Ensure `key` has no explicit entry in storage."],["kill_storage","Remove all `storage_key` key/values"],["len","Return the length in bytes of the value without reading it. `None` if it does not exist."],["put","Put `value` in storage under `key`."],["put_raw","Put a raw byte slice into storage."],["root","Calculate current child root value."],["take","Remove `key` from storage, returning its value if it had an explicit entry or `None` otherwise."],["take_or","Return the value of the item in storage under `key`, or `default_value` if there is no explicit entry. Ensure there is no explicit entry on return."],["take_or_default","Remove `key` from storage, returning its value, or, if there was no explicit entry in storage, the default for its type."],["take_or_else","Return the value of the item in storage under `key`, or `default_value()` if there is no explicit entry. Ensure there is no explicit entry on return."]]});
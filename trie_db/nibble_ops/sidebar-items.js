initSidebarItems({"constant":[["BIT_PER_NIBBLE","Single nibble length in bit."],["CONTENT_HEADER_SIZE","Size of header."],["NIBBLE_LENGTH","Number of child for a branch (trie radix)."],["NIBBLE_PER_BYTE","Number of nibble per byte."],["PADDING_BITMASK","Nibble (half a byte)."],["SPLIT_SHIFTS","The nibble shifts needed to align. We use two value, one is a left shift and the other is a right shift."]],"fn":[["at","Get u8 nibble value at a given index in a `NibbleSlice`."],["at_left","Get u8 nibble value at a given index of a byte."],["biggest_depth","Count the biggest common depth between two left aligned packed nibble slice."],["left_common","Calculate the number of common nibble between two left aligned bytes."],["left_nibble_at","Get u8 nibble value at a given index in a left aligned array."],["number_padding","Calculate the number of needed padding a array of nibble length `i`."],["pad_left","Mask a byte, keeping left nibble."],["pad_right","Mask a byte, keeping right byte."],["push_at_left","Push u8 nibble value at a given index into an existing byte."],["shift_key","Shifts right aligned key to add a given left offset. Resulting in possibly padding at both left and right (example usage when combining two keys)."]]});
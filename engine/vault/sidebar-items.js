initSidebarItems({"mod":[["view",""]],"struct":[["ChainId","A chain identifier.  Used to identify a transaction."],["ClientId","Client Id type used to identify a client."],["Id","A generic Id type used as the underlying type for the `ClientId` and `VaultId` types."],["Key","A key to the crypto box.  [`Key`] is stored on the heap which makes it easier to erase. Makes use of the [`GuardedVec<u8>`] type to protect the data."],["RecordHint","a record hint.  Used as a hint to what this data is used for."],["RecordId","A record identifier.  Contains a [`ChainId`] which refers to the transaction."],["VaultId","Vault Id type used to identify a vault."]],"trait":[["Base64Decodable","a trait to make types base64 decodable"],["Base64Encodable","a trait to make types base64 encodable"],["BoxProvider","A provider interface between the vault and a crypto box. See libsodium’s secretbox for an example."],["Decrypt","Trait for decryptable data. Allows the data to be decrypted."],["Encrypt","trait for encryptable data. Allows the data to be encrypted."]]});
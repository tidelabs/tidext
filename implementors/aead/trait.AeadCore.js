(function() {var implementors = {
"aes_gcm":[["impl&lt;Aes, NonceSize&gt; <a class=\"trait\" href=\"aes_gcm/trait.AeadCore.html\" title=\"trait aes_gcm::AeadCore\">AeadCore</a> for <a class=\"struct\" href=\"aes_gcm/struct.AesGcm.html\" title=\"struct aes_gcm::AesGcm\">AesGcm</a>&lt;Aes, NonceSize&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Aes: <a class=\"trait\" href=\"cipher/block/trait.NewBlockCipher.html\" title=\"trait cipher::block::NewBlockCipher\">NewBlockCipher</a> + <a class=\"trait\" href=\"cipher/block/trait.BlockCipher.html\" title=\"trait cipher::block::BlockCipher\">BlockCipher</a>&lt;BlockSize = <a class=\"type\" href=\"typenum/generated/consts/type.U16.html\" title=\"type typenum::generated::consts::U16\">U16</a>&gt; + <a class=\"trait\" href=\"cipher/block/trait.BlockEncrypt.html\" title=\"trait cipher::block::BlockEncrypt\">BlockEncrypt</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Aes::<a class=\"associatedtype\" href=\"cipher/block/trait.BlockCipher.html#associatedtype.ParBlocks\" title=\"type cipher::block::BlockCipher::ParBlocks\">ParBlocks</a>: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"type\" href=\"cipher/block/type.Block.html\" title=\"type cipher::block::Block\">Block</a>&lt;Aes&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;NonceSize: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.u8.html\">u8</a>&gt;,</span>"]],
"chacha20poly1305":[["impl <a class=\"trait\" href=\"aead/trait.AeadCore.html\" title=\"trait aead::AeadCore\">AeadCore</a> for <a class=\"struct\" href=\"chacha20poly1305/struct.XChaCha20Poly1305.html\" title=\"struct chacha20poly1305::XChaCha20Poly1305\">XChaCha20Poly1305</a>"],["impl&lt;C&gt; <a class=\"trait\" href=\"aead/trait.AeadCore.html\" title=\"trait aead::AeadCore\">AeadCore</a> for <a class=\"struct\" href=\"chacha20poly1305/struct.ChaChaPoly1305.html\" title=\"struct chacha20poly1305::ChaChaPoly1305\">ChaChaPoly1305</a>&lt;C&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"cipher/common/trait.NewCipher.html\" title=\"trait cipher::common::NewCipher\">NewCipher</a>&lt;KeySize = <a class=\"type\" href=\"typenum/generated/consts/type.U32.html\" title=\"type typenum::generated::consts::U32\">U32</a>, NonceSize = <a class=\"type\" href=\"typenum/generated/consts/type.U12.html\" title=\"type typenum::generated::consts::U12\">U12</a>&gt; + <a class=\"trait\" href=\"cipher/stream/trait.StreamCipher.html\" title=\"trait cipher::stream::StreamCipher\">StreamCipher</a> + <a class=\"trait\" href=\"cipher/stream/trait.StreamCipherSeek.html\" title=\"trait cipher::stream::StreamCipherSeek\">StreamCipherSeek</a>,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
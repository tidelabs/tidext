(function() {var implementors = {};
implementors["chacha20poly1305"] = [{"text":"impl <a class=\"trait\" href=\"aead/trait.AeadInPlace.html\" title=\"trait aead::AeadInPlace\">AeadInPlace</a> for <a class=\"struct\" href=\"chacha20poly1305/struct.XChaCha20Poly1305.html\" title=\"struct chacha20poly1305::XChaCha20Poly1305\">XChaCha20Poly1305</a>","synthetic":false,"types":["chacha20poly1305::xchacha20poly1305::XChaCha20Poly1305"]},{"text":"impl&lt;C&gt; <a class=\"trait\" href=\"aead/trait.AeadInPlace.html\" title=\"trait aead::AeadInPlace\">AeadInPlace</a> for <a class=\"struct\" href=\"chacha20poly1305/struct.ChaChaPoly1305.html\" title=\"struct chacha20poly1305::ChaChaPoly1305\">ChaChaPoly1305</a>&lt;C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"cipher/common/trait.NewCipher.html\" title=\"trait cipher::common::NewCipher\">NewCipher</a>&lt;KeySize = <a class=\"type\" href=\"typenum/generated/consts/type.U32.html\" title=\"type typenum::generated::consts::U32\">U32</a>, NonceSize = <a class=\"type\" href=\"typenum/generated/consts/type.U12.html\" title=\"type typenum::generated::consts::U12\">U12</a>&gt; + <a class=\"trait\" href=\"cipher/stream/trait.StreamCipher.html\" title=\"trait cipher::stream::StreamCipher\">StreamCipher</a> + <a class=\"trait\" href=\"cipher/stream/trait.StreamCipherSeek.html\" title=\"trait cipher::stream::StreamCipherSeek\">StreamCipherSeek</a>,&nbsp;</span>","synthetic":false,"types":["chacha20poly1305::ChaChaPoly1305"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
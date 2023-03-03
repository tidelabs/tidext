(function() {var implementors = {
"blake2":[["impl&lt;OutSize&gt; <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"blake2/struct.Blake2bMac.html\" title=\"struct blake2::Blake2bMac\">Blake2bMac</a>&lt;OutSize&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;OutSize: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"typenum/type_operators/trait.IsLessOrEqual.html\" title=\"trait typenum::type_operators::IsLessOrEqual\">IsLessOrEqual</a>&lt;<a class=\"type\" href=\"typenum/generated/consts/type.U64.html\" title=\"type typenum::generated::consts::U64\">U64</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"typenum/operator_aliases/type.LeEq.html\" title=\"type typenum::operator_aliases::LeEq\">LeEq</a>&lt;OutSize, <a class=\"type\" href=\"typenum/generated/consts/type.U64.html\" title=\"type typenum::generated::consts::U64\">U64</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"],["impl&lt;OutSize&gt; <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"blake2/struct.Blake2sMac.html\" title=\"struct blake2::Blake2sMac\">Blake2sMac</a>&lt;OutSize&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;OutSize: <a class=\"trait\" href=\"generic_array/trait.ArrayLength.html\" title=\"trait generic_array::ArrayLength\">ArrayLength</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.67.1/std/primitive.u8.html\">u8</a>&gt; + <a class=\"trait\" href=\"typenum/type_operators/trait.IsLessOrEqual.html\" title=\"trait typenum::type_operators::IsLessOrEqual\">IsLessOrEqual</a>&lt;<a class=\"type\" href=\"typenum/generated/consts/type.U32.html\" title=\"type typenum::generated::consts::U32\">U32</a>&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"type\" href=\"typenum/operator_aliases/type.LeEq.html\" title=\"type typenum::operator_aliases::LeEq\">LeEq</a>&lt;OutSize, <a class=\"type\" href=\"typenum/generated/consts/type.U32.html\" title=\"type typenum::generated::consts::U32\">U32</a>&gt;: <a class=\"trait\" href=\"typenum/marker_traits/trait.NonZero.html\" title=\"trait typenum::marker_traits::NonZero\">NonZero</a>,</span>"]],
"crypto":[["impl <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"crypto/hashes/blake2b/struct.Blake2b256.html\" title=\"struct crypto::hashes::blake2b::Blake2b256\">Blake2b256</a>"],["impl <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"crypto/hashes/blake2b/struct.Blake2b160.html\" title=\"struct crypto::hashes::blake2b::Blake2b160\">Blake2b160</a>"]],
"digest":[],
"hmac":[["impl&lt;D:&nbsp;<a class=\"trait\" href=\"digest/digest/trait.Digest.html\" title=\"trait digest::digest::Digest\">Digest</a> + <a class=\"trait\" href=\"crypto_common/trait.BlockSizeUser.html\" title=\"trait crypto_common::BlockSizeUser\">BlockSizeUser</a>&gt; <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"hmac/struct.SimpleHmac.html\" title=\"struct hmac::SimpleHmac\">SimpleHmac</a>&lt;D&gt;"]],
"sha1":[["impl <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"sha1/struct.Sha1.html\" title=\"struct sha1::Sha1\">Sha1</a>"]],
"twox_hash":[["impl <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"twox_hash/struct.XxHash32.html\" title=\"struct twox_hash::XxHash32\">XxHash32</a>"],["impl <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"twox_hash/struct.XxHash64.html\" title=\"struct twox_hash::XxHash64\">XxHash64</a>"],["impl <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"twox_hash/xxh3/struct.Hash64.html\" title=\"struct twox_hash::xxh3::Hash64\">Hash64</a>"],["impl <a class=\"trait\" href=\"digest/trait.Update.html\" title=\"trait digest::Update\">Update</a> for <a class=\"struct\" href=\"twox_hash/xxh3/struct.Hash128.html\" title=\"struct twox_hash::xxh3::Hash128\">Hash128</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
(function() {var implementors = {};
implementors["bip39"] = [{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"bip39/struct.Seed.html\" title=\"struct bip39::Seed\">Seed</a>","synthetic":false,"types":["bip39::seed::Seed"]}];
implementors["curve25519_dalek"] = [{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"curve25519_dalek/scalar/struct.Scalar.html\" title=\"struct curve25519_dalek::scalar::Scalar\">Scalar</a>","synthetic":false,"types":["curve25519_dalek::scalar::Scalar"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"curve25519_dalek/montgomery/struct.MontgomeryPoint.html\" title=\"struct curve25519_dalek::montgomery::MontgomeryPoint\">MontgomeryPoint</a>","synthetic":false,"types":["curve25519_dalek::montgomery::MontgomeryPoint"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"curve25519_dalek/edwards/struct.CompressedEdwardsY.html\" title=\"struct curve25519_dalek::edwards::CompressedEdwardsY\">CompressedEdwardsY</a>","synthetic":false,"types":["curve25519_dalek::edwards::CompressedEdwardsY"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"curve25519_dalek/edwards/struct.EdwardsPoint.html\" title=\"struct curve25519_dalek::edwards::EdwardsPoint\">EdwardsPoint</a>","synthetic":false,"types":["curve25519_dalek::edwards::EdwardsPoint"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"curve25519_dalek/ristretto/struct.CompressedRistretto.html\" title=\"struct curve25519_dalek::ristretto::CompressedRistretto\">CompressedRistretto</a>","synthetic":false,"types":["curve25519_dalek::ristretto::CompressedRistretto"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"curve25519_dalek/ristretto/struct.RistrettoPoint.html\" title=\"struct curve25519_dalek::ristretto::RistrettoPoint\">RistrettoPoint</a>","synthetic":false,"types":["curve25519_dalek::ristretto::RistrettoPoint"]}];
implementors["ed25519_dalek"] = [{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"ed25519_dalek/struct.SecretKey.html\" title=\"struct ed25519_dalek::SecretKey\">SecretKey</a>","synthetic":false,"types":["ed25519_dalek::secret::SecretKey"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"ed25519_dalek/struct.ExpandedSecretKey.html\" title=\"struct ed25519_dalek::ExpandedSecretKey\">ExpandedSecretKey</a>","synthetic":false,"types":["ed25519_dalek::secret::ExpandedSecretKey"]}];
implementors["elliptic_curve"] = [{"text":"impl&lt;C&gt; <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"elliptic_curve/struct.NonZeroScalar.html\" title=\"struct elliptic_curve::NonZeroScalar\">NonZeroScalar</a>&lt;C&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"elliptic_curve/trait.Curve.html\" title=\"trait elliptic_curve::Curve\">Curve</a> + <a class=\"trait\" href=\"elliptic_curve/trait.ScalarArithmetic.html\" title=\"trait elliptic_curve::ScalarArithmetic\">ScalarArithmetic</a>,&nbsp;</span>","synthetic":false,"types":["elliptic_curve::scalar::nonzero::NonZeroScalar"]}];
implementors["merlin"] = [{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"merlin/struct.Transcript.html\" title=\"struct merlin::Transcript\">Transcript</a>","synthetic":false,"types":["merlin::transcript::Transcript"]}];
implementors["schnorrkel"] = [{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"schnorrkel/keys/struct.MiniSecretKey.html\" title=\"struct schnorrkel::keys::MiniSecretKey\">MiniSecretKey</a>","synthetic":false,"types":["schnorrkel::keys::MiniSecretKey"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"schnorrkel/keys/struct.SecretKey.html\" title=\"struct schnorrkel::keys::SecretKey\">SecretKey</a>","synthetic":false,"types":["schnorrkel::keys::SecretKey"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"schnorrkel/keys/struct.Keypair.html\" title=\"struct schnorrkel::keys::Keypair\">Keypair</a>","synthetic":false,"types":["schnorrkel::keys::Keypair"]}];
implementors["sec1"] = [{"text":"impl&lt;Size&gt; <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"sec1/point/struct.EncodedPoint.html\" title=\"struct sec1::point::EncodedPoint\">EncodedPoint</a>&lt;Size&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Size: <a class=\"trait\" href=\"sec1/point/trait.ModulusSize.html\" title=\"trait sec1::point::ModulusSize\">ModulusSize</a>,&nbsp;</span>","synthetic":false,"types":["sec1::point::EncodedPoint"]}];
implementors["x25519_dalek"] = [{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"x25519_dalek/struct.EphemeralSecret.html\" title=\"struct x25519_dalek::EphemeralSecret\">EphemeralSecret</a>","synthetic":false,"types":["x25519_dalek::x25519::EphemeralSecret"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"x25519_dalek/struct.StaticSecret.html\" title=\"struct x25519_dalek::StaticSecret\">StaticSecret</a>","synthetic":false,"types":["x25519_dalek::x25519::StaticSecret"]},{"text":"impl <a class=\"trait\" href=\"zeroize/trait.Zeroize.html\" title=\"trait zeroize::Zeroize\">Zeroize</a> for <a class=\"struct\" href=\"x25519_dalek/struct.SharedSecret.html\" title=\"struct x25519_dalek::SharedSecret\">SharedSecret</a>","synthetic":false,"types":["x25519_dalek::x25519::SharedSecret"]}];
implementors["zeroize"] = [];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
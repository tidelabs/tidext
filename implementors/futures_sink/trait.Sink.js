(function() {var implementors = {};
implementors["futures_channel"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for <a class=\"struct\" href=\"futures_channel/mpsc/struct.Sender.html\" title=\"struct futures_channel::mpsc::Sender\">Sender</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::Sender"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for <a class=\"struct\" href=\"futures_channel/mpsc/struct.UnboundedSender.html\" title=\"struct futures_channel::mpsc::UnboundedSender\">UnboundedSender</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::UnboundedSender"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for &amp;<a class=\"struct\" href=\"futures_channel/mpsc/struct.UnboundedSender.html\" title=\"struct futures_channel::mpsc::UnboundedSender\">UnboundedSender</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::UnboundedSender"]}];
implementors["futures_sink"] = [];
implementors["tokio_util"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static&gt; <a class=\"trait\" href=\"futures_sink/trait.Sink.html\" title=\"trait futures_sink::Sink\">Sink</a>&lt;T&gt; for <a class=\"struct\" href=\"tokio_util/sync/struct.PollSender.html\" title=\"struct tokio_util::sync::PollSender\">PollSender</a>&lt;T&gt;","synthetic":false,"types":["tokio_util::sync::mpsc::PollSender"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
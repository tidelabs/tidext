window.SIDEBAR_ITEMS = {"enum":[["Phase","A phase of a block’s execution."]],"struct":[["EventDetails","The event details."],["EventSubscription","A subscription to events that implements [`Stream`], and returns [`Events`] objects for each block."],["Events","A collection of events obtained from a block, bundled with the necessary information needed to decode and iterate over them."],["EventsClient","A client for working with events."],["FilterEvents","A stream which filters events based on the `Filter` param provided. If `Filter` is a 1-tuple of a single `Event` type, it will return every instance of that event as it’s found. If `filter` is `tuple of multiple`Event`types, it will return a corresponding tuple of`Option`s, where exactly one of these will be `Some(event)` each iteration."],["FilteredEventDetails","This is returned from the [`FilterEvents`] impl of [`Stream`]. It contains some type representing an event we’ve filtered on, along with couple of additional pieces of information about that event."]],"trait":[["EventFilter","This trait is implemented for tuples of Event types; any such tuple (up to size 8) can be used to filter an event subscription to return only the specified events."],["StaticEvent","Trait to uniquely identify the events’s identity from the runtime metadata."]]};
# Connection rework

Right now, wire inputs are pushed to by a node, and then the outputs pull from the input, and then the next node pulls from the output.

With Connections, the node can push to the wire input through the connection, and then the node pulls from the wire output through a connection as well.

This makes it easy to iterate through all the connections at once.

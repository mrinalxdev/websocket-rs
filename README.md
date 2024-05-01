# websocket-rs

Websocket initials in Rust language

### main.rs Documentation for learning

There is the websocket service stored in the url variable.

- To connect to the service asynchronusly we are using (connect_async) method.

> Connecting to the server .

- There will be lot of errors regarding the async functions used,
  - First mark the main function under tokio `#[tokio::main]` by doing like this above the main function.
  - `connect_async` is method under tokio-tungstenite, so import from that.

> Sending and recieving messages

- To send and recieve messages from the service agent, we need to split up the websocket stream into a write and read stream.
- `.split()` method is used for spliting the stream into multiple stream. Let's take two varaibles known (write)[For writing] and (read)[for reading]

- For writing messages to the stream
  - (write) ``write.send(msg);`` is used, this is method used from ``futures_util::sink::SinkExt`` crate in rust.
  - ``let msg = Message::Text()`` is used for declaring the message
  - At last its an async function so do add ``write.send(msg).await`` at the end.

- For reading messages to the stream
 - You can first check if there is any message by putting it in a if stattement, --> ``read.next()`` to pull out the messages
 - If there is any than print the messages.

 # Your Websocket is ready !! 🥳🥳🥳

# A question about how websockets work

Open two terminal windows.

On one, start the websocket server:

    cargo run --bin server
    
On the other, start the client:

    cargo run --bin client

The client sends a single message to the server, then sleeps.
The server's output is:

    Client connected: 1
    Got msg from 1: "A"

As soon as the client exits (or even if you `kill -9` it), the server reports:

    Error NoDataAvailable, closing connection
    
## The question

How does the server immediately know that the client has quit?

My understanding of websockets is that the protocol includes ping/pong message types, but as far as I can tell, the [websocket crate](https://github.com/cyderize/rust-websocket) doesn't do any automatic heartbeating.
(The ping/pong types exist within the `OwnedMessage` enumeration, but I'm not using them in this example.)

A [Stack Overflow answer](https://stackoverflow.com/questions/10585355/sending-websocket-ping-pong-frame-from-browser) suggests that browsers implement ping/pong for you, but one might want their own heartbeat mechanism at the application level.

Is the behavior I'm seeing --- the server "knowing" immediately when the client has hung up --- a consequence of both client and server being on the same computer?
Is the OS X kernel doing us a favor and sending some kind of "other end hung up" message that wouldn't be sent on an actual remote connection?

Or is this something coming out of the underlying TCP layer that *would* eventually happen on a remote connection?

(And application-level heartbeating simply a kind of [end-to-end](https://en.wikipedia.org/wiki/End-to-end_principle) precaution?)

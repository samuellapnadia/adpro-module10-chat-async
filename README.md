# MODULE 10 BROADCAST ADVPROG - SAMUELLA PUTRI NADIA PAUNTU (2306170446)

## Original code, and how it run
![](img/ss_one.png)
To run this, first start the server using cargo run --bin server, then open three other terminal tabs or windows and run cargo run --bin client in each one to launch the clients. Each client connects to the server via WebSocket on 127.0.0.1:2000. Once connected, every message typed in any client is sent to the server, which then broadcasts it to all clients. The message shown in each client includes the sender's socket address (e.g. 127.0.0.1:49921) followed by the message content. As demonstrated in the screenshot, typing "ella", "loves", and "adpro" from different clients resulted in the messages being received by all other clients and also logged by the server. This confirms that the tokio::select! logic correctly handles both incoming and outgoing streams concurrently, enabling real-time broadcast communication between multiple clients.


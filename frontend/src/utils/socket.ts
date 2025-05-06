type MessageHandler = (data: any) => void;

class ChatSocket {
  private socket: WebSocket | null = null;
  private handlers: MessageHandler[] = [];
  private isConnected: boolean = false;
  private pendingMessages: string[] = [];

  connect(roomId: string, token: string) {
    const encodedToken = encodeURIComponent(token);
    const wsUrl = `ws://localhost:4000/ws/${roomId}?token=${encodedToken}`;
    this.socket = new WebSocket(wsUrl);

    this.socket.onopen = () => {
      console.log("WebSocket connected");
      this.isConnected = true;

      // Flush any pending messages
      this.pendingMessages.forEach((msg) => this.socket!.send(msg));
      this.pendingMessages = [];
    };

    this.socket.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data);
        this.handlers.forEach((h) => h(message));
      } catch (e) {
        console.error("Invalid WebSocket message:", event.data);
      }
    };

    this.socket.onerror = (err) => {
      console.error("WebSocket error:", err);
    };

    this.socket.onclose = () => {
      console.log("WebSocket closed");
      this.isConnected = false;
    };
  }

  onMessage(handler: MessageHandler) {
    this.handlers.push(handler);
  }

  send(data: any) {
    const msg = JSON.stringify(data);
    if (this.isConnected && this.socket?.readyState === WebSocket.OPEN) {
      this.socket.send(msg);
    } else {
      console.warn("WebSocket is not open. Queuing message.");
      this.pendingMessages.push(msg);
    }
  }

  close() {
    this.socket?.close();
    this.isConnected = false;
    this.pendingMessages = [];
  }
}

export const chatSocket = new ChatSocket();

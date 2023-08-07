let msg = Node.IO.msg();

let lower = msg.payload.toLowerCase();
msg.payload = lower;

Node.IO.send(msg);


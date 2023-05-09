let msg = Node.IO.msg();
console.log(msg.payload);

let lower = msg.payload.toLowerCase();
msg.payload = lower;

Node.IO.send(msg);


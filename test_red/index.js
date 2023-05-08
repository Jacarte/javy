function main(){
  let msg = Node.IO.msg();
  console.log(msg.payload);

  let lower = msg1.payload.toLowerCase();

  Node.IO.send(lower);
  Node.IO.done(lower);
}

main()

const engine = {
  working: true,
  hello: {
    world: "How are you?",
    ouch: {
      another: "Nested obj!",
    },
  },
}

const mustang = {
  name: "Mustang",
  engine: { ...engine, hello: { ...engine.hello } },
}

const camaro = {
  name: "Camaro",
  engine: { ...engine, hello: { ...engine.hello } },
}

function checkEngine(car) {
  if (car.name === "Mustang") {
    car.engine.working = false
    car.engine.hello.world = "Howdy?"
  }
}

// !BUG: When we change mustang, it also automatically changes camaro owing to object reference in JavaScript
checkEngine(mustang)

console.log("__Nasty-Bug__", JSON.stringify({ camaro, mustang }, null, 4))

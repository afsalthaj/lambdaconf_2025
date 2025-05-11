### WASM TIME

```sh
cd add
cargo component build --release

cd ../calculator
cargo component build --release


cd ../myrunner
cargo component build --release
```

### Compose components

```sh
## from root
wac plug calculator/target/wasm32-wasip1/release/caclulator.wasm  --plug  add/target/wasm32-wasip1/release/add.wasm  -o calculator_add.wasm
wac plug myrunner/target/wasm32-wasip1/release/myrunner.wasm  --plug calculator_add.wasm  -o myrunner_composed.wasm
```

### Use forked wasmtime to invoke

```sh
/Users/afsalthaj/projects/ribbb/wasmtime/target/debug/wasmtime --invoke 'eval-expression("abc")' myrunner_composed.wasm
```
![image](https://github.com/user-attachments/assets/0579a554-e571-444f-a7fe-e7b67387cf7d)


WASMTIME dependency: https://github.com/afsalthaj/wasmtime

```sh
./wasmtime repl --component-file myrunner_composed.wasm
```


Presentation:



![right fit](/Users/afsalthaj/projects/lambdaconf_2025/rib_image_unplugged.png)

## Rib - way of interacting with web assembly

---


## Why WASM ?

* Fast, portable, and secure by design

* Runs in a lightweight, sandboxed environment

* Enables polyglot programming and language interoperability

* Supported by major platforms and browsers

* Evolving towards a component model for modular, introspectable systems

---

## And one thing that I love - Introspection!

My appreciation for introspection began with **functional programming**

Seeing that same introspection system-wide through **WebAssembly** is deeply satisfying.


---


🔍 _**Introspection**_ — The host sees what your program actually does
🛡 _**Limits**_ unexpected or malicious behavior.  The host decides the implementation.
🧩 _**Designed, not patched**_ — A step beyond reactive security fixes


---

## 101

A WA __**component**__ is a self-contained unit of code that can be independently compiled, shared, and executed.
 

---
## 101

**WIT** (WebAssembly Interface Types): A specification used to define how WebAssembly components communicate with each other. 

---
## 101

_**Wasmtime**_: High-performance WebAssembly runtime. A real world example of how the whole component model and WIT are intended to work in practice

---


## Interact with WASM components


Let's invoke functions in a WASM component, but without using the new Rib


We will use `wasmtime` cli to do this


As of now dev release required to invoke functions in a component



---

## Steps to follow

* We will build a **simple component** and it's called `command` component so that we can use wasmtime CLI to interact
* Build a **command** component - to which `wasmtime` cli will interact with!
* Import the arithmetic package in command component and re-export them
* use **wasmtime cli** to just interact with it by invoking functions

---

### Demo: Wasmtime cli direct component function invoke

---

## Suboptimal interaction

* Gets vary for complex types
* Limited: How about passing the result of one component to the other?
* Basic requirement - invoking a function, and looks far different from what developers already know

---

## Solution

Wasmtime Repl backed by the typesafe scripting language Rib

---

## Let's see this in action

---

## Rib REPL in wasmtime 

![inline 85%](/Users/afsalthaj/projects/lambdaconf_2025/rib_repl_initial.png)

---

## Rib REPL in wasmtime - compiler errors

![inline 90%](/Users/afsalthaj/projects/lambdaconf_2025/rib_repl_initial.png)

---


## What we achieved?

![left filtered 50%](/Users/afsalthaj/projects/ribbb/lambdaconf_2025/rib_image.jpeg)

* Quick _**typesafe**_ interaction with WASM, helps with complex types
* _**Method call syntax**_ to invoke functions
* _**Autocomplete**_ all the way including function arguments
* Type inspection
* Descriptive _**compilation errors**_

---

## Onboard quickly

It's easy to use, and syntax is intuitive 

* Simply start `wasmtime repl`, create an instance and rely on auto completes
* *And you know Rib if you know basic Rust syntax to a great extent*
* Sticks on to _**wasm wave syntax**_

---

## Rib Repl - Consistent syntax

```rust
>>> let x = instance();
>>> let cart = x.cart(user-id);
cart.add-item({product-id: 1, product-name: apple, quantity: 1, price: 1000})
```

---
## Rib Repl - Distinguish components with type parameters

If you have multiple components in the context, we can distinguish them with component 
root package as a type parameter


```rust

>>> let x = instance[foo:bar]()
>>> let y = instanec[bar:baz]()
>>> x.foo()
>>> y.bar()

```
---

---
## Rib Repl - Distinguish functions within a component

If you have multiple components in the context, we can distinguish them with component 
root package as a type parameter


```rust

>>> let x = instance()
>>> y.bar[baz]()

```

---
## Is Rib REPL tied to wasmtime?

No! It's an embeddable REPL that you can integrate with any system that knows how load components
and how to invoke functions

![right fit](/Users/afsalthaj/projects/lambdaconf_2025/repl_embedded.png)

---

## How about stateful applications?

* We used wasmtime repl and interacted with components that are not stateful

* We can somehow make it work though by having persistence layer within your app

* Or an ever running project serving http APIs


---

### But I would rather simply switch to 

_**Golem OSS**_

---
### Why?

* I can write the application with an in-memory map
* Can't get into the boring DB and persistence layer during my development
* Can't deal with deploying http server or any methods to run the wasm forever


---

### Let's see how quick it is to switch

---

### golem CLI and Rib REPL


---

### Golem CLI

Golem CLI is integrated with the same Rib Repl

```rust
golem app build
golem repl

>>> let worker = instance("my_cart")
>>> cart.add-item({product-id: "foo", name: "foo", price: 42, quantity: 42})
>>> cart.get-cart-contents()

[{product-id: "foo", name: "foo", price: 42, quantity: 42}]


```

---
## Rib Syntax

Mostly covered in https://learn.golem.cloud/rib, 
covering `list comprehension`, `list reduction`,`if conditions`, `pattern match` and so on and so forth!


Internally desugars to an `if-else`

Pattern match works for almost all wasm types such as literals, record, result, option, variants, enums etc


---
## Examples

```rust
let result = foo("bar);

match result {
   ok(result) => "found ${result.user-id}",
   err(msg)   =>   msg
  
}

```


```rust
let worker-name = 
  if result.user-id > 1000 then "group0" else "group1"

```

---

## Examples (cont)




---
## Configuration management in Rib

A typical rib script has the concept of global input.


```rust

let token: string = token;

```


Here rib compiler infers _**token**_ is a global input.

Rib infers the type of configuration to be of type   _**string**_ 


---

### Configuration management in Rib



```rust

let token: string = env.token;

```


Here rib compiler infers _**env**_ is a global input.

Rib infers the type of configuration to be wasm type _**{env: string}**_

---

## Can anything be a global input?

No. When compiling rib using `rib::compile`, 

you have to provide valid global variable names 

Otherwise compiler error `unknown variable`

And then 

```rust
rib::interpreter::run(compiled_rib_code, {env: "foo"})

```

---

## Typesafe configuration setup and easier integration

This way you can ask the compiler to emit the `input requirements` or `configuration requirements`.

This static analysis is powerful. We will see how!

---

## Rib in API gateway of golem

---

## Golem API gateway

* golem provides API gateway to quickly expose your component functions as http APIs

* Create an API definition which is a set of routes and each route has an embedded rib

* This rib is responsible for invoking component functions with optional transformation of input and output


---
## Example Rib script in API definition

```rust
let user-id = request.path.user-id;

let worker-name = "user-${user-id}";

let worker = instance(worker-name);

let input = request.body;

worker.add-user(user)

```

---

API gateway compiles Rib with valid global variables to be `["request"]`.

Hence it knows statically the http request body, the path variables in the request route,
And therefore easily generate an open API spec.

And any http calls that don't satisfy these requirements will become a BadRequest

---
## Rib Implementation overview

* Parser implemented using combine library in Rust: 
* Find the grammer in https://learn.golem.cloud/rib
* Rib compiler consist of independent 15+ compilation phases
* Examples: `type_pull_up`,  `type_push_down_phase`, `type_check` and `unification`
* It scans script repeatedly for some of these phases, until it reaches a fix-point of inference defined by the sate of `InferredType` which is a superset of wasm types.

---

## Rib Implementation overview (contd)

* Rib Intermediate Representation (IR): 
   **https://github.com/golemcloud/golem/blob/main/golem-rib/src/compiler/ir.rs**
* Stack based interpreter
* Configurable compiler and interpreter to customize the behaviour
   * Example customizations are golem API gateway, Rib REPL

---

## Type inference FixPoint

```rust
pub fn type_inference_fix_point<F, E>(
  mut scan_and_infer: F, 
  expr: &mut Expr
) -> Result<(), E>
where
    F: FnMut(&mut Expr) -> Result<(), E> {
  ...
}
```
---
## Rib IR



---

### Immedediate things to improve

* Precise and better compilation errors, at the unification phase
* std functions and user defined functions
* Possibly

---



## A note on precise compilation errors

* Compilation errors are mainly handled at type checker phase
* The ones that are not captured at type checker phase, will be captured at the type unification phase.
* type unification phase currently doesn't lookup the origin of types, but this is added in a WIP PR


---



---
## Quick summary of Rib interactions

* API gateway in golem - embedded rib scripts are significantly used in golem-timeline
* golem REPL and wasmtime REPL (forked)

---

## How Rib is used in golem-timeline ?

* Get the current state of computation from every components involved in a streaming pipeline, and do various dynamic operations. 
* We cannot afford writing a component for each of those numerous dynamic requirements
* We need a reliable, light-weight yet type safe way of interacting with it

---


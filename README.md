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


## Presentation:


![right fit](/Users/afsalthaj/projects/lambdaconf_2025/rib_image_unplugged.png)

## Rib - way of interacting with web assembly

---


## Why WebAssembly ?

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

* We will build a **simple component** and it's called `command` component
* We will walk through a simple **WIT** file that's part of the component
* A quick skim through the component code written in rust
* Try running the component using **wasmtime** and try invoking functions

---

## Let' see this in action

---

## Suboptimal interaction

* Gets difficult with complex types
* Limited interaction capability. Example: How about passing the result of one component to the other?
* Prone to making errors, less productivity


---
## Solution

###A [**REPL**](https://github.com/afsalthaj/wasmtime) within wasmtime backed by the type-safe **Rib** language

---

## Let's see this in action

---

## Rib REPL in wasmtime 

![inline 85%](/Users/afsalthaj/projects/lambdaconf_2025/rib_repl_initial.png)

---
## Rib REPL in wasmtime - parser errors


![inline](/Users/afsalthaj/projects/lambdaconf_2025/repl_parser_error.png)


---


## Rib REPL in wasmtime - compiler errors

![inline](/Users/afsalthaj/projects/lambdaconf_2025/repl_compilation_error.png)


---


## The errors without REPL

![inline](/Users/afsalthaj/projects/lambdaconf_2025/wasmtime_compilation_error.png)


---
## What we achieved?
![right filtered 50%](/Users/afsalthaj/projects/ribbb/lambdaconf_2025/rib_image.jpeg)

* Quick _**typesafe**_ interactions with WASM components
* _**Method call syntax**_ to invoke functions
* _**Autocomplete**_ all the way
* **Inspect types** anytime
* Distinguish components, functions etc properly
* Descriptive _**compilation errors**_

---

## Onboard quickly

It's easy to use, and syntax is intuitive 

* Simply start `wasmtime repl`, create an instance and rely on auto completes
* You know Rib if you know basic Rust syntax for most of it
* Sticks on to _**wasm wave syntax**_ for wasm values

---

## Consistent Method Call Syntax



```rust
// x is a component instance, cart is a resource instance
>>> let x = instance();
>>> let cart = x.cart(user-id);
>>> cart.add-item({product-id: 1, product-name: apple, quantity: 1, price: 1000})
```

---
## Distinguish components with type parameters


```rust

>>> let x = instance[foo:bar]()
>>> let y = instance[bar:baz]()
>>> x.foo()
>>> y.bar()

```

---
## Rib REPL - Distinguish functions within a component


_**baz**_ and _**qux**_ are interfaces having function _**bar**_

```rust

>>> let x = instance()
>>> x.bar[baz]()
>>> x.bar[qux]()

```


---

## Embeddable 

Is **Rib REPL** tied to `wasmtime`?

No! `Rib REPL` embeddable - that you integrate with any system/runtime clients.

We will see an example of it outside wasmtime


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
* I can't get into the boring DB and persistence layer
* I don't need to re-engineer what golem has done in running a component forever


---

## Let's see how quick it is!

---
## Bootstrap an app with golem

```sh
golem server run
golem app new shopping-cart rust
cd shopping-cart
golem component new rust my:shopping-cart
golem repl

```
---


## Let's see this in action

---

### Golem CLI

Golem CLI is integrated with the same Rib REPL crate

```rust

>>> let worker = instance("my_cart")
>>> cart.add-item({product-id: "foo", name: "foo", price: 42, quantity: 42})
>>> cart.get-cart-contents()

[{product-id: "foo", name: "foo", price: 42, quantity: 42}]


```

---

## More on Rib Syntax

---
## Rib Syntax and Grammer

![right fit](/Users/afsalthaj/projects/lambdaconf_2025/rib_doc.png)

https://learn.golem.cloud/rib covers most of it

 * grammer
 * types
 * if-else
 * pattern match
 * list comprehension/reduction
 * ranges and many more


---
## Examples

![inline](/Users/afsalthaj/projects/lambdaconf_2025/pattern_match_exhaustive.png)

---

## Examples (cont)


```rust
let worker = 
  if result.user-id > 1000 then "group0" else "group1"

```

---

## Examples (cont)


```rust
let limits = 1..10;

for i in limits {
  cart.add-items(i);
  yield "success";
} 


```

---


## Configuration management in Rib

Rib has the concept of global input.


```rust

let token: string = token;

```


Here rib compiler infers _**token**_ as a global input. 

In other words, it expects a configuration `token` as input

Rib infers the type of configuration to be of type   _**string**_ 


---

### Configuration Management in Rib



```rust

let token: string = env.token;

```


Here rib compiler infers _**env**_ as a global input.

Rib infers the type of configuration to be wasm record of type _**{ env: string }**_

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

### Generating OpenAPI  based on Rib script

* API gateway compiles Rib with valid global variables to be `["request"]`.
* Hence it knows statically the http request body, the path variables in the request route,
* Easily generate an open API spec.
* And any http calls that don't satisfy these requirements will become a BadRequest

---
## Rib Implementation overview

* Parser implemented using combine library in Rust: 
* Find the grammer in https://learn.golem.cloud/rib
* Rib compiler consist of independent 15+ compilation phases
* Examples: `type_pull_up`,  `type_push_down_phase`, `type_check` and `unification`
* It scans script repeatedly for some of these phases, until it reaches a fix-point of inference defined by the sate of `InferredType` which is a superset of wasm types.

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

## Rib Implementation overview (contd)

* Rib Intermediate Representation (IR): 
   **https://github.com/golemcloud/golem/blob/main/golem-rib/src/compiler/ir.rs**
* Stack based interpreter
* Configurable compiler and interpreter to customize the behaviour
   * Example customizations are golem API gateway, Rib REPL


---

## Immediate things to fix and improve


* Precise and better compilation errors, at the unification phase
* std functions and user defined functions
And resolve known issues [here](https://github.com/golemcloud/golem/issues?q=is%3Aissue%20state%3Aopen%20author%3Aafsalthaj%20label%3Arib)

---


## A note on precise compilation errors

* Compilation errors are mainly handled at type checker phase
* The ones that are not captured at type checker phase, will be captured at the type unification phase.
* type unification phase currently doesn't lookup the origin of types, but this is added in a WIP PR


---

## How Rib is used in golem-timeline ?

* Get the current state of computation from every components involved in a streaming pipeline, and do various dynamic operations. 
* We cannot afford writing a component for each of those numerous dynamic requirements
* Easily create an observability tool for timeline

---

## Find more details at

https://learn.golem.cloud/rib

https://github.com/golemcloud/golem/tree/main/golem-rib

https://github.com/golemcloud/golem/tree/main/golem-rib-repl

---


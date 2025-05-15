---
theme: seriph
class: 'text-center'
highlighter: shikiji
lineNumbers: false
fonts:
  sans: Roboto
  serif: Roboto Slab
  mono: Fira Code
layout: cover
transition: slide-left
---

# Typescript, schema, Effect and Rust

"Things can always be wrong, let your system to prevent it"

---
layout: center
---

## The Story: Basic Error Handling in JS

How many errors can be found in the following code?

```ts
async function fetchDetail(id: string) {
  const res = await fetch(`/items/${id}`)
  return await res.json()
}
```

---
layout: center
---

## The Story: Improved Error Handling

```ts
function fetchDetail(id: string) {
  invariant(id.length > 0, 'id can not be empty string')
  invairant(id.match(/^\\d+$/), 'id must contains only number')
  let res: Response
  try {
    res = fetch(`/items/${id}`)
  } catch (error) {
    throw new Error('Network Error', { cause: error })
  }
  if (res.status !== 200) {
    throw new Error('Status code is not success')
  }
  try {
    const json = await res.json()
    // we finally check all of the possible errors
    return json
  } catch (error) {
    throw new Error('json parse error', { cause: error })
  }
}
```

- Increased complexity
- Still no prevention of misusing the return value, the return value is `any`
- Errors are only found at runtime

---
layout: center
---

## TypeScript to the rescue (partial)

```ts
interface ItemDetail {
  data_field: string
  correct_field_name: string
}

function fetchDetail(id: `${number}`): Promise<ItemDetail> {
  // we don't need these two invariant anymore, as typescript will check this
  // invariant(id.length > 0, 'id can not be empty string')
  // invairant(id.match(/^\\d+$/), 'id must contains only number')
  // ...
}
```

TypeScript helps with input types, but how do we ensure the server returns the correct data shape?

---
layout: center
---

## The schema

Using libraries like Zod for runtime validation.

```ts
import { z } from 'zod'

const ItemDetailSchema = z.object({
  data_field: z.string(),
  correct_field_name: z.string(),
})
type ItemDetail = z.infer<typeof ItemDetailSchema>
```

---
layout: center
---

## The schema (cont.)

Integrating schema validation into the function:

```ts
function fetchDetail(id: `${number}`): Promise<ItemDetail> {

  try {
    const json = await res.json()
    // we want to make sure the external data source works as expected
    const parsed = ItemDetailSchema.safeParse(json) // Use ItemDetailSchema here
    if (!parsed.success) {
      throw parsed.error
    }
    return parsed.data
  } catch (error) {
    throw new Error('json parse error', { cause: error })
  }
}
```

Schema validation ensures data conforms to the expected shape at runtime.

---
layout: center
---

## Effect, the checked error

Introducing the `Effect` library for explicit error handling in TypeScript.

The `Effect` type: `Effect<A, E, R>`
- A: Result type
- E: Error type
- R: Environment (ignored for now)

---
layout: center
---

## Effect, the checked error (cont.)

Wrapping the function with `Effect`:

```ts
const fetchDetail = Effect.fn('fetchDetail')(function * (id: `${number}`) {
  const res = yield * Effect.tryPromise({
    try: () => fetch(`/items/${id}`),
    catch: (error) => error as Error,
  });
  const json = yield * Effect.tryPromise({
    try: () => res.json(),
    catch: (error) => error as Error,
  });
  const parsed = yield * Effect.try({
    try: () => ItemDetailSchema.parse(json), // Use ItemDetailSchema here
    catch: (error) => error as Error,
  })
  return parsed
})
```

The return type `Effect<ItemDetail, Error>` clearly indicates potential errors.

---
layout: center
---

## How Rust handles this

Rust uses the `Result<T, E>` enum for fallible operations.

<<< @/snippets/rust-example.rs

---
layout: center
---

## How Rust handles this (cont.)

`Result<T, E>` has two variants:
- `Result::Ok(T)`: Success with value T
- `Result::Err(E)`: Error with error type E

This forces users to handle potential errors. `panic!` is used for unrecoverable errors.

---
layout: center
---

<<< @/snippets/rust-example.rs {12,15}

---
layout: center
---

## Conclusion

Beyond the "happy path", consider the possibilities of errors.

For TypeScript, recommendations:
- Utilize `invariant` for preconditions.
- Use schema validation libraries for data type validation.
- Prefer libraries that offer higher type-safety.

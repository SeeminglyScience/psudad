# `psudad`

`psudad` aka "PS Utility Decode and Deflate" is a small console app that decodes base64 encoded gzip
strings and writes the content to stdout.

The primary (and likely exclusive) use case for this is sending large pre-built strings to render in
`fzf`'s preview pane without hitting the `cmd` character limit for lines. If the preview you want to
generate would come from PowerShell, and `fzf` is being invoked from PowerShell, this lets you avoid
the high start up cost of re-entry.

## Install

```powershell
scoop install https://gist.github.com/SeeminglyScience/8dc717be6e6d362ad65efbdf124922b8/raw/psudad.json
```

## Usage

```powershell
psudad H4sIAAAAAAAACitJLS7JzEsHAAZa8+gHAAAA
```

outputs:

```raw
testing
```

---

```powershell
psudad 'H4sIAAAAAAAACu1dW2/bNhR+H9Bf0Jc+bkBQxK4Xp/MWIHOCxliaGFXegj7QEudwoUhNpNwZQ//7KMtO5dnDRMqSSeoEiQwHOCJ1Lt+58FB89d3rx/4wVpeBugzOR/3R4LT4Vd/f5d9774ebv/jx9WP+//+hGKrLaW/UG57FgUyzUN6iJc9kJdryaN/rjlaM8xthkfZYbytR9PvqWvzFl5nk2sOc5BRvtMaZovC5GlV5oF8qkgyGintno97ZeXyq/TQ/aFN8zileVVS6dwP12Y+TbEZJqMGCNV1IkRDVyEo6NOVfcBo8YUr1mf6T9miTKyISLtCMYh3OrMf7u6B5o34a42iZNcGYxzEqrKvCiP2BUuDTUf/HQbwmrCiOnWfUe7w51seakf4wouFh1iRfCyG3KeYrJJFCboxioS9qPUK7JX0sEXzIiIGRTZiQiIV4EnnB/A0LU7JAcgWPYJ+FfU7YgodIEs4CqVgzYb9zE20xvQmojTOYUqabcU4N1ETcYSExQIp3Lr++btyg6DpNeeqHswd/sz8Y+ZQxkaAQ6+uHJqUT6uG7lKdGSGBADdJ2D/AnYiPo+y8Mpz5JmjCJU4ZoxxFh68FkStjcICwgQvJ0GeiQO6Ej3koaLzCT2i7jOqe6UWU9WkCB1kP+rFvc30l7x0+IzXG0msVlOhfaU7jQL8bvn0IrOo6iqA0dT3HMF9hPNVflMWlSENVcEigrzFgVQ4viVQNLX/XWYlYiA/7X4P+e8PkjjwwSpbSUKIHULbe68vIdI5IgGmAhNi5BX/hk/01AD9zRA/MSiXeWf1C/WZSet+/TUT96IL5eRlFpEb8pXtZMIMNYpRT6qRaIzlnRnbRVwcoEvlUZHA1CngDo7oJDEKYkkRYrmDCboP/Y4KvkABo6ETeU5rkeyKytJdwmBpDYkuEUpSjGaqXDYpxINnO8Ux9twAWf/YFDqT/RBaIZxBBdVLIOyVC0UEW7VQuGNUSoP0WQ30Hld0XCfEkMpUuQ4kGtUC1rZvF6abZhIAUneOB0LK/jNy07d+BszCnFK5Roo1Pg/iWia7ohYMwZMxnKSyFPLsWShZ+wyKg0ZuXqHsBPG/hZaE4l7dnZP+aTwZejTp7JxGAv84k5P1cCHCNKZ9V3Am8tL29alsr3aHTyh4gr8l4dLxMznwzj361xz13udAIZHy6jvGYq70k32+J1Ac8EoEGr/ESO42pVxWr0f7U5YylVddNgE74oUYIZmDvQB4v1v62HAXfeZsgGGuehG4DgAnS8eVSFUMcLoyzTLXjVFwF1O6BwTfVflrytFFArFVEwP2fMz6xufSxDNJwtWGQN7kHVwnVXPmnJ8z3cm7mLC7+XMA35f+EAsh6BmXV0zPblYIDaZttQfsVzwmAx0wZ22hsRW45/9voRW7M+0Hc/wih7Nd9dl3+83rUQOtYAECGNtT7ecCF5td1zWZ+zQuhmgc1Awa1LmOVM2G09eEH0DdG3mz7tAYnnNnaKuLVpqwmY2V13g72IoIbOqiFUz72rIYK7AnfVIZxwJv2xFzEg74G8BxyJTWDhb94DNVA3ukNcqHza7vGtL3hCqgAIDwjf+czEBax3Js2zHvQh24Nsz81YwLU8TC+rus4PJGjpJUfa3fpl3EHbxF5EnPU34QWSJw1Jrnu98Q0y05bGY3hD5jGsVIFsO7oFEFuhXUUfYZtstgBk6DIyXBGRcAHvpl0ZaCBxkuQv5ZiSBFPCsD4/P2C59y4+xEdrykddWLnLKM35MeZM4r8aOxBgMFQjno16Z+dxrwUGfv5Wn2mWfROWH+QkCkywc4ofsXzi0SROmjqGeO9Y90meDutv2ntbiWJ76U/ilCGaR802qlbNNtplYoZ0G0IfwK2u9yWpzBDVJ9Q5pa3M/gcevFAC/835X/2s5K2E6k81WFPnvx3ijAhFB3ZWZiQpjpvSxrgbJJ7GPLIa57RovuY0/wD+PM6W3aIAAA=='
```

outputs:

![output-example](https://user-images.githubusercontent.com/24977523/190881929-030a226b-9be4-42cd-a401-992057282d3f.png)

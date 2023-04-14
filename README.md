# Fender example plugin

> ![warn](https://raw.githubusercontent.com/Mqxx/GitHub-Markdown/main/blockquotes/badge/dark-theme/warning.svg)
>
> Fender is still in alpha so any element is subject to, and likely to, breaking changes

## Fender Plugins

plugins are a way to add rust interop to fender

they work as though you were adding to the standard library of fender itself but

## Structure

> ![info](https://raw.githubusercontent.com/Mqxx/GitHub-Markdown/main/blockquotes/badge/dark-theme/info.svg)
>
> **Much of the following structure will not be written by hand in the future but will be generated from attribute macros**

each rust library can only expose 1 plugin currently

to expose a plugin use the `declare_plugin` macro from `fender`, this will take in a `plugin` and a constructor.

the constructor is just a function that takes no args and return an instance of your `plugin`. This function could follow the rust convention of adding a `new` function in an impl block on the struct, implementing default on the struct, or just a free function that fits the pointer type

The `plugin` itself is a struct that implements the `Plugin` trait from the `fender` crate.

The fields of the struct should only consist of `FenderPluginFunction` and `FenderValue`.

to implement the `Plugin` trait you need 3 functions `name`, `get_functions`, and `get_values`

`name` doesn't do much of anything at the moment but it will be a way to refer to the plugin from inside of fender

`get_functions` and `get_values` each return a `HashMap` of references to either the `FenderPluginFunction` or `FenderValue` held by the `plugin` respectively.
The key used in the hashmap is the name used to access it in fender so should follow the camel case convention that fender uses.


## Cargo toml

until the tooling for fender Plugins is refined both `fender` and `freight_vm` will need to
be added as dependencies. It is recommended to use the version of Freight used by the version of Fender that is being depended on

for `fender`, `default-features` can be set to `false`, as the fender repl is likely not needed for plugins

for `freight_vm` the `variadic_functions` feature should be enabled

under the `[lib]` section crate type should be set to `["dylib"]`

use fender::freight_vm;

use fender::{
    declare_plugin,
    fender_value::FenderValue,
    fndr_native_func,
    plugin::{FenderPluginFunction, Plugin},
    type_sys::freight_type_system::FenderTypeSystem,
};
use fender::freight_vm::{expression::NativeFunction, function::ArgCount};
use std::collections::HashMap;

#[derive(Debug)]
struct ExamplePlugin {
    example_1: FenderPluginFunction,
    example_2: FenderPluginFunction,
    name_list: FenderValue,
}

declare_plugin!(ExamplePlugin, ExamplePlugin::new);

impl ExamplePlugin {
    pub fn new() -> ExamplePlugin {
        ExamplePlugin {
            // `FenderPluginFunction` fields must be set to a tuple of
            // `freight_vm::expression::NativeFunction` and `freight_vm::function::ArgCount`
            //
            // the easiest way to create the function held by `NativeFunction` is to use
            // the `fender::fndr_native_func` macro
            example_1: (
                NativeFunction::new(example_1_func),
                ArgCount::Variadic { min: 1, max: 1 },
            ),
            example_2: (NativeFunction::new(other_name_for_func), ArgCount::Fixed(0)),
            name_list: FenderValue::make_list(vec![
                FenderValue::make_string("FuzzyNovaGoblin".into()).into(),
                FenderValue::make_string("Redempt".into()).into(),
                FenderValue::make_string("GigaRyno".into()).into(),
            ]),
        }
    }
}

impl Plugin for ExamplePlugin {
    fn name(&self) -> &'static str {
        "defaultPlugin"
    }

    fn get_functions(&self) -> HashMap<&str, &(NativeFunction<FenderTypeSystem>, ArgCount)> {
        let mut ret = HashMap::new();
        // The following lines tie the rust function to what it will be called in fender,
        // in this case `example1` and `example2` are the functions you will call in fender
        // ```fndr
        // @plugin path/to/libexample_plugin.so
        // example1("hello",2,3,4,5)
        // ```
        ret.insert("example1", &self.example_1);
        ret.insert("example2", &self.example_2);
        ret
    }

    fn get_values(&self) -> std::collections::HashMap<&str, &FenderValue> {
        let mut ret = HashMap::new();
        // Same as in `get_functions` the nameList key is what will be used in
        ret.insert("nameList", &self.name_list);
        ret
    }
}

#[doc = " example1"]
#[allow(unused)]
pub fn example_1_func(_: &mut freight_vm::execution_engine::ExecutionEngine<fender::type_sys::freight_type_system::FenderTypeSystem>,args:freight_vm::execution_engine::Stack<fender::type_sys::fender_reference::FenderReference>,) -> Result<fender::type_sys::fender_reference::FenderReference,freight_vm::error::FreightError>{
  const _ARG_COUNT:usize = fender::count!(item,argv);
  let[item,argv]:[fender::type_sys::fender_reference::FenderReference;
  _ARG_COUNT] = args.try_into().unwrap();
  {
    println!("item: {}",item.fender_dbg_string());
    println!("argv: {}",argv.fender_dbg_string());
    use fender::fender_value::FenderValue::*;
    Ok(FenderValue::make_list(vec![item,Int(argv.len().unwrap()as i64).into()]).into())
  }
}

fndr_native_func!(
    /// example2
    #[no_mangle]
    other_name_for_func,
    |_| {
        use fender::fender_value::FenderValue::*;
        Ok(FenderValue::make_string(
            "this is function example 2 function, or in rust `other_name_for_func`".into(),
        )
        .into())
    }
);

#[doc = " example2"]
#[allow(unused)]
pub fn thing(
    _: &mut fender::freight_vm::execution_engine::ExecutionEngine<
        fender::type_sys::freight_type_system::FenderTypeSystem,
    >,
    args: Vec<fender::type_sys::fender_reference::FenderReference>,
) -> Result<fender::type_sys::fender_reference::FenderReference, fender::freight_vm::error::FreightError> {
    const _ARG_COUNT: usize = fender::count!();
    {
        Ok(FenderValue::make_string("test".into()).into())
    }
}

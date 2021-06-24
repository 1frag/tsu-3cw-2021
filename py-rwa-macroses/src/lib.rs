extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{LitStr, parse_macro_input, Data, DeriveInput};
use syn::__private::TokenStream2;
use std::process::{Command, Stdio};
use std::io::Write;
use std::str::from_utf8;

/// # Generate conversation for Enum to PyObject:
/// ## Example:
/// ```
/// #[derive(EnumToPyObject)]
/// pub enum StatusFlight {
///     Departed,
///     Arrived,
///     OnTime,
///     Cancelled,
///     Delayed,
///     Scheduled,
/// }
/// ```
/// Will generate:
/// ```
/// impl ToPyObject for StatusFlight
/// {
///     fn to_object(&self, py: Python) -> PyObject
///     {
///         match self
///         {
///             StatusFlight::Departed => "Departed",
///             StatusFlight::Arrived => "Arrived",
///             StatusFlight::OnTime => "OnTime",
///             StatusFlight::Cancelled => "Cancelled",
///             StatusFlight::Delayed => "Delayed",
///             StatusFlight::Scheduled => "Scheduled",
///             _ => unreachable!()
///         }.to_string().to_object(py)
///     }
/// }
///
/// impl IntoPy<PyObject> for StatusFlight {
///     fn into_py(self, py: Python) -> PyObject { self.to_object(py) }
/// }
///
/// impl Adaptable for StatusFlight
/// {
///     fn adaptable(row: Option<&Row>, idx: usize, py: Python) -> Self
///     {
///         let d: String = row.unwrap().get(idx);
///         match d.as_str()
///         {
///             "Departed" => StatusFlight::Departed,
///             "Arrived" => StatusFlight::Arrived,
///             "OnTime" => StatusFlight::OnTime,
///             "Cancelled" => StatusFlight::Cancelled,
///             "Delayed" => StatusFlight::Delayed,
///             "Scheduled" => StatusFlight::Scheduled,
///             _ => unreachable!(),
///         }
///     }
/// }
/// ```
///
#[proc_macro_derive(EnumToPyObject)]
pub fn enum_to_py_object(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields: Vec<String> = match input.data {
        Data::Enum(t) => {
            t.variants.iter().map(|v| {
                v.ident.to_string().clone()
            }).collect::<Vec<String>>()
        }
        _ => panic!("Что-то не так")
    };

    let t1 = fields
        .iter()
        .map(|s| format!("{}::{} => \"{}\",", struct_name, s, s))
        .collect::<Vec<String>>()
        .join("\n")
        .parse::<TokenStream2>()
        .unwrap();

    let t2 = fields
        .iter()
        .map(|s| format!("\"{}\" => {}::{},", s, struct_name, s))
        .collect::<Vec<String>>()
        .join("\n")
        .parse::<TokenStream2>()
        .unwrap();

    let gen = quote! {
        impl ToPyObject for #struct_name {
            fn to_object(&self, py: Python) -> PyObject {
                match self {
                    #t1
                    _ => unreachable!()
                }.to_string().to_object(py)
            }
        }

        impl IntoPy<PyObject> for #struct_name {
            fn into_py(self, py: Python) -> PyObject {
                self.to_object(py)
            }
        }

        impl Adaptable for #struct_name {
            fn adaptable(row: Option<&Row>, idx: usize, py: Python) -> Self {
                let d: String = row.unwrap().get(idx);
                match d.as_str() {
                    #t2
                    _ => unreachable!(),
                }
            }
        }
    };
    // println!("{}", gen.to_string()); // useful for debug
    TokenStream::from(gen)
}

#[proc_macro_derive(Iterable)]
pub fn derive_iterable(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields: Vec<String> = match input.data {
        Data::Struct(t) => {
            t.fields.iter().map(|v| {
                v.ident.as_ref().unwrap().to_string().clone()
            }).collect::<Vec<String>>()
        }
        _ => panic!("Что-то не так")
    };

    let t1 = fields
        .iter()
        .map(|s| format!("(\"{}\", slf.{}.clone().into_py(py)),", s, s))
        .collect::<Vec<String>>()
        .join("\n")
        .parse::<TokenStream2>()
        .unwrap();

    let gen = quote! {
        use pyo3::PyIterProtocol;

        #[pyproto]
        impl PyIterProtocol for #struct_name {
            fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PyAny>> {
                let gil = Python::acquire_gil();
                let py = gil.python();

                let fields = vec![
                    #t1
                ];
                Ok(fields.to_object(py).getattr(py, "__iter__")?.call0(py)?)
            }
        }
    };
    // println!("{}", gen.to_string()); // useful for debug
    TokenStream::from(gen)
}

#[derive(Debug)]
struct AddFunctions {
    py_module_name: Option<String>,
    modules: Vec<AddFunctionsModule>,
    _idx: i32,
}

#[derive(Debug)]
struct AddFunctionsModule {
    module_name: Option<String>,
    functions: Vec<String>,
}

impl AddFunctions {
    fn new() -> Self {
        AddFunctions { py_module_name: None, modules: vec![], _idx: 0 }
    }

    fn next(&mut self, token: &TokenTree) {
        match self._idx {
            0 => self.py_module_name = Some(token.to_string()),
            1 => assert_eq!(",".to_string(), token.to_string()),
            2 => {
                let functions = match token {
                    TokenTree::Group(group) => {
                        group
                            .stream()
                            .into_iter()
                            .filter_map(|s| {
                                match s.to_string().as_str() {
                                    "," => None,
                                    k => Some(k.to_string()),
                                }
                            }).collect()
                    }
                    _ => panic!("Что пошло не так"),
                };
                let module_info = AddFunctionsModule { module_name: None, functions };
                self.modules.push(module_info);
            }
            3 => assert_eq!("from".to_string(), token.to_string()),
            4 => {
                self.modules.last_mut().unwrap().module_name = Some(token.to_string());
            }
            _ => panic!("Что пошло не так"),
        };
        self._idx = self._idx % 4 + 1;
    }

    fn stream(self) -> TokenStream {
        let mut res: Vec<String> = vec![];
        for module in self.modules {
            for function in module.functions {
                res.push(format!(
                    "{pm}.add_function({m}::__pyo3_get_function_{f}(PyFunctionArguments::PyModule({pm}))?)?;",
                    pm = self.py_module_name.as_ref().unwrap(),
                    m = module.module_name.as_ref().unwrap(),
                    f = function,
                ))
            }
        }

        res.join("\n").parse().unwrap()
    }
}

/// # Добавить функции в модуль
/// ## Example:
/// ```
/// add_functions!(m,
///     [init, configure] from utils,
///     [cities_by_timezone] from cities_by_timezone,
///     [get_bookings] from get_bookings,
///     [flight_by_min_duration] from flight_by_min_duration,
/// );
/// ```
#[proc_macro]
pub fn add_functions(input: TokenStream) -> TokenStream {
    let mut st = AddFunctions::new();

    for row in input.into_iter() {
        st.next(&row);
    }
    st.stream()
}

/// # Проверить синтаксис SQL запроса
/// ## Example
/// ```
/// sql!(r"
///     SELECT * FORM my_table;
/// ") //         ^^  - incorrect
/// ```
/// Ошибка произойдет на этапе компиляции
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    if cfg!(all(unix, not(debug_assertions))) {
        let input2 = input.clone();
        let output = parse_macro_input!(input2 as LitStr);
        let value = output.value();

        let mut proc = Command::new("sh")
            .arg("-c")
            .arg("pgsanity")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        let mut stdin = proc.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin.write_all(value.as_bytes()).expect("Failed to write to stdin");
        });

        let output = proc.wait_with_output().expect("Failed to read stdout");
        if output.status.code().unwrap() != 0 {
            println!("Linter output:\n{}\nwith code {}",
                     from_utf8(&output.stdout).unwrap(),
                     output.status.code().unwrap());
            panic!();
        }
    }
    input
}

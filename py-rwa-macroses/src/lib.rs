extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};
use syn::__private::TokenStream2;

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

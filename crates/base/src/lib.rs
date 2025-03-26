pub mod model;

pub mod macro_support {
    pub use bach::{
        self,
        ext::*,
        time::{sleep, Instant},
        *,
    };
    pub use core::time::Duration;
    pub use wasm_bindgen::{self, prelude::*};

    #[cfg(feature = "test")]
    pub use wasm_bindgen_test;
    #[cfg(feature = "test")]
    pub use wasm_bindgen_test::*;

    pub use super::model::queue::Ext;
}

#[macro_export]
macro_rules! def {
    (pub fn $name:ident($($arg:ident: $arg_t:ty),* $(,)?) $body:block) => {
        #[allow(non_snake_case)]
        pub mod $name {
            use $crate::macro_support::*;
            use super::*;

            #[allow(non_camel_case_types)]
            #[wasm_bindgen(inspectable)]
            #[derive(Default)]
            pub struct $name {
                $(
                    pub $arg: $arg_t,
                )*
            }

            #[wasm_bindgen]
            impl $name {
                #[wasm_bindgen(constructor)]
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn _run(&self) -> $crate::model::Sim {
                    $crate::run(|| {
                        let $name {
                            $(
                                $arg,
                            )*
                            ..
                        } = self;

                        $(
                            let $arg = $arg.clone();
                        )*

                        $body
                    })
                }
            }

            // #[cfg(all(test, target_arch = "wasm32"))]
            // #[wasm_bindgen_test]
            // fn run_test() {
            //     run(Params::default());
            // }
        }
    };
}

pub fn run<F: FnOnce()>(f: F) -> model::Sim {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let (scope, _final_time) = model::scope::with(Default::default(), || {
        let mut sim = bach::environment::default::Runtime::default();
        sim.run(f);
        sim.elapsed()
    });

    scope.finish()
}

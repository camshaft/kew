use std::time::Duration;

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
}

#[macro_export]
macro_rules! def {
    (pub fn $name:ident($($arg:ident: $arg_t:ty),* $(,)?) $body:block) => {
        #[allow(non_snake_case)]
        pub mod $name {
            use $crate::macro_support::*;

            #[allow(non_camel_case_types)]
            #[wasm_bindgen(inspectable)]
            #[derive(Clone, Default)]
            pub struct $name {
                $(
                    pub $arg: $arg_t,
                )*
                final_time: Option<Duration>,
                // TODO outputs
            }

            #[wasm_bindgen]
            impl $name {
                #[wasm_bindgen(constructor)]
                pub fn new() -> Self {
                    Self::default()
                }

                pub fn final_time(&self) -> f32 {
                    self.final_time.map_or(0.0, |v| v.as_secs_f32())
                }

                pub fn _run(&mut self) {
                    self.final_time = $crate::run(|| {
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
                    });
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

pub fn run<F: FnOnce()>(f: F) -> Option<Duration> {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut sim = bach::environment::default::Runtime::default();
    sim.run(f);
    Some(sim.elapsed())
}
